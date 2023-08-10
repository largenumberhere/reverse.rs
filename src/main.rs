mod byte_utils;
mod chunk_file_backwards;
mod wav_bytes;

use crate::chunk_file_backwards::ChunkFileBackwards;
use crate::wav_bytes::calculate_sample_length;
use crate::wav_bytes::contain_wav_phrase;
use std::fs::File;
use std::io::{ErrorKind, Seek, Write};
use wav_bytes::Header;

#[derive(Debug, thiserror::Error)]
enum ProgramError {
    #[error("An unexpected IO error has occurred while trying to read from file '{0}'. Maybe try again later? Error details: {1:?} ")]
    IOErrorReadingFile(String, std::io::Error),

    #[error("Usage ./reverse input.wav output.wav")]
    InvalidUsage,

    #[error("No file '{0}' was not found. Please ensure the path is correct and the file exists")]
    FileNotFound(String),

    #[error("Miscellaneous error opening file '{0}'. Error: {1}")]
    FileOpeningError(String, std::io::Error),

    #[error("'{0}' contains an invalid wav header. Perhaps it's corrupted or does not use the wav file format?")]
    InvalidWavHeader(String),

    #[error("Fatal error: converting back the bytes read from the header somehow failed catastrophically")]
    ByteConversionError,

    #[error("Failed to create file '{0}' because {1:?}")]
    FileCreationError(String, std::io::Error),
}

fn main() {
    match convert() {
        Ok(_) => {}
        Err(e) => {
            println!("{e}");
            std::process::exit(-1);
        }
    }
}

fn convert() -> Result<(), ProgramError> {
    // Make sure we were given enough arguments
    if std::env::args().count() != 3 {
        return Err(ProgramError::InvalidUsage);
    }

    let mut args = std::env::args();

    // Discard args[0] because it is just this executable's path and we don't need it
    _ = args.next();
    let file_in_path = args.next().expect("Failed to grab the first file path :(");
    let file_out_path = args.next().expect("Failed to grab the 2nd file path :(");

    // Open file0 for reading
    let file_in = std::fs::File::open(&file_in_path);
    let mut file_in = match file_in {
        Ok(v) => v,
        Err(e) => {
            return match e.kind() {
                ErrorKind::NotFound => Err(ProgramError::FileNotFound(file_in_path)),
                _ => Err(ProgramError::FileOpeningError(file_in_path, e)),
            }
        }
    };

    // Load file0's header
    // let mut file_in_reader = std::io::BufReader::new(file_in);
    let header = Header::from_reader(&mut file_in);
    let header = match header {
        Ok(v) => v,
        Err(_) => return Err(ProgramError::InvalidWavHeader(file_in_path)),
    };

    // Verify file0's header
    if !contain_wav_phrase(&header) {
        return Err(ProgramError::InvalidWavHeader(file_in_path));
    }

    // Prepare to iterate over file
    let sample_size = calculate_sample_length(&header);
    let position = file_in
        .stream_position()
        .map_err(|e| ProgramError::IOErrorReadingFile(file_in_path.to_string(), e))?;
    let backwards_file_reader = ChunkFileBackwards::new(file_in, position, sample_size as u64)
        .map_err(|e| ProgramError::IOErrorReadingFile(file_in_path.to_string(), e))?;

    // Create output file
    let output_file = File::create(&file_out_path);
    let mut output_file = match output_file {
        Ok(v) => v,
        Err(e) => return Err(ProgramError::FileCreationError(file_out_path, e)),
    };

    // Copy over header to output file
    let header_raw = header
        .to_bytes()
        .map_err(|_| ProgramError::ByteConversionError)?;
    output_file
        .write(&header_raw)
        .map_err(|e| ProgramError::IOErrorReadingFile(file_out_path.to_string(), e))?;

    // Copy over the rest in reverse to output file
    for chunk in backwards_file_reader {
        let chunk = match chunk {
            Ok(v) => v,
            Err(e) => return Err(ProgramError::IOErrorReadingFile(file_in_path, e)),
        };

        output_file
            .write(&chunk)
            .map_err(|e| ProgramError::IOErrorReadingFile(file_out_path.to_string(), e))?;
    }

    Ok(())
}
