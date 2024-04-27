use clap::Parser;
use rcli::{process_csv, Opts, OutputFormat, Subcommand};
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
    }
    Ok(())
}
