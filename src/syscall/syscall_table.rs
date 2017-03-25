use super::{SyscallMeta, SyscallParam, Category, Flags, ParamType, ParamFormat};

pub enum Syscalls {
    GenericEnter = 0,
    GenericExit = 1,
    OpenEnter = 2,
    OpenExit = 3,
    CloseEnter = 4,
    CloseExit = 5,
    ReadEnter = 6,
    ReadExit = 7,
    WriteEnter = 8,
    WriteExit = 9,
    Brk1Enter = 10,
    Brk1Exit = 11,
    Execve8Enter = 12,
    Execve8Exit = 13,
    Clone11Enter = 14,
    Clone11Exit = 15,
    ProcExitEnter = 16,
    ProcExitExit = 17,
    SocketEnter = 18,
    SocketExit = 19,
    BindEnter = 20,
    BindExit = 21,
    ConnectEnter = 22,
    ConnectExit = 23,
    ListenEnter = 24,
    ListenExit = 25,
    AcceptEnter = 26,
    AcceptExit = 27,
    SendEnter = 28,
    SendExit = 29,
    SendToEnter = 30,
    SendToExit = 31,
    RecvEnter = 32,
    RecvExit = 33,
    RecvFromEnter = 34,
    RecvFromExit = 35,
    ShutdownEnter = 36,
    ShutdownExit = 37,
    GetSockNameEnter = 38,
    GetSockNameExit = 39,
    GetPeerNameEnter = 40,
    GetPeerNameExit = 41,
    SocketPairEnter = 42,
    SocketPairExit = 43,
    SetSockOptEnter = 44,
    SetSockOptExit = 45,
    GetSockOptEnter = 46,
    GetSockOptExit = 47,
    SendMsgEnter = 48,
    SendMsgExit = 49,
    SendMMsgEnter = 50,
    SendMMsgExit = 51,
    RecvMsgEnter = 52,
    RecvMsgExit = 53,
    RecvMMsgEnter = 54,
    RecvMMsgExit = 55,
    Accept4Enter = 56,
    Accept4Exit = 57,
    CreatEnter = 58,
    CreatExit = 59,
    PipeEnter = 60,
    PipeExit = 61,
    EventFdEnter = 62,
    EventFdExit = 63,
    FutexEnter = 64,
    FutexExit = 65,
    StatEnter = 66,
    StatExit = 67,
    LstatEnter = 68,
    LstatExit = 69,
    FstatEnter = 70,
    FstatExit = 71,
    Stat64Enter = 72,
    Stat64Exit = 73,
    Lstat64Enter = 74,
    Lstat64Exit = 75,
    Fstat64Enter = 76,
    Fstat64Exit = 77,
    EpollWaitEnter = 78,
    EpollWaitExit = 79,
    PollEnter = 80,
    PollExit = 81,
    SelectEnter = 82,
    SelectExit = 83,
    NewSelectEnter = 84,
    NewSelectExit = 85,

}

pub struct SyscallTable {
    syscall_metas: Vec<SyscallMeta>
}

impl Default for SyscallTable {

