
/// Container for values. It's used as return type for functions that can
/// deal with heterogeneous data types.
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
