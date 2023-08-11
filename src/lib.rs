use std::io::Read;
use byteorder::ReadBytesExt;



pub trait ToBytes{
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait StructFromBytes<TStruct, TReader: Read>{
    fn struct_from_bytes(reader: &mut TReader) -> TStruct;
}

// struct Hi{
//     n: u8
// }
//
// impl<TReader: Read> StructFromBytes<Hi, TReader> for TReader{
//     fn struct_from_bytes(reader: &mut TReader) -> Hi {
//         Hi{
//             n: reader.read_u8().unwrap()
//
//         }
//     }
// }