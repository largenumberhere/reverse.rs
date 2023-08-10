use std::io::Read;

// Read an array of bytes and return it in a vec
pub trait ReadOutBytesN {
    fn read_out_bytes_array<const N: usize>(&mut self) -> Result<[u8; N], std::io::Error>;
}

impl<R: Read> ReadOutBytesN for R {
    fn read_out_bytes_array<const N: usize>(&mut self) -> Result<[u8; N], std::io::Error> {
        let mut buff: [u8; N] = [0; N];
        self.read_exact(&mut buff)?;

        return Ok(buff);
    }
}

// Read a vec of bytes and return it
pub trait ReadOutBytesV {
    fn read_out_bytes_vec(&mut self, len: usize) -> Result<Vec<u8>, std::io::Error>;
}

impl<R: Read> ReadOutBytesV for R {
    fn read_out_bytes_vec(&mut self, len: usize) -> Result<Vec<u8>, std::io::Error> {
        let mut buff: Vec<u8> = vec![0; len];

        self.read_exact(&mut buff)?;

        Ok(buff)
    }
}
