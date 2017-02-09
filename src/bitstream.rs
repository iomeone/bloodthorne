use std::io;

pub struct BitStream {
    buffer: Vec<u8>,
    position: usize,
    bit_val: u64, // Value of the remaining bits in the current byte
    bit_count: u32, // Number of remaining bits in the current byte
}

impl BitStream {
    pub fn new(bytes: Vec<u8>) -> BitStream {
        BitStream {
            buffer: bytes,
            position: 0,
            bit_val: 0,
            bit_count: 0,
        }
    }

    pub fn read_bits(&mut self, n: u8) -> io::Result<u32> {
        let n = n as u32;

        while n > self.bit_count {
            self.bit_val |= self.next_byte().map(|a| a as u64)? << self.bit_count;
            self.bit_count += 8;
        }

        let x = self.bit_val & ((1 << n) - 1);
        self.bit_val >>= n;
        self.bit_count -= n;

        Ok(x as u32)
    }

    pub fn next_byte(&mut self) -> io::Result<u8> {
        if self.position + 1 > self.buffer.len() {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected EOF"))
        } else {
            self.position += 1;
            Ok(self.buffer[self.position - 1])
        }
    }

    pub fn read_bytes(&mut self, n: usize) -> io::Result<Vec<u8>> {
        if self.position + n > self.buffer.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected EOF"));
        }

        // Byte-aligned
        if self.bit_count == 0 {
            self.position += n;
            return Ok(self.buffer[self.position - n..self.position].to_vec());
        }

        // Not byte-aligned
        let mut bytes = Vec::<u8>::new();

        for _ in 0..n {
            bytes.push(self.read_bits(8)? as u8);
        }

        Ok(bytes)
    }

    pub fn read_ubitvarint(&mut self) -> io::Result<u32> {
        let v = self.read_bits(6)? as u32;

        let result = match v & 48 {
            16 => (v & 15) | (self.read_bits(4)? as u32) << 4,
            32 => (v & 15) | (self.read_bits(8)? as u32) << 4,
            48 => (v & 15) | (self.read_bits(28)? as u32) << 4,
            _ => v,
        };

        Ok(result)
    }

    pub fn read_u32var(&mut self) -> io::Result<u32> {
        let max = 32u32;
        let m: u32 = max + 6;
        let mut s = 0u32;
        let mut v = 0u32;
        let mut b: u32;

        loop {
            b = self.read_bits(8)? as u32;
            v |= (b & 0x7F) << s;
            s += 7;
            if ((b & 0x80) == 0) || s == m {
                break;
            }
        }

        Ok(v)
    }

    pub fn remaining_bits(&self) -> usize {
        self.remaining_bytes() * 8 + self.bit_count as usize
    }

    pub fn remaining_bytes(&self) -> usize {
        self.buffer.len() - self.position
    }
}

#[cfg(test)]
mod tests {
    use bitstream::BitStream;
    use std::io::{Error, ErrorKind};

    #[test]
    fn test_read_one_bit() {
        let mut b = BitStream::new(vec![0b0000_0001]);
        assert_eq!(b.read_bits(1).unwrap(), 1);
    }

    #[test]
    fn test_read_all_bit_by_bit() {
        let mut b = BitStream::new(vec![0b0000_0101]);
        assert_eq!(b.read_bits(1).unwrap(), 1);
        assert_eq!(b.read_bits(1).unwrap(), 0);
        assert_eq!(b.read_bits(1).unwrap(), 1);
        assert_eq!(b.read_bits(1).unwrap(), 0);

        assert_eq!(b.read_bits(1).unwrap(), 0);
        assert_eq!(b.read_bits(1).unwrap(), 0);
        assert_eq!(b.read_bits(1).unwrap(), 0);
        assert_eq!(b.read_bits(1).unwrap(), 0);
    }

    #[test]
    fn test_read_all_bit_by_bit_EOF() {
        let mut b = BitStream::new(vec![0b0000_0101]);
        assert_eq!(b.read_bits(1).unwrap(), 1);
        assert_eq!(b.read_bits(1).unwrap(), 0);
        assert_eq!(b.read_bits(1).unwrap(), 1);
        assert_eq!(b.read_bits(1).unwrap(), 0);

        assert_eq!(b.read_bits(1).unwrap(), 0);
        assert_eq!(b.read_bits(1).unwrap(), 0);
        assert_eq!(b.read_bits(1).unwrap(), 0);
        assert_eq!(b.read_bits(1).unwrap(), 0);

        match b.read_bits(1) {
            Err(err) => assert_eq!(err.kind(), ErrorKind::UnexpectedEof),
            _ => panic!(),
        }
    }

}