use crate::byte_utils::ReadOutBytesN;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::Write;

pub fn calculate_sample_length(wav_header: &Header) -> u16 {
    let bytes_per_sample = wav_header.bitsPerSample / 8;
    bytes_per_sample * wav_header.numChannels
}

pub fn contain_wav_phrase(wav_header: &Header) -> bool {
    let expected = "WAVE";
    for (byte, e) in wav_header.format.into_iter().zip(expected.chars()) {
        let c = byte as char;
        if c != e {
            return false;
        }
    }

    true
}

#[allow(unused, non_snake_case)] // The structs' fields intentionally exactly match the official WAV field names
#[derive(Debug)]
pub struct Header {
    chunkID: [u8; 4],
    chunkSize: u32,
    format: [u8; 4],
    subchunk1ID: [u8; 4],
    subchunk1Size: u32,
    audioFormat: u16,
    numChannels: u16,
    sampleRate: u32,
    byteRate: u32,
    blockAlign: u16,
    bitsPerSample: u16,
    subchunk2ID: [u8; 4],
    subchunk2Size: u32,
}

type WavEndian = byteorder::LittleEndian;

impl Header {
    pub fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<Header, std::io::Error> {
        let header = Header {
            chunkID: reader.read_out_bytes_array()?,
            chunkSize: reader.read_u32::<WavEndian>()?,
            format: reader.read_out_bytes_array()?,
            subchunk1ID: reader.read_out_bytes_array()?,
            subchunk1Size: reader.read_u32::<WavEndian>()?,
            audioFormat: reader.read_u16::<WavEndian>()?,
            numChannels: reader.read_u16::<WavEndian>()?,
            sampleRate: reader.read_u32::<WavEndian>()?,
            byteRate: reader.read_u32::<WavEndian>()?,
            blockAlign: reader.read_u16::<WavEndian>()?,
            bitsPerSample: reader.read_u16::<WavEndian>()?,
            subchunk2ID: reader.read_out_bytes_array()?,
            subchunk2Size: reader.read_u32::<WavEndian>()?,
        };

        Ok(header)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::with_capacity(44);
        bytes.write(&self.chunkID)?;
        bytes.write_u32::<WavEndian>(self.chunkSize)?;
        bytes.write(&self.format)?;
        bytes.write(&self.subchunk1ID)?;
        bytes.write_u32::<WavEndian>(self.subchunk1Size)?;
        bytes.write_u16::<WavEndian>(self.audioFormat)?;
        bytes.write_u16::<WavEndian>(self.numChannels)?;
        bytes.write_u32::<WavEndian>(self.sampleRate)?;
        bytes.write_u32::<WavEndian>(self.byteRate)?;
        bytes.write_u16::<WavEndian>(self.blockAlign)?;
        bytes.write_u16::<WavEndian>(self.bitsPerSample)?;
        bytes.write(&self.subchunk2ID)?;
        bytes.write_u32::<WavEndian>(self.subchunk2Size)?;

        Ok(bytes)
    }
}
