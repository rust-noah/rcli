mod cli;
mod process;
mod utils;

pub use cli::{Base64Format, Base64Subcommand, Opts, OutputFormat, SubCommand};
pub use process::{process_csv, process_decode, process_encode, process_genpass};
pub use utils::{get_content, get_reader};
// pub use cli::*;
