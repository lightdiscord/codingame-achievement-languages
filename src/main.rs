use std::io::{self, Read};
use failure::Fallible;

use node_everywhere::Displayer;

fn main() -> Fallible<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let displayer = Displayer {
        node: "/usr/local/lib/nodejs/bin/node".to_string(),
        code: buffer
    };

    println!("{}", displayer);

    Ok(())
}
