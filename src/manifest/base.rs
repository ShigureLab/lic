use std::fs;

pub trait Manifest {
    fn license(self) -> String;
    fn filename() -> String;
    fn from_str(text: &str) -> Self;
    fn read() -> Option<String> {
        // TODO: 自动向上层寻找
        match fs::read_to_string(Self::filename()) {
            Ok(text) => Some(text),
            Err(_) => None,
        }
    }
}
