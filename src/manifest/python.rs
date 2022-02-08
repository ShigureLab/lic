use super::base::Manifest;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PyprojectToml {
    // https://docs.npmjs.com/cli/v7/configuring-npm/package-json
    tool: Tool,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    poetry: Poetry,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Poetry {
    license: String,
}

impl Manifest for PyprojectToml {
    fn license(self) -> String {
        self.tool.poetry.license
    }

    fn filename() -> String {
        "pyproject.toml".into()
    }

    fn from_str(text: &str) -> Self {
        toml::from_str(text).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pyproject_toml() {
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
                tool: Tool {
                    poetry: Poetry {
                        license: String::from("GPL-3.0")
                    }
                }
            }
        )
    }
}
