use crate::{Displayer, Translate};
use std::fmt;

pub struct OCaml;

impl Translate for OCaml {
    fn translate(displayer: &Displayer, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sys.command \"{} -e \\\"{}\\\"\"", displayer.node, displayer.code)
    }
}
