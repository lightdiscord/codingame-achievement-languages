use std::fmt::{self, Display};

pub mod langs;

use langs::*;

pub trait Translate {
    fn translate(displayer: &Displayer, f: &mut fmt::Formatter) -> fmt::Result;
}

pub struct Displayer {
    pub node: String,
    pub code: String
}

impl Display for Displayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "C:\n")?;
        CLang::translate(self, f)?;

        write!(f, "\n\nVB.NET:\n")?;
        VBNet::translate(self, f)?;

        write!(f, "\n\nPascal:\n")?;
        Pascal::translate(self, f)?;

        write!(f, "\n\nOCaml:\n")?;
        OCaml::translate(self, f)?;

        write!(f, "\n\nLua:\n")?;
        Lua::translate(self, f)?;

        write!(f, "\n\nGroovy:\n")?;
        Groovy::translate(self, f)?;

        Ok(())
    }

}
