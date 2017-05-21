//! Parses the `status` files from the `/proc` pseudo file system and collects information about
//! currently running processes.

use super::parsers::{parse_line, consume_until_line_ending, parse_u32_octal, parse_u64, parse_u32};
use super::cgroups::{CGroup, cgroups};
use nom::{IResult, line_ending, space};
use std::collections::HashMap;
use std::io::{Error, Result, ErrorKind, Read};
use std::fs::File;
use glob::glob;
use std::path::Path;

#[derive(Debug, Serialize)]
pub enum ThreadState {
    Running,
    Sleeping,
    Waiting,
    Stopped,
    TraceStopped,
    Dead,
    Zombie
}

pub struct ThreadRegistry {
    pub threads: HashMap<u64, ThreadInfo>,
    proc_root: String
}

#[derive(Serialize, Debug)]
pub struct ThreadInfo {
    /// filename of the executable
    pub comm: String,
    /// current state of the process
    pub state: ThreadState,
    /// process id (i.e., thread group id)
    pub pid: u64,
    /// thread id
    pub tid: u64,
    /// process id of the parent process
    pub ppid: u64,
    /// real user id
    pub uid: u32,
    /// real group id
    pub gid: u32,
    /// cgroups bounded to this thread
    pub cgroups: Option<Vec<CGroup>>
}

named!(parse_thread_state<ThreadState>,
       alt!(tag!("R (running)") => { |_| ThreadState::Running  }
          | tag!("S (sleeping)") => { |_| ThreadState::Sleeping }
          | tag!("D (disk sleep)") => { |_| ThreadState::Waiting }
          | tag!("T (stopped)") => { |_| ThreadState::Stopped }
          | tag!("t (tracing stop)") => { |_| ThreadState::TraceStopped }
          | tag!("X (dead)") => { |_| ThreadState::Dead }
          | tag!("Z (zombie)") => { |_| ThreadState::Zombie }));


named!(parse_command<String>, delimited!(tag!("Name:\t"), parse_line, line_ending));
named!(parse_umask<u32>, delimited!(tag!("Umask:\t"), parse_u32_octal, line_ending));
named!(parse_state<ThreadState>, delimited!(tag!("State:\t"), parse_thread_state, line_ending));
named!(parse_pid<u64>, delimited!(tag!("Tgid:\t"), parse_u64, line_ending));
named!(parse_tid<u64>, delimited!(tag!("Pid:\t"), parse_u64, line_ending));
named!(parse_ppid<u64>, delimited!(tag!("PPid:\t"), parse_u64, line_ending));

named!(parse_uid<u32>, chain!(tag!("Uid:\t") ~ uid: parse_u32 ~ consume_until_line_ending,
                              || {(uid)}));
named!(parse_gid<u32>, chain!(tag!("Gid:\t") ~ gid: parse_u32 ~ consume_until_line_ending,
                              || {(gid)}));

named!(thread_info<ThreadInfo>,
       do_parse!(
            comm: parse_command >>
            opt!(parse_umask) >>
            state: parse_state >>
            pid: parse_pid >>
            consume_until_line_ending >>
            tid: parse_tid >>
            ppid: parse_ppid >>
            consume_until_line_ending >>
            uid: parse_uid >>
            gid: parse_gid >>
            (ThreadInfo {
                 comm: comm,
                 state: state,
                 pid: pid,
                 tid: tid,
                 ppid: ppid,
                 uid: uid,
                 gid: gid,
                 cgroups: None,
            })
       ));

fn parse_thread(buf: &[u8]) -> IResult<&[u8], ThreadInfo> {
    map!(buf, thread_info, |t| {t})
}

pub fn parse_thread_info(pid: u64, root: String) -> Result<ThreadInfo> {
    let mut buf = String::new();
    let mut f = try!(File::open(format!("{}/{}/status", root, pid)));
    f.read_to_string(&mut buf);
    match parse_thread(buf.as_bytes()) {
        IResult::Done(i, o) => {
            Ok(o)
        },
        IResult::Error(e) => {
            Err(Error::new(ErrorKind::InvalidInput, "unable to parse status file"))
        },
        _ => Err(Error::new(ErrorKind::InvalidInput, "unable to parse status file")),
    }
}

impl ThreadRegistry {

    pub fn new() -> ThreadRegistry {
        ThreadRegistry {
            threads: HashMap::new(),
            proc_root: "/proc".to_string()
        }
    }

    pub fn collect(&mut self) {
        for e in glob(&format!("{}/*[0-9]*", self.proc_root.clone()))
            .expect("error") {
            match e {
                Ok(path) => {
                    let pid = path.file_name().unwrap()
                                .to_str()
                                .unwrap().parse::<u64>().unwrap();
                    let mut ti = parse_thread_info(pid, self.proc_root.clone()).unwrap();
                    ti.cgroups = Some(cgroups(pid, self.proc_root.clone()).unwrap());
                    self.threads.insert(pid, ti);
                },
                Err(e) => {

                }
            }
        }
    }
}
