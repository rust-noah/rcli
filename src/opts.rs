use clap::Parser;
use std::{path::Path, str::FromStr};

// rcli csv -i input.csv -o output.json --header -d ','
// -> 支持多格式: json, yaml
// -> 取消 -o 参数, 默认输出到 output.xxx
// rcli csv -i input.csv --format json
// rcli csv -i input.csv --format yaml

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

#[derive(Debug, Copy, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
    // Toml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// Input file
    #[arg(short, long, value_parser=verify_input_file)]
    pub input: String,

    /// Output file
    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    pub output: String,

    /// Output format
    #[arg(long, value_parser = parse_output_format, default_value = "json")]
    pub format: OutputFormat,

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

// 会传入文件名
// csv --input filename(xxx.csv)
pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        // Ok(filename.to_string())
        Ok(filename.into())
    } else {
        Err("File not found")
    }
}

// 会传入 json or yaml
// -> csv --format json or csv --format yaml
fn parse_output_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

// impl From<OutputFormat> for &'static str {
//     fn from(format: OutputFormat) -> Self {
//         match format {
//             OutputFormat::Json => "json",
//             OutputFormat::Yaml => "yaml",
//             // OutputFormat::Toml => "toml",
//         }
//     }
// }

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            // "toml" => Ok(OutputFormat::Toml), // unsupported format
            _ => Err(anyhow::anyhow!("Invalid output format: {}", format)),
        }
    }
}
