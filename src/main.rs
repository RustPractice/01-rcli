use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, Opts, SubCommand, process_genpass};

// rcli csv -i input.csv -o output.csv --header -d ','

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }
        SubCommand::Genpass(opts) => {
            println!("{:?}", opts);
            let res = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", res);
        }
    }

    Ok(())
}
