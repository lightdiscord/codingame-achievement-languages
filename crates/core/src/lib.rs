pub struct Context {
    node: String,
    code: String
}

impl Context {
    pub fn new(node: String, code: String) -> Self {
        Context { node, code }
    }
}
