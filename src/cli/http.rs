use crate::utils::verify_path;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct ServeOpts {
    #[arg(short, long, value_parser=verify_path, default_value = ".")]
    pub dir: PathBuf,

    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

#[derive(Debug, Parser)]
pub enum HTTPSubCommand {
    #[command(name = "server", about = "http server")]
    Server(ServeOpts),
}
