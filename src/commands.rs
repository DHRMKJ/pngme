use std::convert::TryFrom;
use std::fs;
use crc::{Crc, CRC_32_ISO_HDLC};
use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::png::Png;
use crate::{Error, Result};

fn make_png (path: &std::path::PathBuf) -> Result<Png> {
    let png_file_as_bytes = std::fs::read(path)
        .map_err(|err| Error::from(format!("Error reading `{:?}`: {}", path, err)))?;
    let png = Png::try_from(&png_file_as_bytes[..])
        .map_err(|err| Error::from(format!("invalid png file`{:?}`: {}", path, err)))?;
    Ok(png)
}

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let mut png  = make_png(&args.path)?;
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
    let chunk_data = Chunk::try_from(data.as_ref()).map_err(|err| {
        Error::from(format!(
            "Invalid chunk_type`{:?}`: {}",
            args.chunk_type, err
        ))
    })?;

    png.append_chunk(chunk_data);

    fs::write(&args.path, png.as_bytes())
        .map_err(|err| Error::from(format!("failed to write`{:?}`: {}", &args.path, err)))?;
    Ok(())
}

///Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = make_png(&args.path)?;
    let encoded_message = png.chunk_by_type(&args.chunk_type[..]).unwrap();
    let decoded_message = encoded_message.data_as_string().unwrap();

    println!("{:?}", decoded_message);

    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = make_png(&args.path)?;
    png.remove_chunk(&args.chunk_type[..])
        .map_err(|err| Error::from(format!("Failed to remove the chunk :{:?} : {}", args.chunk_type,err)))?;
    
    fs::write(&args.path, png.as_bytes())
        .map_err(|err| Error::from(format!("failed to write`{:?}`: {}", &args.path, err)))?;
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let png = make_png(&args.path)?;
    println!("{:?}",png.as_bytes());
    Ok(())
}
