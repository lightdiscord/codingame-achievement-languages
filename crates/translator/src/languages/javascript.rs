use cal::POLYFILL;
use crate::FromSource;

pub struct Javascript;

impl FromSource for Javascript {
    fn from_source(self, code: String) -> String {
        let code = format!("{};{}", POLYFILL, code);
        let code = base64::encode(code);

        format!("eval(Buffer.from('{}', 'base64').toString())", code)
    }
}
