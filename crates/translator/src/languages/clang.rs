use cal::NODE;
use crate::IntoDestination;

pub struct CLang;

impl IntoDestination for CLang {
    fn into_destination(self, code: String) -> String {
        format!("int main(void) {{system(\"{} -e \\\"{}\\\"\");}}", NODE, code)
    }
}
