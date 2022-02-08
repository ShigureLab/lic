use super::base::Manifest;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CargoToml {
    // https://doc.rust-lang.org/cargo/reference/manifest.html
    package: Package,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    license: String,
}

impl Manifest for CargoToml {
    fn license(self) -> String {
        self.package.license
    }

    fn filename() -> String {
        "Cargo.toml".into()
    }

    fn from_str(text: &str) -> Self {
        toml::from_str(text).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_toml() {
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
                package: Package {
                    license: String::from("MIT")
                }
            }
        )
    }
}
