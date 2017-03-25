pub mod syscall_table;

use libc;
use std::ffi::CStr;
use std::{ptr, slice};
use std::mem::size_of;
use std::collections::HashMap;
use chrono::{DateTime, UTC};
use value::Value;

#[repr(C, packed)]
pub struct SyscallHdr {
    /// timestamp in nanoseconds from epoch
    pub ts: u64,
    /// the thread id that generated the syscall
    pub tid: u64,
    /// length of the syscall + header
    pub len: u32,
    /// the identifier of the syscall
    pub id: u16
}

pub struct SyscallMeta {
    /// name of the system call
    pub name: &'static str,
    /// syscall category
    pub category: Category,
    /// flags for this syscall
    pub flags: Vec<Flags>,
    /// the number of parameters
    pub nparams: usize,
    /// the list of syscall's parameters
    pub params: Vec<SyscallParam>
}

pub struct SyscallParam {
    /// parameter name
    pub name: &'static str,
    /// parameter type
    pub kind: ParamType,
    /// parameter rendering format
    pub fmt: ParamFormat
}

#[derive(Serialize, Debug)]
pub struct Syscall {
    /// timestamp expressed as UTC date/time structure
    pub ts: DateTime<UTC>,
    /// name of the system call
    pub name: String,
    /// syscall's parameter map
    pub params: HashMap<String, Value>

}

/// determines the syscall category
pub enum Category {
    Unknown,
    Other,
    File,
    Net,
    IPC,
    Memory,
    Process,
    Sleep,
    System,
    Signal,
    User,
    Time,
    Processing,
    IOBase,
    IORead,
    IOWrite,
    IOOther,
    Wait,
    Scheduler,
    Internal

}

pub enum Flags {
    None,
    CreatesFd,
    DestroysFd,
    UsesFd,
    ReadsFromFd,
    WritesToFd,
    ModifiesState,
    Unused,
    Waits,
    SkipParser,
    OldVesion

}

pub enum ParamType {
    None,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    /// a NULL terminated printable buffer of bytes
    CharBuffer,
    ByteBuffer,
    ErrNo,
    SockAddr,
    SockTuple,
    Fd,
    Pid,
    FdList,
    FsPath,
    SyscallId,
    SigType,
    /// a relative time
    RelTime,
    /// an absolute time interval
    AbsTime,
    Port,
    L4Proto,
    SockFamily,
    Bool,
    Ipv4Addr,
    Dyn,
    Flags8,
    Flags16,
    Flags32,
    Uid,
    Gid,
    Double,
    Sigset,
    CharBufferArray,
    CharBufferPairArray,
    Ipv4Net
}

pub enum ParamFormat {
    Na,
    Dec,
    Hex,
    PaddedDec,
    Id,
    Dir
}

pub enum Direction {
    Enter,
    Exit
}

impl SyscallMeta {
    /// Populates the syscall parameter map by applying pointer arithmetic
    /// operations on the system call header structure. The parameter's buffer is extracted
    /// from the slice which is constructed from syscall header structure pointer
    /// and the number of parameters.
    /// For each parameter found in the slice, the parsing stage is delegated to
    /// the `SyscallParam::parse` method. The return value of the previous method is being put into
    /// the hash map and indexed by parameter name.
    ///
    pub fn build_params<'a>(&self, syscall_hdr: *mut SyscallHdr) -> HashMap<String, Value> {
        let mut params = HashMap::<String, Value>::default();
        unsafe {
            let lens = syscall_hdr.offset(1) as *const u16;
            let mut buf = lens.offset(self.nparams as isize) as *const u8;
            let mut i = 0;

            for len in slice::from_raw_parts(lens, self.nparams) {
                let ref param = self.params[i];
                params.insert(param.name.to_string(), param.parse(buf));
                buf = buf.offset(*len as isize);
                i += 1;
            }
        }
        params
    }
}

impl SyscallParam {
    /// Transforms the raw buffer which contains the parameter value to a native
    /// data type suitable for serialization.
    pub fn parse(&self, buf: *const u8) -> Value {
        match self.kind {
            ParamType::Int8 => {
                unsafe { Value::Int8(*(buf as *const i8)) }
            },
            ParamType::Int16 => {
                unsafe { Value::Int16(*(buf as *const i16)) }
            },
            ParamType::Int32 => {
                unsafe { Value::Int32(*(buf as *const i32)) }
            },
            ParamType::Int64 => {
                unsafe { Value::Int64(*(buf as *const i64)) }
            },
            ParamType::UInt8 => {
                unsafe { Value::UInt8(*(buf as *const u8)) }
            },
            ParamType::UInt16 => {
                unsafe { Value::UInt16(*(buf as *const u16)) }
            },
            ParamType::UInt32 => {
                unsafe { Value::UInt32(*(buf as *const u32)) }
            },
            ParamType::UInt64 => {
                unsafe { Value::UInt64(*(buf as *const u64)) }
            },
            ParamType::FsPath => {
                Value::String(self.to_string(buf))
            },
            ParamType::ErrNo => {
                unsafe { Value::Int64(*(buf as *const i64)) }
            },
            ParamType::Fd => {
                unsafe { Value::Int64(*(buf as *const i64)) }
            },
            ParamType::Pid => {
                unsafe { Value::Int64(*(buf as *const i64)) }
            },
            ParamType::Uid | ParamType::Gid  => {
                unsafe { Value::UInt32(*(buf as *const u32)) }
            },
            ParamType::SyscallId => {
                unsafe { Value::UInt16(*(buf as *const u16)) }
            },
            ParamType::CharBuffer => {
                Value::String(self.to_string(buf))
            },
            ParamType::ByteBuffer => {
                Value::String(self.to_string(buf))
            }
            _ => Value::None
        }
    }

    fn to_string(&self, buf: *const u8) -> String {
        unsafe {
            CStr::from_ptr(buf as *const libc::c_char).to_string_lossy().into_owned()
        }
    }
}