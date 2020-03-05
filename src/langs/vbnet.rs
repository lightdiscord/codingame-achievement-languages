use crate::{Displayer, Translate};
use std::fmt;

pub struct VBNet;

impl Translate for VBNet {
    fn translate(displayer: &Displayer, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "imports System.Diagnostics\n")?;
        write!(f, "Module Solution\n")?;
        write!(f, "\tSub Main ()\n")?;
        write!(f, "\t\tProcess.Start(\"{}\", \"-e \"\"{}\"\"\")\n", displayer.node, displayer.code)?;
        write!(f, "\tEnd Sub\n")?;
        write!(f, "End Module")?;

        Ok(())
    }
}
