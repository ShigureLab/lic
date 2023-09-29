use std::fs;
use std::marker::Sized;
#[derive(Debug, PartialEq)]
pub enum ManifestError {
    LicenseNotFound,
    FileNotFound,
    ParseError,
}

pub trait Manifest {
    fn license(self) -> Result<String, ManifestError>;
    fn filename() -> String;
    fn from_str(text: &str) -> Result<Self, ManifestError>
    where
        Self: Sized;
    fn read() -> Option<String> {
        // TODO: 自动向上层寻找
        match fs::read_to_string(Self::filename()) {
            Ok(text) => Some(text),
            Err(_) => None,
        }
    }
}
