use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(ops) => {
            process_csv(&ops.input, &ops.output)?;
        }
    }
    Ok(())
}
