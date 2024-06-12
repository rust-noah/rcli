// use rcli::{process_csv, process_genpass, Opts, SubCommand};

use clap::Parser;
use rcli::{
    get_reader, process_csv, process_decode, process_encode, process_genpass, Base64Subcommand,
    Opts, SubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            process_csv(&opts.input, &opts.output, &opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.upper,
                opts.lower,
                opts.number,
                opts.symbol,
            )?;
            println!("Generate password: {:?}", password);
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64Subcommand::Encode(opts) => {
                let mut reader = get_reader(&opts.input)?;
                process_encode(&mut reader, opts.format)?;
            }
            Base64Subcommand::Decode(opts) => {
                // get_reader 返回值是 Box 类型
                // Box 当做没有既可, 会自动 deref
                let mut reader = get_reader(&opts.input)?;
                process_decode(&mut reader, opts.format)?;
            }
        },
    }
    Ok(())
}
