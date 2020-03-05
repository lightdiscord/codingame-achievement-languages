use crate::{Displayer, Translate};
use std::fmt;

pub struct CLang;

impl Translate for CLang {
    fn translate(displayer: &Displayer, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "int main(void) {{\n\tsystem(\"{} -e \\\"{}\\\"\");\n}}", displayer.node, displayer.code)
    }
}
