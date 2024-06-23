use clap::Parser;
use std::{fmt, str::FromStr};

use crate::{process_csv, CmdExecuter};

use super::verify_file;

// rcli csv -i input.csv -o output.json --header -d ','
// -> 支持多格式: json, yaml
// -> 取消 -o 参数, 默认输出到 output.xxx
// rcli csv -i input.csv --format json
// rcli csv -i input.csv --format yaml
#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// Input file
    #[arg(short, long, value_parser=verify_file)]
    pub input: String,

    /// Output file
    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    pub output: Option<String>,

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

#[derive(Debug, Copy, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
    // Toml,
}

// region:    --- impls
impl CmdExecuter for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let output = if let Some(output) = self.output {
            output
        } else {
            format!("output.{}", self.format)
        };
        process_csv(&self.input, &output, &self.format)
    }
}

// 生命周期与内存一样的类型, 统称为 'static (静态生命周期)
// 1. const
// 2. Box::leak

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            // OutputFormat::Toml => "toml",
        }
    }
}

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

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
// endregion: --- impls

// 会传入 json or yaml
// -> csv --format json or csv --format yaml
fn parse_output_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}
