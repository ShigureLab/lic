use super::base::Manifest;
use serde::Deserialize;
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    // https://docs.npmjs.com/cli/v7/configuring-npm/package-json
    license: String,
}

impl Manifest for PackageJson {
    fn license(self) -> String {
        self.license
    }

    fn filename() -> String {
        "package.json".into()
    }

    fn from_str(text: &str) -> Self {
        serde_json::from_str(text).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_json() {
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
                license: String::from("MIT")
            }
        )
    }
}
