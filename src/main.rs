use clap::Parser;
use rcli::csv_convert::OutputFormat;
use rcli::{
    decode_base64, encode_base64, generate_key, process_csv, process_genpass, process_http_serve,
    sign_text, verify_text,
};
use rcli::{Opts, Subcommand};
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts: Opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(ops) => {
            let output = if let Some(output) = ops.output {
                output
            } else {
                let mut output = Path::new(&ops.input)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                match ops.formatter {
                    OutputFormat::Json => {
                        output.push_str(".json");
                    }
                    OutputFormat::Yaml => {
                        output.push_str(".yaml");
                    }
                }
                output
            };
            process_csv(&ops.input, output, ops.formatter)?;
        }
        Subcommand::GenPass(ops) => {
            process_genpass(
                ops.uppercase,
                ops.lowercase,
                ops.number,
                ops.symbol,
                ops.length,
            )?;
        }
        Subcommand::Base64(sub_cmd) => match sub_cmd {
            rcli::b64::Base64SubCommand::Encode(ops) => {
                encode_base64(&ops.input, ops.formatter)?;
            }
            rcli::b64::Base64SubCommand::Decode(ops) => {
                decode_base64(&ops.input, ops.formatter)?;
            }
        },
        Subcommand::Text(sub_cmd) => match sub_cmd {
            rcli::text::TextSubCommand::Sign(ops) => {
                sign_text(&ops.input, &ops.key, ops.formatter)?;
            }
            rcli::text::TextSubCommand::Verify(ops) => {
                verify_text(&ops.input, &ops.key, &ops.signature, ops.formatter)?;
            }
            rcli::text::TextSubCommand::GenerateKey(ops) => {
                generate_key(ops.formatter, &ops.output)?;
            }
        },
        Subcommand::Http(sub_cmd) => match sub_cmd {
            rcli::http::HTTPSubCommand::Server(ops) => {
                process_http_serve(ops.dir, ops.port).await?;
            }
        },
    }
    Ok(())
}
