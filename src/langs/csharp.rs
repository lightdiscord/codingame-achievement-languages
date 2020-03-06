use crate::{Displayer, Translate};
use std::fmt;

pub struct CSharp;

impl Translate for CSharp {
    fn translate(displayer: &Displayer, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "using System.Diagnostics;\n")?;
        write!(f, "class Solution {{\n")?;
        write!(f, "\tstatic void Main() {{\n")?;
        write!(f, "\t\tProcess.Start(\"{}\", \"-e \\\"{}\\\"\");\n", displayer.node, displayer.code)?;
        write!(f, "\t}}\n")?;
        write!(f, "}}")?;

        Ok(())
    }
}
