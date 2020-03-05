use crate::{Displayer, Translate};
use std::fmt;

pub struct Pascal;

impl Translate for Pascal {
    fn translate(displayer: &Displayer, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = displayer.code.replace("'", "\"");

        write!(f, "program Answer;\n")?;
        write!(f, "uses sysutils;\n")?;
        write!(f, "begin\n")?;
        write!(f, "\tExecuteProcess('{}', ['-e', '{}'], []);\n", displayer.node, code)?;
        write!(f, "end.")?;

        Ok(())
    }
}
