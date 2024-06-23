mod cli;
mod process;
mod utils;

pub use cli::*;
use enum_dispatch::enum_dispatch;
pub use process::*;
pub use utils::*;
// pub use cli::{
//     Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64Subcommand, HttpServeOpts,
//     HttpSubCommand, Opts, OutputFormat, SubCommand, TextSignFormat, TextSubcommand,
// };
// pub use process::{
//     process_csv, process_decode, process_encode, process_genpass, process_http_serve,
//     process_text_key_generate, process_text_sign, process_text_verify,
// };
// pub use utils::{get_content, get_reader};

// after rust 1.75, async fn in trait is allowed
#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecuter {
    async fn execute(self) -> anyhow::Result<()>;
}
