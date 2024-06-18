// use rcli::{process_csv, process_genpass, Opts, SubCommand};

use clap::Parser;
use rcli::{
    get_content, get_reader, process_csv, process_decode, process_encode, process_genpass,
    process_text_sign, process_text_verify, Base64Subcommand, Opts, SubCommand, TextSignFormat,
    TextSubcommand,
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
        SubCommand::Text(subcmd) => match subcmd {
            // TextSubcommand::Sign(opts) => match opts.format {
            //     TextSignFormat::Blake3 => {
            //         let mut reader = get_reader(&opts.input)?;
            //         let key = get_content(&opts.key)?;
            //         process_text_sign(&mut reader, &key, opts.format)?;
            //     }
            //     TextSignFormat::Ed25519 => {
            //         let mut reader = get_reader(&opts.input)?;
            //         let key = get_content(&opts.key)?;
            //         process_text_sign(&mut reader, &key, opts.format)?;
            //     }
            // },
            TextSubcommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                process_text_sign(&mut reader, &key, opts.format)?;
            }
            TextSubcommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sig = get_content(&opts.sig)?;
                process_text_verify(&mut reader, &key, &sig, opts.format)?;
            }
            TextSubcommand::Generate(opts) => {
                // let mut reader = get_reader(&opts.input)?;
                // process_generate(&mut reader, &opts.key)?;
            }
        },
    }
    Ok(())
}
