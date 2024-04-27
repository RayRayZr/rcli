pub mod base64;
pub mod csv_convert;
pub mod gen_pass;
use crate::base64::Base64Opts;
use crate::csv_convert::CsvOpts;
use crate::gen_pass::GenPassOpts;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name="rcli",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}
#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate random password")]
    GenPass(GenPassOpts),

    #[command(name = "base64", about = "Base64 encode/decode")]
    Base64(Base64Opts),
}
