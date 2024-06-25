use crate::{process_jwt_sign, process_jwt_verify, CmdExecutor};
use anyhow::Result;
use clap::Parser;
use enum_dispatch::enum_dispatch;

// region:    --- enum and struct
#[derive(Debug, Clone, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum JwtSubCommand {
    #[command(about = "Sign a JWT token")]
    Sign(JwtSignOpts),
    #[command(about = "Verify a JWT token")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Clone, Parser)]
pub struct JwtSignOpts {
    /// key to sign the token
    #[arg(short, long, default_value = "")]
    key: String,
    #[arg(short, long)]
    /// subject
    sub: String,
    #[arg(short, long)]
    /// audience
    aud: String,
    /// expiration time
    #[arg(short, long, value_parser= parse_exp,default_value="7d")]
    exp: usize,
}

#[derive(Debug, Clone, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long, default_value = "")]
    key: String,
    #[arg(short, long)]
    token: String,
}
// endregion: --- enum and struct

// region:    --- impls
impl CmdExecutor for JwtSignOpts {
    async fn execute(self) -> Result<()> {
        let token = process_jwt_sign(&self.key, &self.sub, &self.aud, self.exp)?;
        println!("Token: {}", token);
        Ok(())
    }
}

impl CmdExecutor for JwtVerifyOpts {
    async fn execute(self) -> Result<()> {
        let verified: bool = process_jwt_verify(&self.key, &self.token)?;
        println!("Token valid: {}", verified);
        Ok(())
    }
}
// endregion: --- impls

fn parse_exp(exp: &str) -> Result<usize> {
    match fancy_duration::FancyDuration::<std::time::Duration>::parse(exp) {
        Ok(d) => Ok(d.0.as_secs() as usize),
        Err(_) => Err(anyhow::anyhow!("invalid unit: {}", exp)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// test date time parse
    #[test]
    fn test_parse_exp() {
        assert_eq!(parse_exp("1s").unwrap(), 1);
        assert_eq!(parse_exp("1m").unwrap(), 60);
        assert_eq!(parse_exp("1h").unwrap(), 60 * 60);
        assert_eq!(parse_exp("1d").unwrap(), 60 * 60 * 24);
    }
}
