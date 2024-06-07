use clap::Parser;

// If you add #[arg()] on the structure field, then when executing the command, you need to use — or - to execute.

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'x', long)]
    noah: String, // if the type is not Option, it is a required parameter
    #[arg(short, long)]
    name: Vec<String>,
    #[arg(short, long)]
    verbose: bool, // if the type is bool, it is a flag
}

fn main() {
    let cli = Cli::parse();

    println!("name: {:?}", cli.name);
    println!("verbose: {:?}", cli.verbose);
}
