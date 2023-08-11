use std::io::Read;

pub trait ToBytes {
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error>;
}

pub trait StructFromBytes<TStruct, TReader: Read, E> {
    fn struct_from_bytes(reader: &mut TReader) -> Result<TStruct, E>;
}
