use clap::Parser;
use std::path::Path;

// rcli csv -i input.csv -o output.json --header -d ','
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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// Input file
    #[arg(short, long, value_parser=verify_input_file)]
    pub input: String,

    /// Output file
    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    pub output: String,

    /// Delimiter
    #[arg(short, long, default_value_t = ',')] // ',' as char
    pub delimiter: char,

    /// CSV has header
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

// fn verify_input_file(filename: &str) -> Result<String, String> {
//     if std::path::Path::new(filename).exists() {
//         // Ok(filename.to_string())
//         Ok(filename.into())
//     } else {
//         Err(format!("File not found: {}", filename))
//     }
// }

// 生命周期与内存一样的类型, 统称为 'static (静态生命周期)
// 1. const
// 2. Box::leak
pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        // Ok(filename.to_string())
        Ok(filename.into())
    } else {
        Err("File not found")
    }
}
