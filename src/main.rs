use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: Vec<String>, // --name <name> ... --name <name>
    verbose: bool, // --verbose
}

fn main() {
    let cli = Cli::parse();

    println!("name: {:?}", cli.name);
}
