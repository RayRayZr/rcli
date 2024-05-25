use std::fs::File;
use std::io::Read;

pub fn read_input(input: &str) -> Result<Vec<u8>, anyhow::Error> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn read_file(input: &str) -> Result<Vec<u8>, anyhow::Error> {
    let mut reader = Box::new(File::open(input)?);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}
