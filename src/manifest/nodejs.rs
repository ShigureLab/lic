use super::base::{Manifest, ManifestError};
use serde::Deserialize;
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    // https://docs.npmjs.com/cli/v7/configuring-npm/package-json
    license: Option<String>,
}

impl Manifest for PackageJson {
    fn license(self) -> Result<String, ManifestError> {
        self.license.ok_or(ManifestError::LicenseNotFound)
    }

    fn filename() -> String {
        "package.json".into()
    }

    fn from_str(text: &str) -> Result<Self, ManifestError> {
        serde_json::from_str(text).map_err(|_| ManifestError::ParseError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_json_ok() {
        // https://github.com/moefyit/moefy-canvas/blob/main/package.json
        let data = r#"
        {
            "name": "moefy-canvas-monorepo",
            "version": "0.3.0",
            "private": true,
            "author": "SigureMo <sigure.qaq@gmail.com>",
            "license": "MIT",
            "repository": {
                "type": "git",
                "url": "git+ssh://git@github.com/moefyit/moefy-canvas.git"
            }
        }"#;
        let lic: PackageJson = serde_json::from_str(data).unwrap();
        assert_eq!(
            lic,
            PackageJson {
                license: Some(String::from("MIT"))
            }
        );
        assert_eq!(lic.license(), Ok(String::from("MIT")));
    }

    #[test]
    fn test_package_json_license_not_found() {
        let data = r#"
        {
            "name": "moefy-canvas-monorepo"
        }"#;
        let lic: PackageJson = serde_json::from_str(data).unwrap();
        assert_eq!(lic, PackageJson { license: None });
        assert_eq!(lic.license(), Err(ManifestError::LicenseNotFound));
    }

    #[test]
    fn test_package_json_parse_error() {
        let data = r#"
        {
            "name": "moefy-canvas-monorepo",
            \(^_^)/   <-- parse error
        }"#;
        let lic = PackageJson::from_str(data);
        assert_eq!(lic, Err(ManifestError::ParseError));
    }
}
