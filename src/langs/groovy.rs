use crate::{Displayer, Translate};
use std::fmt;

pub struct Groovy;

impl Translate for Groovy {
    fn translate(displayer: &Displayer, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"new ProcessBuilder("{}", "-e" , "{}")"#, displayer.node, displayer.code)?;
        write!(f, "\n\t.inheritIO()\n")?;
        write!(f, "\t.start()\n")?;

        Ok(())
    }
}
