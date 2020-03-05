use crate::{Displayer, Translate};
use std::fmt;

pub struct Bash;

impl Translate for Bash {
    fn translate(displayer: &Displayer, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -e \"{}\"", displayer.node, displayer.code)
    }
}
