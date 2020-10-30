use std::io::{self, Read};
use cli_core::Opts;
use translator::{FromSource, IntoDestination};

fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn transform(opts: Opts) {
    let code = read_stdin().unwrap();
    let code = opts.source.from_source(code);
    let code = opts.destination.into_destination(code);
    println!("{}", code)
}
