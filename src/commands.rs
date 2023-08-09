
#![allow(unused_variables)]
use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

use crc::{Crc, CRC_32_ISO_HDLC};

use crate::chunk::Chunk;
use crate::{Error, Result, chunk_type};
use crate::args::{ EncodeArgs, DecodeArgs};
use crate::png::Png;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let png_file_as_bytes =  std::fs::read(&args.path)
        .map_err(|err| Error::from(format!("Error reading `{:?}`: {}", args.path, err)))?;
    let mut png = Png::try_from(&png_file_as_bytes[..])
        .map_err(|err| Error::from(format!("invalid png file`{:?}`: {}", args.path, err)))?;

    let data_length: u32 = args.message.len() as u32;
    let message_bytes = args.message.as_bytes();
    let chunk_type = args.chunk_type.as_bytes();
    const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    let mut data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .copied()
            .collect();
    let buffer = &data[4..(data_length as usize + 8)];
    let crc = CRC.checksum(buffer);
    data.extend(crc.to_be_bytes().iter());
    let chunk_data = Chunk::try_from(data.as_ref())
        .map_err(|err| Error::from(format!("Invalid chunk_type`{:?}`: {}", args.chunk_type, err)))?;

    png.append_chunk(chunk_data);

    fs::write(&args.path,png.as_bytes()).map_err(
    |err| Error::from(format!("failed to write`{:?}`: {}", &args.path, err))
    )?;
    Ok(())
}

///Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png_file_as_bytes =  std::fs::read(&args.path)
        .map_err(|err| Error::from(format!("Error reading `{:?}`: {}", args.path, err)))?;
    let png = Png::try_from(&png_file_as_bytes[..])
        .map_err(|err| Error::from(format!("invalid png file`{:?}`: {}", args.path, err)))?;
    
   
    let encoded_message = png.chunk_by_type(&args.chunk_type[..]).unwrap();
    let decoded_message = encoded_message.data_as_string().unwrap();
    
    println!("{:?}",decoded_message);

    Ok(())
}

// /// Removes a chunk from a PNG file and saves the result
// pub fn remove(args: RemoveArgs) -> Result<()> {
//      println!("remove");
//     Ok(())
// }

// /// Prints all of the chunks in a PNG file
// pub fn print_chunks(args: PrintArgs) -> Result<()> {
//      println!("print");
//     Ok(())
// }

