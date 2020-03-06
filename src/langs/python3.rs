use crate::{Displayer, Translate};
use std::fmt;

pub struct Python3;

impl Translate for Python3 {
    fn translate(displayer: &Displayer, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "from os import system\n")?;
        write!(f, "system(\"{} -e \\\"{}\\\"\")", displayer.node, displayer.code)?;

        Ok(())
    }
}
