use std::path::Path;

use clap::Parser;

mod base64;
mod csv;
mod genpass;

// pub use csv_opts::{CsvOpts, OutputFormat};
// pub use genpass_opts::GenPassOpts;

// 使用 self 避免歧义
pub use self::base64::{Base64Format, Base64Subcommand};
pub use self::csv::{CsvOpts, OutputFormat};
pub use self::genpass::GenPassOpts;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64Subcommand),
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

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("not-exist"), Err("File does not exist"));
    }
}
