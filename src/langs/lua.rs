use crate::{Displayer, Translate};
use std::fmt;

pub struct Lua;

impl Translate for Lua {
    fn translate(displayer: &Displayer, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "os.execute(\"{} -e \\\"{}\\\"\")", displayer.node, displayer.code)
    }
}
