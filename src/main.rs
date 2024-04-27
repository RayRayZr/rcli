use clap::Parser;
use rcli::csv_convert::OutputFormat;
use rcli::{process_csv, process_genpass};
use rcli::{Opts, Subcommand};
use std::path::Path;

fn main() -> anyhow::Result<()> {
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
        Subcommand::GenPass(ops) => process_genpass(
            ops.uppercase,
            ops.lowercase,
            ops.number,
            ops.symbol,
            ops.length,
        ),
        _ => {}
    }
    Ok(())
}
