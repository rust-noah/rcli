use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    /// Length of the password
    #[arg(short, long, default_value = "16")]
    pub length: u8,

    /// Uppercase letters
    #[arg(long, default_value_t = true)]
    pub upper: bool,

    /// Lowercase
    #[arg(long, default_value_t = true)]
    pub lower: bool,

    /// Numbers
    #[arg(short, long, default_value_t = true)]
    pub number: bool,

    /// Symbols
    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}
