use crate::byte_utils::ReadOutBytesN;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::Write;
use reverse::ToBytes;
use reverse::StructFromBytes;

use struct_bytes_derive::{StructFromBytes, ToBytes};

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

#[derive(StructFromBytes)]
#[derive(ToBytes)]
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
