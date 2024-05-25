use crate::utils::{verify_input, verify_input_file};
use clap::Parser;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "sign text with private key")]
    Sign(SignTextOpts),

    #[command(name = "verify", about = "verify text with public key")]
    Verify(VerifyTextOpts),

    #[command(name = "generate-key", about = "generate key pair")]
    GenerateKey(GenerateKeyOpts),
}

#[derive(Debug, Parser)]
pub struct GenerateKeyOpts {
    #[arg(short, long, value_parser = verify_input)]
    pub output: String,

    #[arg(long, default_value = "blake3", value_parser = parse_formatter)]
    pub formatter: TextSignFormatter,
}

#[derive(Debug, Parser)]
pub struct SignTextOpts {
    #[arg(short, long, default_value = "-", value_parser = verify_input)]
    pub input: String,

    #[arg(short, long, value_parser=verify_input_file)]
    pub key: String,

    #[arg(long, default_value = "blake3", value_parser = parse_formatter)]
    pub formatter: TextSignFormatter,
}

#[derive(Debug, Parser)]
pub struct VerifyTextOpts {
    #[arg(short, long, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser=verify_input_file)]
    pub key: String,

    #[arg(long, value_parser=verify_input_file)]
    pub signature: String,

    #[arg(long, default_value = "blake3", value_parser = parse_formatter)]
    pub formatter: TextSignFormatter,
}

#[derive(Debug, Parser, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextSignFormatter {
    Blake3,
    Ed25519,
}

fn parse_formatter(s: &str) -> Result<TextSignFormatter, anyhow::Error> {
    s.parse()
}

impl FromStr for TextSignFormatter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        match s {
            "blake3" => Ok(TextSignFormatter::Blake3),
            "ed25519" => Ok(TextSignFormatter::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid text sign formatter")),
        }
    }
}

impl From<TextSignFormatter> for &'static str {
    fn from(f: TextSignFormatter) -> &'static str {
        match f {
            TextSignFormatter::Blake3 => "blake3",
            TextSignFormatter::Ed25519 => "ed25519",
        }
    }
}

impl Display for TextSignFormatter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "textformatter:{}", Into::<&str>::into(*self))
    }
}
