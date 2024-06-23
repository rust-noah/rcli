use std::path::{Path, PathBuf};

use clap::Parser;
use enum_dispatch::enum_dispatch;

mod base64;
mod csv;
mod genpass;
mod http;
mod text;

// pub use csv_opts::{CsvOpts, OutputFormat};
// pub use genpass_opts::GenPassOpts;

// 使用 self 避免歧义
// pub use self::base64::{Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64Subcommand};
// pub use self::csv::{CsvOpts, OutputFormat};
// pub use self::genpass::GenPassOpts;
// pub use self::http::{HttpServeOpts, HttpSubCommand};
// pub use self::text::{TextSignFormat, TextSignOpts, TextSubcommand, TextVerifyOpts};
pub use self::{base64::*, csv::*, genpass::*, http::*, text::*};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecuter)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64Subcommand),
    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubcommand),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
}

// 会传入文件名
// csv --input filename(xxx.csv)
// pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
//     if Path::new(filename).exists() {
//         // Ok(filename.to_string())
//         Ok(filename.into())
//     } else {
//         Err("File not found")
//     }
// }

// region:    --- functions
// 参数需要什么类型, 这种 verify 的返回值就是 Result<需要的类型, &'static str>
// 右侧的是 E, 可以是 &'static str, 也可以是 String(看自己的需求)
fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    // if input is "-" or file exists
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}
// endregion: --- functions

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File does not exist"));
    }
}
