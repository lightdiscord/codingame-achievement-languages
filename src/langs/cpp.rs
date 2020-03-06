use crate::{Displayer, Translate};
use std::fmt;

pub struct CPP;

impl Translate for CPP {
    fn translate(displayer: &Displayer, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#include <stdlib.h>\n")?;
        write!(f, "int main() {{\n")?;
        write!(f, "\tsystem(\"{} -e \\\"{}\\\"\");\n", displayer.node, displayer.code)?;
        write!(f, "}}")?;

        Ok(())
    }
}
