use clap::Clap;

#[derive(Clap)]
pub enum SourceLanguage {
    #[clap(alias = "js")]
    Javascript
}

#[derive(Clap)]
pub enum DestinationLanguage {
    C
}

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(author, about, version)]
pub struct Opts {
    /// Sets the path for the node binary
    #[clap(short, long, default_value = "/usr/local/lib/nodejs/bin/node")]
    pub node_path: String,

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
