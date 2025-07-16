use clap::Parser;
use rcli::{Opts, SubCommand};

fn main() {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            println!("{:?}", opts);
        }
    }
}
