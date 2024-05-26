pub mod b64;
pub mod csv_convert;
pub mod gen_pass;
pub mod http;
pub mod myjwt;
pub mod text;

pub use crate::b64::Base64SubCommand;
pub use crate::csv_convert::CsvOpts;
pub use crate::gen_pass::GenPassOpts;
pub use crate::http::HTTPSubCommand;
pub use crate::myjwt::JWTSubCommand;
pub use crate::text::*;
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[command(name="rcli",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}
#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
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

    #[command(subcommand)]
    JWT(JWTSubCommand),
}
