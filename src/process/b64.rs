use crate::b64::Base64Format;
use crate::utils::read_input;
use base64::engine::general_purpose::STANDARD;
use base64::engine::general_purpose::URL_SAFE;
use base64::engine::GeneralPurpose;
use base64::Engine as _;

pub fn encode_base64(input: &str, formatter: Base64Format) -> anyhow::Result<()> {
    let buf = read_input(input)?;
    let output = handle_formatter(formatter).encode(buf);
    println!("{}", output);
    Ok(())
}

pub fn decode_base64(input: &str, formatter: Base64Format) -> anyhow::Result<()> {
    let buf = read_input(input)?;
    let output = handle_formatter(formatter).decode(buf)?;
    println!("{}", String::from_utf8(output)?);
    Ok(())
}

fn handle_formatter(format: Base64Format) -> GeneralPurpose {
    match format {
        Base64Format::Standard => STANDARD,
        Base64Format::Url => URL_SAFE,
    }
}
