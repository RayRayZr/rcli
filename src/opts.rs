use clap::Parser;
use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;

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
}
#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short,long,value_parser=verify_input_file)]
    pub input: String, // 输入文件
    #[arg(short, long)]
    pub output: Option<String>, // 输出文件
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char, //  分割符
    #[arg(long, default_value_t = true)]
    pub header: bool,

    #[arg(long, default_value = "json", value_parser=parser_formatter)]
    pub formatter: OutputFormat,
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format {}", s)),
        }
    }
}

fn parser_formatter(value: &str) -> Result<OutputFormat, anyhow::Error> {
    value.parse()
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exists")
    }
}
