use crate::{Displayer, Translate};
use std::fmt;

pub struct Clojure;

impl Translate for Clojure {
    fn translate(displayer: &Displayer, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(ns Solution (:gen-class))\n")?;
        write!(f, r#"(defn -main [& args] (-> (ProcessBuilder. ["{}" "-e" "{}"]) .inheritIO .start))"#, displayer.node, displayer.code)?;

        Ok(())
    }
}
