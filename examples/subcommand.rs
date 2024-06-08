use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    // Add { name: Option<String> }, // 使用结构变体作为子命令
    Add(AddArgs), // 使用结构体作为子命令
}

#[derive(Args)]
struct AddArgs {
    name: Option<String>,
    // age: Option<u8>,
    #[arg(default_value_t = 18)]
    age: u8, // can be set as optional, or can be set a default value
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        // Commands::Add { name } => {
        //     println!("'myapp add' was used, name is: {:?}", name);
        // }
        // Commands::Add(arg) => {
        //     println!("'myapp add' was used, name is: {:?}", arg.name);
        // }
        Some(Commands::Add(arg)) => {
            println!("'myapp add' was used, name is: {:?}, age is: {:?}", arg.name, arg.age);
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
