use crate::byte_utils::ReadOutBytesV;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub struct ChunkFileBackwards<TFile> {
    file_reader: TFile,
    file_position: u64,
    min_position: u64,
    chunk_size: u64,
}

impl<TFile: Seek+ Read> ChunkFileBackwards<TFile> {
    pub fn new(
        mut file: TFile,
        min_position: u64,
        chunk_size: u64,
    ) -> Result<ChunkFileBackwards<TFile>, std::io::Error> {
        let new_position = file.seek(SeekFrom::End(0))?;

        Ok(ChunkFileBackwards {
            file_position: new_position,
            file_reader: file,
            min_position,
            chunk_size,
        })
    }

    pub fn destruct(self) -> (TFile, u64) {
        (self.file_reader, self.file_position)
    }
}

impl<TFile: Seek+ Read> Iterator for ChunkFileBackwards<TFile> {
    type Item = Result<Vec<u8>, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let step = -(self.chunk_size as i64);

        let new_pos = match self.file_reader.seek(SeekFrom::Current(step)) {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        if new_pos < self.min_position {
            return None;
        } else {
            self.file_position = new_pos;
        }

        let chunk = match self
            .file_reader
            .read_out_bytes_vec(self.chunk_size as usize)
        {
            Ok(v) => v,
            Err(e) => {
                return Some(Err(e));
            }
        };
        match self.file_reader.seek(SeekFrom::Current(step)) {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };

        Some(Ok(chunk))
    }
}
