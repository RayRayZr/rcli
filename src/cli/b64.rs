use anyhow::anyhow;
use clap::Parser;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, default_value = "-")]
    pub input: String,

    #[arg(long,value_parser=parse_base64_format, default_value = "standard")]
    pub formatter: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, default_value = "-")]
    pub input: String,

    #[arg(long,value_parser=parse_base64_format, default_value = "standard")]
    pub formatter: Base64Format,
}

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "encode base64 string")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "decode base64 string")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser, Clone)]
pub enum Base64Format {
    Standard,
    Url,
}

fn parse_base64_format(s: &str) -> Result<Base64Format, anyhow::Error> {
    s.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "url" => Ok(Base64Format::Url),
            _ => Err(anyhow!("Invalid base64 format")),
        }
    }
}
