//! Parses the proc virtual file system for cgroup (control groups) information. The `/proc/[pid]/cgroup`
//! file describes control groups to which the process belongs. For each cgroup hierarchy there is
//! an entry of the form `hierarchy-id:subsystems:cgroup-path`, for example:
//!
//!     7:cpu,cpuacct:/user.slice
//!
//! where 7 is an unique hierarchy identifier, `cpu` and `cpuacct` are the cgroup subsystems bound
//! to the hierarchy, and finally, `/user.slice` is the cgroup pathname. There is a special `name=systemd`
//! cgroup with no bounded subsystem and is used by `systemd` to track services and user sessions.

use std::path::PathBuf;
use std::str::{self, FromStr};
use nom::{alpha, IResult};
use std::io::{Error, Result, ErrorKind, Read};
use std::fs::File;
use super::parsers::{consume_until_line_ending, parse_u8};

#[derive(Serialize, Debug)]
pub struct CGroup {
    /// the unique identifier of the cgroup hierarchy
    pub id: u8,
    /// vector of cgroup subsystems
    pub controllers: Vec<String>,
    /// pathname of the control group
    pub path: String
}

// parses cgroup subsystems
named!(controller, recognize!(chain!(
                                alpha ~
                                opt!(alt!(tag!("_") | tag!("="))) ~
                                opt!(alpha),
                                || {})));

named!(parse_controllers<Vec<String> >,
       many0!(do_parse!(c: map_res!(
                            map_res!(
                                controller,
                                str::from_utf8), FromStr::from_str) >>
                        opt!(tag!(",")) >>
                        opt!(tag!(":")) >> (c))));


// parses cgroup pathname
named!(parse_path<String>,
       map_res!(map_res!(consume_until_line_ending, str::from_utf8), FromStr::from_str));

// parses a single entry from the cgroup file
named!(cgroup<CGroup>, do_parse!(id: parse_u8 >>
                                 char!(':') >>
                                 controllers: parse_controllers >>
                                 path:  parse_path >>
                                 (CGroup { id: id,
                                          controllers: controllers,
                                          path: path })
                                 ));

fn parse_cgroups(buf: &[u8]) -> IResult<&[u8], Vec<CGroup>> {
    map!(buf, many0!(cgroup), |c| {c})
}

pub fn cgroups(pid: u64, root: String) -> Result<Vec<CGroup>> {
    let mut buf = String::new();
    let mut f = try!(File::open(format!("{}/{}/cgroup", root, pid)));
    f.read_to_string(&mut buf);
    match parse_cgroups(buf.as_bytes()) {
        IResult::Done(i, o) => {
            Ok(o)
        },
        IResult::Error(e) => {
            Err(Error::new(ErrorKind::InvalidInput, "unable to parse input"))
        },
        _ => Err(Error::new(ErrorKind::InvalidInput, "unable to parse input")),
    }
}