///! Encapsulates the implementation of the syscall collectors.

use libc;
use num_cpus;
use chrono::{NaiveDateTime, DateTime, UTC, TimeZone};
use nix::Error as NixError;
use nix::errno;
use nix::fcntl::{open, O_RDWR, O_SYNC};
use nix::unistd::close;
use nix::sys::stat::S_IRUSR;
use nix::sys::mman::{mmap, munmap};
use nix::sys::mman::{MAP_SHARED, PROT_READ, PROT_WRITE};
use std::ptr;
use std::mem::size_of;

use syscall::{Syscall, SyscallInfo};
use syscall::syscall_table::SyscallTable;
use error::{Error, Result};

const RING_BUF_SIZE: usize = 8 * 1024 * 1024;
const BUFFER_EMPTY_WAIT_TIME_MS: u32 = 30;
const MAX_N_CONSECUTIVE_WAITS: usize = 4;

const PPM_IOCTL_MAGIC: u8 = 's' as u8;
const PPM_IOCTL_DISABLE_CAPTURE: u8 = 1;
const PPM_IOCTL_ENABLE_CAPTURE: u8 = 1;


ioctl!(none ioctl_start with PPM_IOCTL_MAGIC, PPM_IOCTL_ENABLE_CAPTURE);
ioctl!(none ioctl_stop with PPM_IOCTL_MAGIC, PPM_IOCTL_DISABLE_CAPTURE);


#[repr(C)]
struct RingBufferInfo {
    head: u32,
    tail: u32,
    num_syscalls: u64,
    num_drops_buffer: u64,
    num_drops_pf: u64,
    num_preemptions: u64,
    num_context_switches: u64
}

struct RingBufferDev {
    fd: i32,
    buffer: *mut libc::c_char,
    buffer_info: *mut RingBufferInfo,
    last_readsize: u32,
    next_syscall: *mut libc::c_char,
    len: u32
}

pub trait Collector {
    fn start(&mut self) -> Result<usize>;

    fn stop(&mut self) -> Result<()>;

    fn next(&mut self) -> Option<SyscallInfo>;
}

pub struct RingBufferCollector {
    devs: Vec<RingBufferDev>,
    consecutive_waits: usize,
    syscall_table: SyscallTable
}

/// The implementation of the syscall collector based on the
/// Sysdig's rock solid kernel driver. The driver uses circular / ring
/// buffer data structure to publish the intercepted system calls from the
/// kernel space. This aims to provide extremely high throughput and efficiency.
impl Collector for RingBufferCollector {
    /// Opens the available ring buffer devices and maps the corresponding
    /// buffer content and the buffer information to a memory region
    /// of the calling thread.
    ///
    /// If the previous operations are done successfully, it sends the
    /// IO code to the underlying device to start the capture from the
    /// kernel driver.
    ///
    /// Returns `Result::Ok(i)` where `i` is the number of devices (CPUs) detected on the
    /// system or `Result::Err(e)` where `e` is the payload with a string containing the detailed
    /// error message.
    ///
    /// # Example
    ///
    /// ```
    /// let collector = RingBufferCollector::new();
    /// match collector.start() {
    ///     Ok(num_devs) => {
    ///
    ///     },
    ///     Err(e) => {
    ///     }
    /// }
    /// ```
    ///
    fn start(&mut self) -> Result<usize> {
        let num_devs = num_cpus::get();
        let len = RING_BUF_SIZE * 2;

        // ~ open and map each of the char devices
        // which represent the ring buffers
        for i in 0..num_devs {
            match open(format!("/dev/sysdig{}", i).as_str(), O_RDWR | O_SYNC, S_IRUSR) {
                Ok(fd) => {
                    let buffer = mmap(ptr::null_mut(),
                                      len,
                                      PROT_READ,
                                      MAP_SHARED, fd, 0);
                    let buffer_info = mmap(ptr::null_mut(), size_of::<RingBufferInfo>(),
                                           PROT_READ | PROT_WRITE,
                                           MAP_SHARED, fd, 0);
                    if buffer.is_err() || buffer_info.is_err() {
                        munmap(buffer.unwrap() as *mut libc::c_void, len);
                        close(fd);
                        return Err(Error::RingBufferMapping);
                    }
                    let dev = RingBufferDev {
                        fd: fd,
                        buffer: buffer.unwrap() as *mut libc::c_char,
                        buffer_info: buffer_info.unwrap() as *mut RingBufferInfo,
                        last_readsize: 0,
                        next_syscall: ptr::null_mut(),
                        len: 0
                    };

                    self.devs.push(dev);
                    // ~ send the ioctl code to start the capture
                    unsafe { ioctl_start(fd); }

                },
                Err(e) => {
                    match e {
                        NixError::Sys(err) => {
                            if err == errno::ENODEV {
                                continue;
                            } else if err == errno::EBUSY {
                                return Err(Error::TooManyCollectors);
                            } else {
                                return Err(Error::DeviceError);
                            }
                        },
                        NixError::InvalidPath => {}
                    }
                }
            }
        }
        Ok(num_devs)
    }

