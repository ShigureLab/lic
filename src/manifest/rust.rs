use super::base::{Manifest, ManifestError};
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CargoToml {
    // https://doc.rust-lang.org/cargo/reference/manifest.html
    package: Option<Package>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    license: Option<String>,
}

impl Manifest for CargoToml {
    fn license(self) -> Result<String, ManifestError> {
        self.package
            .and_then(|package| package.license)
            .ok_or(ManifestError::LicenseNotFound)
    }

    fn filename() -> String {
        "Cargo.toml".into()
    }

    fn from_str(text: &str) -> Result<Self, ManifestError> {
        toml::from_str(text).map_err(|_| ManifestError::ParseError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_toml_ok() {
        // https://github.com/ShigureLab/lic/blob/main/Cargo.toml
        let data = r#"
        [package]
        name = "lic"
        version = "0.1.1"
        edition = "2021"
        authors = ["Nyakku Shigure"]
        rust-version = "1.58.0"
        description = "Generate license using cli."
        license = "MIT"
        "#;
        let lic: CargoToml = toml::from_str(data).unwrap();
        assert_eq!(
            lic,
            CargoToml {
                package: Some(Package {
                    license: Some(String::from("MIT"))
                })
            }
        );
        assert_eq!(lic.license(), Ok(String::from("MIT")));
    }

    #[test]
    fn test_cargo_toml_license_not_found() {
        let data = r#"
        [package]
        "#;
        let lic: CargoToml = toml::from_str(data).unwrap();
        assert_eq!(
            lic,
            CargoToml {
                package: Some(Package { license: None })
            }
        );
        assert_eq!(lic.license(), Err(ManifestError::LicenseNotFound));
    }

    #[test]
    fn test_cargo_toml_parse_error() {
        let data = r"
        [package]
        \(^_^)/   <-- parse error
        ";
        let lic = CargoToml::from_str(data);
        assert_eq!(lic, Err(ManifestError::ParseError));
    }
}
