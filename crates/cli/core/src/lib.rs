mod languages;

use clap::Clap;
pub use crate::languages::*;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(author, about, version)]
pub struct Opts {
    /// Sets the source code language
    #[clap(short, long, arg_enum, default_value = "js")]
    pub source: SourceLanguage,

    /// Sets the destination language
    #[clap(short, long, arg_enum)]
    pub destination: DestinationLanguage,

    #[clap(subcommand)]
    pub subcmd: Subcommand,
}

#[derive(Clap)]
pub enum Subcommand {
    /// Transform a code from a language to another
    Transform,
}
