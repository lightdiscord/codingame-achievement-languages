use clap::Clap;
use translator::{FromSource, IntoDestination};

#[derive(Clap)]
pub enum SourceLanguage {
    #[clap(alias = "js")]
    Javascript
}

#[derive(Clap)]
pub enum DestinationLanguage {
    #[clap(name = "clang", alias = "c")]
    CLang
}

impl FromSource for SourceLanguage {
    fn from_source(self, code: String) -> String {
        match self {
            SourceLanguage::Javascript => FromSource::from_source(translator::languages::Javascript, code)
        }
    }
}

impl IntoDestination for DestinationLanguage {
    fn into_destination(self, code: String) -> String {
        match self {
            DestinationLanguage::CLang => IntoDestination::into_destination(translator::languages::CLang, code)
        }
    }
}
