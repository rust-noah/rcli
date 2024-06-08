use clap::Parser;
use std::ops::RangeInclusive;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Network port to use
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,
    /// Network port to use
    #[arg(value_parser = port_in_range)]
    port2: u16,
}

fn main() {
    let cli = Cli::parse();

    println!("PORT = {}", cli.port);
}

// Custom validation
const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

fn port_in_range(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a port number"))?;
    if PORT_RANGE.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}
