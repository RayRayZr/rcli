use crate::{process_jwt_sign, process_jwt_verify, CmdExecutor};
use clap::Parser;

#[derive(Debug, Parser)]
pub enum JWTSubCommand {
    #[command(name = "sign", about = "sign jwt token")]
    Sign(JWTSignOpts),

    #[command(name = "verify", about = "verify jwt token")]
    Verify(JWTVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JWTSignOpts {
    #[arg(short, long)]
    pub key: String,

    #[arg(short, long)]
    pub input: String,

    #[arg(short, long, default_value = "")]
    pub aud: String,
}

#[derive(Debug, Parser)]
pub struct JWTVerifyOpts {
    #[arg(short, long)]
    pub key: String,

    #[arg(short, long)]
    pub input: String,
}

impl CmdExecutor for JWTSubCommand {
    async fn execute(&self) -> anyhow::Result<()> {
        match self {
            JWTSubCommand::Sign(opts) => {
                let token = process_jwt_sign(&opts.input, &opts.key, &opts.aud).await?;
                println!("{}", token);
                Ok(())
            }
            JWTSubCommand::Verify(opts) => {
                let res = process_jwt_verify(&opts.input, &opts.key).await?;
                println!("{}", res);
                Ok(())
            }
        }
    }
}
