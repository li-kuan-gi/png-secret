use std::str::FromStr;

use pngsecret::{chunk::Chunk, chunk_type::ChunkType, png::Png, result::Result};

pub fn encode(
    file_path: String,
    chunk_type: String,
    message: String,
    output_file: String,
) -> Result<()> {
    let bytes = std::fs::read(file_path)?;
    let mut png = Png::try_from(&bytes[..])?;

    let chunk_type = ChunkType::from_str(&chunk_type)?;
    let data = message.as_bytes().to_owned();
    let chunk = Chunk::new(chunk_type, data);

    png.append_chunk(chunk);

    Ok(std::fs::write(output_file, png.as_bytes())?)
}

pub fn decode(file_path: String, chunk_type: String) -> Result<()> {
    let bytes = std::fs::read(file_path)?;
    let png = Png::try_from(&bytes[..])?;

    let content = png
        .chunk_by_type(&chunk_type)
        .ok_or("no chunk with such type")?
        .data_as_string()?;

    println!("The content is:\n{}", content);
    Ok(())
}

pub fn remove(file_path: String, chunk_type: String, output_file: String) -> Result<()> {
    let bytes = std::fs::read(file_path)?;
    let mut png = Png::try_from(&bytes[..])?;

    png.remove_chunk(&chunk_type)?;

    std::fs::write(output_file, png.as_bytes())?;
    Ok(())
}

pub fn print(file_path: String) -> Result<()> {
    let bytes = std::fs::read(file_path)?;
    let png = Png::try_from(&bytes[..])?;

    for chunk in png.chunks() {
        println!("{}\n", chunk);
    }

    Ok(())
}
