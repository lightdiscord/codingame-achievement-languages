use crate::{Displayer, Translate};
use std::fmt;

pub struct Java;

impl Translate for Java {
    fn translate(displayer: &Displayer, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "import java.io.*;\n")?;
        write!(f, "class Solution {{\n")?;
        write!(f, "\tpublic static void main(String args[]) throws IOException {{\n")?;
        write!(f, "\t\tnew ProcessBuilder(\"{}\", \"-e\" , \"{}\")\n", displayer.node, displayer.code)?;
        write!(f, "\t\t\t.inheritIO()\n")?;
        write!(f, "\t\t\t.start();\n")?;
        write!(f, "\t}}\n")?;
        write!(f, "}}")?;

        Ok(())
    }
}
