use std::io::Read;

// A trait with a Derive macro for easy conversion of an struct to bytes
pub trait ToBytes {
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error>;
}

// A trait with a Derive macro for easy conversion of raw bytes to a struct
pub trait StructFromBytes<TStruct, TReader: Read, E> {
    fn struct_from_bytes(reader: &mut TReader) -> Result<TStruct, E>;
}
