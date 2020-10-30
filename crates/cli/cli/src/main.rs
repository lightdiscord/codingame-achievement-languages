use clap::Clap;

use cli_core::{ Opts, Subcommand };
use cli_command_transform::transform;

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        Subcommand::Transform => transform(opts)
    };
}
