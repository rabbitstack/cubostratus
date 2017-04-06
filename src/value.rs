/// Container for primitive stack allocated `(i32, u32, bool, etc)` as well as heap
/// allocated `(String)` data types. This enum is used by `SyscallParam::parse` method
/// to store the content of the system call parameter's payload.
/// Some syscall's parameters can have multiple attributes, for example, the socket tuple
/// is comprised of IP address and port number. In that case, the `Value` enum can also have
/// structures associated with it.
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    None
}
