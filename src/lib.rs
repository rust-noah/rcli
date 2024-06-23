mod cli;
mod process;
mod utils;

pub use cli::{
    Base64Format, Base64Subcommand, HttpSubCommand, Opts, OutputFormat, SubCommand, TextSignFormat,
    TextSubcommand,
};
pub use process::{
    process_csv, process_decode, process_encode, process_genpass, process_http_serve,
    process_text_key_generate, process_text_sign, process_text_verify,
};
pub use utils::{get_content, get_reader};
// pub use cli::*;