    fn stop(&mut self) -> Result<()> {
        let len = RING_BUF_SIZE * 2;
        for dev in &self.devs {
            unsafe { ioctl_stop(dev.fd); }
            munmap(dev.buffer as *mut libc::c_void, len);
            munmap(dev.buffer_info as *mut libc::c_void, size_of::<RingBufferInfo>());
            close(dev.fd);
        }
        Ok(())
    }

    /// Consumes the next available syscall event from the ring buffer. If the buffers
    /// are empty, the head and tail are updated accordingly to pick up the
    /// new generated syscall events. When `consecutive_waits` is above the defined threshold, the
    /// main thread is suspended for `BUFFER_EMPTY_WAIT_TIME_MS` milliseconds. This can give a
    /// chance for buffers to refill.
    ///
    /// Returns `Some(syscall)` where `syscall` stores a plethora of information about the
    /// intercepted syscall event. On error it returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// let collector = RingBufferCollector::new();
    /// loop {
    ///     match collector.next() {
    ///         Some(syscall_info) => {
    ///             println!("syscall name {}", syscall_info.name);
    ///         },
    ///         None => {}
    ///     }
    /// }
    /// ```
    ///
    fn next(&mut self) -> Option<SyscallInfo> {
        let mut cpu = None;
        let mut syscall = ptr::null_mut();

        for (j, dev) in self.devs.iter().enumerate() {
            if dev.len == 0 {
                continue;
            } else {
                syscall = dev.next_syscall as *mut Syscall;
                cpu = Some(j);
            }
        }
        if cpu != None {
            let cpuid = cpu.unwrap();
            unsafe {
                assert!(self.devs[cpuid].len >= (*syscall).len);
                let len = (*syscall).len as isize;
                self.devs[cpuid].len -= (*syscall).len;
                self.devs[cpuid].next_syscall = self.devs[cpuid].next_syscall.offset(len);
            };
        } else {
            // ~ in case buffers are empty
            if self.check_next_wait() {
                unsafe { libc::usleep(BUFFER_EMPTY_WAIT_TIME_MS * 1000); }
                self.consecutive_waits += 1;
            }

            for mut dev in &mut self.devs {
                let buffer_info = dev.buffer_info;
                let ttail: usize = unsafe { ((*buffer_info).tail + dev.last_readsize) as usize };
                if ttail < RING_BUF_SIZE {
                    unsafe { (*dev.buffer_info).tail = ttail as u32 }
                } else {
                    unsafe { (*dev.buffer_info).tail = (ttail - RING_BUF_SIZE) as u32 }
                }

                let read_size = Self::get_buffer_readsize(buffer_info);
                dev.last_readsize = read_size;
                dev.len = read_size;

                unsafe { dev.next_syscall = dev.buffer.offset(ttail as isize); }
            }
        }

        if syscall.is_null() {
            None
        } else {
            let id = unsafe { (*syscall).id };
            let ts = unsafe { (*syscall).ts };

            match self.syscall_table.get_syscall_meta(id as usize) {
                Some(meta) => {
                    let timestamp = NaiveDateTime::from_timestamp((ts / 1000000000) as i64, 0);
                    let syscall_info = SyscallInfo {
                        ts: DateTime::<UTC>::from_utc(timestamp, UTC),
                        name: meta.name.to_string(),
                        params: meta.build_params(syscall),
                    };
                    return Some(syscall_info);

                },
                None => { return None; }
            }
        }
    }
}

impl RingBufferCollector {

    pub fn new() -> RingBufferCollector {
        RingBufferCollector {
            devs: Vec::<RingBufferDev>::new(),
            consecutive_waits: 0,
            syscall_table: SyscallTable::default()
        }
    }

    /// Determines if we should wait while the ring buffers
    /// are refilled with new system call events. Returns `true` if the
    /// condition is met, or `false` otherwise.
    fn check_next_wait(&mut self) -> bool {
        let mut res = true;
        for dev in &self.devs {
            let read_size = Self::get_buffer_readsize(dev.buffer_info);

            if read_size > 20000 {
                self.consecutive_waits = 0;
                res = false;
            }
        }
        if res == false {
            return false
        }

        if self.consecutive_waits >= MAX_N_CONSECUTIVE_WAITS {
            self.consecutive_waits = 0;
            return false;
        } else {
            return true;
        }
    }

    /// Calculates the new size of the buffer to be read.
    fn get_buffer_readsize(buffer_info: *mut RingBufferInfo) -> u32 {
        let head: usize = unsafe { (*buffer_info).head as usize };
        let tail: usize = unsafe { (*buffer_info).tail as usize };
        (if tail > head { RING_BUF_SIZE - tail + head } else { head - tail }) as u32
    }
}
