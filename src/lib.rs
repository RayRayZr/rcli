mod cli;
mod process;
mod utils;

pub use cli::*;
pub use process::*;

#[warn(async_fn_in_trait)]
#[enum_dispatch::enum_dispatch]
pub trait CmdExecutor {
    async fn execute(&self) -> anyhow::Result<()>;
}
