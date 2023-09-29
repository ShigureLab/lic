use super::base::{Manifest, ManifestError};
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PyprojectToml {
    // https://peps.python.org/pep-0621/
    // https://packaging.python.org/en/latest/specifications/declaring-project-metadata/#declaring-project-metadata
    project: Option<Project>,
    tool: Option<Tool>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    license: Option<License>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct License {
    file: Option<String>,
    text: Option<String>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    poetry: Option<Poetry>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Poetry {
    license: Option<String>,
}

impl Manifest for PyprojectToml {
    fn license(self) -> Result<String, ManifestError> {
        self.project
            .and_then(|project| project.license)
            .and_then(|license| license.text)
            .or_else(|| {
                self.tool
                    .and_then(|tool| tool.poetry)
                    .and_then(|poetry| poetry.license)
            })
            .ok_or(ManifestError::LicenseNotFound)
    }

    fn filename() -> String {
        "pyproject.toml".into()
    }

    fn from_str(text: &str) -> Result<Self, ManifestError> {
        toml::from_str(text).map_err(|_| ManifestError::ParseError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pyproject_toml_poetry() {
        // https://github.com/SigureMo/yutto/blob/main/pyproject.toml
        let data = r#"
        [tool.poetry]
        name = "yutto"
        version = "2.0.0-beta.9"
        description = "🧊 一个可爱且任性的 B 站视频下载器"
        authors = ["Nyakku Shigure <sigure.qaq@gmail.com>"]
        license = "GPL-3.0"
        readme = "README.md"
        homepage = "https://github.com/SigureMo/yutto"
        repository = "https://github.com/SigureMo/yutto"
        keywords = ["python", "bilibili", "video", "downloader", "danmaku"]
        classifiers = [
            "Environment :: Console",
            "Operating System :: OS Independent",
            "License :: OSI Approved :: GNU General Public License v3 (GPLv3)",
            "Programming Language :: Python",
            "Programming Language :: Python :: 3",
            "Programming Language :: Python :: 3.9",
            "Programming Language :: Python :: 3.10",
            "Programming Language :: Python :: Implementation :: CPython",
        ]"#;
        let lic: PyprojectToml = toml::from_str(data).unwrap();
        assert_eq!(
            lic,
            PyprojectToml {
                project: None,
                tool: Some(Tool {
                    poetry: Some(Poetry {
                        license: Some(String::from("GPL-3.0"))
                    })
                })
            }
        );
        assert_eq!(lic.license(), Ok(String::from("GPL-3.0")));
    }

    #[test]
    fn test_pyproject_toml_pep621() {
        // https://github.com/yutto-dev/bilili/blob/main/pyproject.toml
        let data = r#"
        [project]
        name = "bilili"
        description = "🍻 bilibili video and danmaku downloader | B站视频、弹幕下载器"
        readme = "README.md"
        requires-python = ">=3.8"
        authors = [{ name = "Nyakku Shigure", email = "sigure.qaq@gmail.com" }]
        keywords = ["python", "bilibili", "video", "download", "spider", "danmaku"]
        license = { text = "GPLv3" }
        classifiers = [
        "Environment :: Console",
        "Operating System :: OS Independent",
        "License :: OSI Approved :: GNU General Public License v3 (GPLv3)",
        "Programming Language :: Python",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: Implementation :: CPython",
        ]
        dependencies = ["requests", "biliass==1.3.7"]
        dynamic = ["version"]
        "#;
        let lic: PyprojectToml = toml::from_str(data).unwrap();
        assert_eq!(
            lic,
            PyprojectToml {
                project: Some(Project {
                    license: Some(License {
                        file: None,
                        text: Some(String::from("GPLv3"))
                    })
                }),
                tool: None
            }
        );
        assert_eq!(lic.license(), Ok(String::from("GPLv3")));
    }

    #[test]
    fn test_pyproject_toml_license_not_found() {
        let data = r#"
        [project]
        name = "bilili"
        "#;
        let lic: PyprojectToml = toml::from_str(data).unwrap();
        assert_eq!(
            lic,
            PyprojectToml {
                project: Some(Project { license: None }),
                tool: None
            }
        );
        assert_eq!(lic.license(), Err(ManifestError::LicenseNotFound));
    }

    #[test]
    fn test_pyproject_toml_parse_error() {
        let data = r#"
        [project]
        name = "bilili"
        \(^_^)/   <-- parse error
        "#;
        let lic = PyprojectToml::from_str(data);
        assert_eq!(lic, Err(ManifestError::ParseError));
    }
}
