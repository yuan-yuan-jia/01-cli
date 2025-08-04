mod cli;
mod process;
mod utils;

pub use cli::*;
use enum_dispatch::enum_dispatch;
pub use process::*;
pub use utils::{get_content, get_reader};

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
