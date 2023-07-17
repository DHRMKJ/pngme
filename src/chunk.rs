
#![allow(unused_variables)]

use crc::{Crc, CRC_32_ISO_HDLC};
use std::convert::TryFrom;
use std::fmt::{Display, self};
use crate::chunk_type::ChunkType;

use crate::{Error, Result};

struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    crc: u32,
    data: Vec<u8>
}

#[allow(dead_code)]
impl Chunk {
    fn new(chunk_type:ChunkType, data:Vec<u8>) -> Chunk {
    
        let mut chunk_instance = Chunk {
            length: data.len() as u32,
            chunk_type,
            crc: 0,
            data
        };
        chunk_instance.crc = chunk_instance.crc();
        chunk_instance
    }

    fn crc(&self) -> u32 {
        const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let mut buffer = self.chunk_type.bytes().to_vec();
        buffer.append(&mut self.data.to_vec());      
        CRC.checksum(&buffer)
    }

    fn length(&self) -> u32 {
        self.length 
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.data.to_vec()
    }

    fn data_as_string(&self) -> Result<String> {
        let data_as_string = std::str::from_utf8(&self.data).unwrap();
        Ok(data_as_string.to_string())
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.data_as_string().unwrap())
    }
}


impl TryFrom<&Vec<u8>> for Chunk {
    type Error = Error;
    fn try_from(bytes: &Vec<u8>) -> Result<Self>{
        if bytes.len() < 8 {
            return Err(Error::from("invalid length of bytes"));
        }
        let length_slice = &bytes[0..4];
        let length = u32::from_be_bytes(length_slice.try_into().unwrap());
        let mut chunk_type_slice = [0; 4];
        for i in 4..8 {
            chunk_type_slice[i-4] = bytes[i]; 
        }
        let chunk_type = ChunkType::try_from(chunk_type_slice).unwrap();

        if length + 8 >= bytes.len() as u32 {
            return Err(Error::from("Invalid length of data"));
        }
        let mut data_slice: Vec<u8> = vec![];
        for i in 8..(8+length) {
            data_slice.push(bytes[i as usize]);
        }
        let final_chunk = Chunk::new(chunk_type, data_slice);
        let crc_slice = &bytes[(length + 8) as usize..];
        let crc = u32::from_be_bytes(crc_slice.try_into().unwrap());
        if final_chunk.crc != crc {
            return Err(Error::from("checksum mismatch"));
        }
        Ok(final_chunk)
    }
} 
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}

