use crate::{Displayer, Translate};
use std::fmt;

pub struct FSharp;

impl Translate for FSharp {
    fn translate(displayer: &Displayer, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "System.Diagnostics.Process.Start(\"{}\", \"-e \\\"{}\\\"\")", displayer.node, displayer.code)
    }
}
