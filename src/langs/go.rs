use crate::{Displayer, Translate};
use std::fmt;

pub struct Go;

impl Translate for Go {
    fn translate(displayer: &Displayer, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "package main\n")?;
        write!(f, "import \"os\"\n")?;
        write!(f, "import \"os/exec\"\n")?;
        write!(f, "func main() {{\n")?;
        write!(f, "\tcmd := exec.Command(\"{}\", \"-e\", \"{}\");\n", displayer.node, displayer.code)?;
        write!(f, "\tcmd.Stdin = os.Stdin\n")?;
        write!(f, "\tcmd.Stdout = os.Stdout\n")?;
        write!(f, "\tcmd.Stderr = os.Stderr\n")?;
        write!(f, "\tcmd.Run()\n")?;
        write!(f, "}}")?;

        Ok(())
    }
}
