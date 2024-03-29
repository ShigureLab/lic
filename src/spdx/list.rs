use super::details::{get_details, LicenseDetails};
use serde::Deserialize;
use strsim::jaro_winkler;

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Licenses {
    pub license_list_version: String,
    pub licenses: Vec<License>,
    pub release_date: String,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct License {
    reference: String,
    pub is_deprecated_license_id: bool,
    details_url: String,
    reference_number: u32,
    pub name: String,
    pub license_id: String,
    see_also: Vec<String>,
    pub is_osi_approved: bool,
    is_fsf_libre: Option<bool>,
}

pub async fn get_licenses() -> Result<Licenses, reqwest::Error> {
    let url = "https://spdx.org/licenses/licenses.json".to_string();
    let resp = reqwest::get(url).await?.json::<Licenses>().await?;
    Ok(resp)
}

impl Licenses {
    #[allow(dead_code)]
    pub fn contains(&self, id: &str) -> bool {
        self.licenses
            .iter()
            .map(|license| license.license_id.clone())
            .any(|x| x == *id)
    }

    #[allow(dead_code)]
    pub fn contains_case_insensitive(&self, id: &str) -> bool {
        self.licenses
            .iter()
            .map(|license| license.license_id.clone().to_lowercase())
            .any(|x| x == id.to_lowercase())
    }

    pub fn similar_licenses(&self, id: &str, num: usize) -> Vec<License> {
        let mut license_with_similarity: Vec<_> = self
            .licenses
            .iter()
            .map(|license| {
                (
                    jaro_winkler(
                        &license.license_id.clone().to_lowercase(),
                        &id.to_lowercase(),
                    ),
                    license,
                )
            })
            .collect();
        license_with_similarity.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        license_with_similarity[0..num]
            .iter()
            .map(|(_, license)| license.to_owned().clone())
            .collect()
    }

    #[allow(dead_code)]
    pub fn get_license(&self, id: &str) -> Option<&License> {
        self.licenses
            .iter()
            .find(|&license| license.license_id == id)
    }

    pub fn get_license_case_insensitive(&self, id: &str) -> Option<&License> {
        self.licenses
            .iter()
            .find(|&license| license.license_id.to_lowercase() == id.to_lowercase())
    }
}

impl License {
    pub async fn get_details(&self) -> Result<LicenseDetails, reqwest::Error> {
        get_details(self.details_url.clone()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_lgpl_3_0() {
        let data = r#"
        {
            "reference": "https://spdx.org/licenses/LGPL-3.0.html",
            "isDeprecatedLicenseId": true,
            "detailsUrl": "https://spdx.org/licenses/LGPL-3.0.json",
            "referenceNumber": 483,
            "name": "GNU Lesser General Public License v3.0 only",
            "licenseId": "LGPL-3.0",
            "seeAlso": [
                "https://www.gnu.org/licenses/lgpl-3.0-standalone.html",
                "https://opensource.org/licenses/LGPL-3.0"
            ],
            "isOsiApproved": true,
            "isFsfLibre": true
        }"#;
        let lic: License = serde_json::from_str(data).unwrap();
        assert_eq!(
            lic,
            License {
                reference: String::from("https://spdx.org/licenses/LGPL-3.0.html"),
                is_deprecated_license_id: true,
                details_url: String::from("https://spdx.org/licenses/LGPL-3.0.json"),
                reference_number: 483,
                name: String::from("GNU Lesser General Public License v3.0 only"),
                license_id: String::from("LGPL-3.0"),
                see_also: vec![
                    String::from("https://www.gnu.org/licenses/lgpl-3.0-standalone.html"),
                    String::from("https://opensource.org/licenses/LGPL-3.0")
                ],
                is_osi_approved: true,
                is_fsf_libre: Some(true),
            }
        )
    }

    #[test]
    fn test_license_cc_by_nc_sa_4_0() {
        let data = r#"
        {
            "reference": "https://spdx.org/licenses/CC-BY-NC-SA-4.0.html",
            "isDeprecatedLicenseId": false,
            "detailsUrl": "https://spdx.org/licenses/CC-BY-NC-SA-4.0.json",
            "referenceNumber": 97,
            "name": "Creative Commons Attribution Non Commercial Share Alike 4.0 International",
            "licenseId": "CC-BY-NC-SA-4.0",
            "seeAlso": [
                "https://creativecommons.org/licenses/by-nc-sa/4.0/legalcode"
            ],
            "isOsiApproved": false
        }"#;
        let lic: License = serde_json::from_str(data).unwrap();
        assert_eq!(
            lic,
            License {
                reference: String::from("https://spdx.org/licenses/CC-BY-NC-SA-4.0.html"),
                is_deprecated_license_id: false,
                details_url: String::from("https://spdx.org/licenses/CC-BY-NC-SA-4.0.json"),
                reference_number: 97,
                name: String::from(
                    "Creative Commons Attribution Non Commercial Share Alike 4.0 International"
                ),
                license_id: String::from("CC-BY-NC-SA-4.0"),
                see_also: vec![String::from(
                    "https://creativecommons.org/licenses/by-nc-sa/4.0/legalcode"
                ),],
                is_osi_approved: false,
                is_fsf_libre: None,
            }
        )
    }

    #[test]
    fn test_licenses() {
        let data = r#"
        {
            "licenseListVersion": "3.16",
            "licenses": [
                {
                    "reference": "https://spdx.org/licenses/CC-BY-NC-SA-4.0.html",
                    "isDeprecatedLicenseId": false,
                    "detailsUrl": "https://spdx.org/licenses/CC-BY-NC-SA-4.0.json",
                    "referenceNumber": 97,
                    "name": "Creative Commons Attribution Non Commercial Share Alike 4.0 International",
                    "licenseId": "CC-BY-NC-SA-4.0",
                    "seeAlso": [
                        "https://creativecommons.org/licenses/by-nc-sa/4.0/legalcode"
                    ],
                    "isOsiApproved": false
                }
            ],
            "releaseDate": "2022-02-06"
        }"#;
        let lic: Licenses = serde_json::from_str(data).unwrap();
        assert_eq!(
            lic,
            Licenses {
                license_list_version: String::from("3.16"),
                licenses: vec![License {
                    reference: String::from("https://spdx.org/licenses/CC-BY-NC-SA-4.0.html"),
                    is_deprecated_license_id: false,
                    details_url: String::from("https://spdx.org/licenses/CC-BY-NC-SA-4.0.json"),
                    reference_number: 97,
                    name: String::from(
                        "Creative Commons Attribution Non Commercial Share Alike 4.0 International"
                    ),
                    license_id: String::from("CC-BY-NC-SA-4.0"),
                    see_also: vec![String::from(
                        "https://creativecommons.org/licenses/by-nc-sa/4.0/legalcode"
                    ),],
                    is_osi_approved: false,
                    is_fsf_libre: None,
                }],
                release_date: String::from("2022-02-06")
            }
        )
    }

    #[tokio::test]
    async fn test_get_licenses() {
        assert!(get_licenses().await.is_ok());
    }
}
