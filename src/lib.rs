mod cli;
mod process;
mod utils;

pub use cli::{
    Base64SubCommand, HttpServeOpts, HttpSubCommand, Opts, SubCommand, TextSignFormat,
    TextSubCommand,
};
pub use process::*;
pub use utils::{get_content, get_reader};

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