    fn default() -> SyscallTable {
        SyscallTable {
            syscall_metas: vec![
                SyscallMeta{ name: "syscall", category: Category::Other, flags: vec![Flags::None], nparams: 2, params: vec![SyscallParam { name: "id", kind: ParamType::SyscallId, fmt: ParamFormat::Dec }, SyscallParam { name: "native_id", kind: ParamType::UInt16, fmt: ParamFormat::Dec } ] },
                SyscallMeta{ name: "syscall", category: Category::Other, flags: vec![Flags::None], nparams: 1, params: vec![SyscallParam { name: "id", kind: ParamType::SyscallId, fmt: ParamFormat::Dec }]},
                SyscallMeta{ name: "open", category: Category::File, flags: vec![Flags::CreatesFd, Flags::ModifiesState], nparams: 0, params: vec![]},
                SyscallMeta{ name: "open", category: Category::File, flags: vec![Flags::CreatesFd, Flags::ModifiesState], nparams: 4, params: vec![SyscallParam { name: "fd", kind: ParamType::Fd, fmt: ParamFormat::Dec}, SyscallParam {name: "name", kind: ParamType::FsPath, fmt: ParamFormat::Na}, SyscallParam { name: "flags", kind: ParamType::Flags32, fmt: ParamFormat::Hex}, SyscallParam {name: "mode", kind: ParamType::UInt32, fmt: ParamFormat::Hex} ] },
                SyscallMeta{ name: "close", category: Category::IOOther, flags: vec![Flags::DestroysFd, Flags::UsesFd, Flags::ModifiesState], nparams: 1, params: vec![SyscallParam { name: "fd", kind: ParamType::Fd, fmt: ParamFormat::Dec }]},
                SyscallMeta{ name: "close", category: Category::IOOther, flags: vec![Flags::DestroysFd, Flags::UsesFd, Flags::ModifiesState], nparams: 1, params: vec![SyscallParam { name: "res", kind: ParamType::ErrNo, fmt: ParamFormat::Dec }]},
                SyscallMeta{ name: "read", category: Category::IORead, flags: vec![Flags::UsesFd, Flags::ReadsFromFd], nparams: 2, params: vec![SyscallParam{ name: "fd", kind: ParamType::Fd, fmt: ParamFormat::Dec }, SyscallParam{ name: "size", kind: ParamType::UInt32, fmt: ParamFormat::Dec }]},
                SyscallMeta{ name: "read", category: Category::IORead, flags: vec![Flags::UsesFd, Flags::ReadsFromFd], nparams: 2, params: vec![SyscallParam{ name: "res", kind: ParamType::ErrNo, fmt: ParamFormat::Dec }, SyscallParam{ name: "data", kind: ParamType::ByteBuffer, fmt: ParamFormat::Na }]},
                SyscallMeta{ name: "write", category: Category::IOWrite, flags: vec![Flags::UsesFd, Flags::WritesToFd], nparams: 2, params: vec![SyscallParam{ name: "fd", kind: ParamType::Fd, fmt: ParamFormat::Dec }, SyscallParam{ name: "size", kind: ParamType::UInt32, fmt: ParamFormat::Dec }]},
                SyscallMeta{ name: "write", category: Category::IOWrite, flags: vec![Flags::UsesFd, Flags::WritesToFd], nparams: 2, params: vec![SyscallParam{ name: "res", kind: ParamType::ErrNo, fmt: ParamFormat::Dec }, SyscallParam{ name: "data", kind: ParamType::ByteBuffer, fmt: ParamFormat::Na }]},
                SyscallMeta{ name: "brk", category: Category::Memory, flags: vec![Flags::OldVesion], nparams: 1, params: vec![SyscallParam { name: "size", kind: ParamType::UInt32, fmt: ParamFormat::Dec }]},
                SyscallMeta{ name: "brk", category: Category::Memory, flags: vec![Flags::OldVesion], nparams: 1, params: vec![SyscallParam { name: "res", kind: ParamType::UInt64, fmt: ParamFormat::Hex }]},
                SyscallMeta{ name: "execve", category: Category::Process, flags: vec![Flags::ModifiesState], nparams: 0, params: vec![]},
                SyscallMeta{ name: "execve", category: Category::Process, flags: vec![Flags::ModifiesState, Flags::OldVesion], nparams: 8, params: vec![SyscallParam { name: "res", kind: ParamType::ErrNo, fmt: ParamFormat::Dec }, SyscallParam { name: "exe", kind: ParamType::CharBuffer, fmt: ParamFormat::Na }, SyscallParam { name: "args", kind: ParamType::ByteBuffer, fmt: ParamFormat::Na }, SyscallParam { name: "tid", kind: ParamType::Pid, fmt: ParamFormat::Dec }, SyscallParam { name: "pid", kind: ParamType::Pid, fmt: ParamFormat::Dec }, SyscallParam { name: "ptid", kind: ParamType::Pid, fmt: ParamFormat::Dec }, SyscallParam { name: "cwd", kind: ParamType::ByteBuffer, fmt: ParamFormat::Na }, SyscallParam { name: "fdlimit", kind: ParamType::UInt64, fmt: ParamFormat::Dec }]},
                SyscallMeta{ name: "clone", category: Category::Process, flags: vec![Flags::ModifiesState], nparams: 0, params: vec![]},
            ]

        }
    }
}

impl SyscallTable {
    pub fn get_syscall_meta(&self, id: usize) -> Option<&SyscallMeta> {
        self.syscall_metas.get(id)
    }
}
