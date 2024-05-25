pub mod b64;
pub mod csv_convert;
pub mod gen_pass;
pub mod http;
pub mod text;

use crate::b64::Base64SubCommand;
use crate::cli::text::TextSubCommand;
use crate::csv_convert::CsvOpts;
use crate::gen_pass::GenPassOpts;
use crate::http::HTTPSubCommand;
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

    #[command(subcommand)]
    Base64(Base64SubCommand),

    #[command(subcommand)]
    Text(TextSubCommand),

    #[command(subcommand)]
    Http(HTTPSubCommand),
}
