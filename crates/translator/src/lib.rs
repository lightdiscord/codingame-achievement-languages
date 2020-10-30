pub mod languages;

pub trait FromSource {
    fn from_source(self, code: String) -> String;
}

pub trait IntoDestination {
    fn into_destination(self, code: String) -> String;
}
