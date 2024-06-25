use clap::Parser;

use crate::CmdExecutor;
use zxcvbn::zxcvbn;

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

impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = crate::process_genpass(
            self.length,
            self.upper,
            self.lower,
            self.number,
            self.symbol,
        )?;
        println!("{}", ret);

        // output password strength in stderr
        let estimate = zxcvbn(&ret, &[]);
        eprintln!("Password strength: {}", estimate.score());
        Ok(())
    }
}
