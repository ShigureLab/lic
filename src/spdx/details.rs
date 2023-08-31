use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LicenseDetails {
    pub is_deprecated_license_id: bool,
    pub license_text: String,
    standard_license_header_template: Option<String>,
    standard_license_template: String,
    name: String,
    license_id: String,
    cross_ref: Vec<CrossRef>,
    see_also: Vec<String>,
    is_osi_approved: bool,
    is_fsf_libre: Option<bool>,
    license_text_html: String,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct CrossRef {
    #[serde(rename = "match")]
    match_: String,
    url: String,
    is_valid: bool,
    is_live: bool,
    timestamp: String,
    is_way_back_link: bool,
    order: u32,
}

pub async fn get_details(url: String) -> Result<LicenseDetails, reqwest::Error> {
    let resp = reqwest::get(url).await?.json::<LicenseDetails>().await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_details_mit() {
        // https://spdx.org/licenses/MIT.json
        let data = r#"
        {
            "isDeprecatedLicenseId": false,
            "isFsfLibre": true,
            "licenseText": "MIT License .. A very long text...",
            "standardLicenseTemplate": "MIT License .. A very long text...",
            "name": "MIT License",
            "licenseId": "MIT",
            "crossRef": [
            {
                "match": "false",
                "url": "https://opensource.org/licenses/MIT",
                "isValid": true,
                "isLive": true,
                "timestamp": "2022-02-06T22:01:14Z",
                "isWayBackLink": false,
                "order": 0
            }
            ],
            "seeAlso": [
                "https://opensource.org/licenses/MIT"
            ],
            "isOsiApproved": true,
            "licenseTextHtml": "A very long text..."
        }"#;
        let lic_details: LicenseDetails = serde_json::from_str(data).unwrap();
        assert_eq!(
            lic_details,
            LicenseDetails {
                is_deprecated_license_id: false,
                license_text: String::from("MIT License .. A very long text..."),
                standard_license_header_template: None,
                standard_license_template: String::from("MIT License .. A very long text..."),
                name: String::from("MIT License"),
                license_id: String::from("MIT"),
                cross_ref: vec![CrossRef {
                    match_: String::from("false"),
                    url: String::from("https://opensource.org/licenses/MIT"),
                    is_valid: true,
                    is_live: true,
                    timestamp: String::from("2022-02-06T22:01:14Z"),
                    is_way_back_link: false,
                    order: 0,
                }],
                see_also: vec![String::from("https://opensource.org/licenses/MIT")],
                is_osi_approved: true,
                is_fsf_libre: Some(true),
                license_text_html: String::from("A very long text..."),
            }
        )
    }

    #[test]
    fn test_cross_ref_mit() {
        // https://spdx.org/licenses/MIT.json
        let data = r#"
        {
            "match": "false",
            "url": "https://opensource.org/licenses/MIT",
            "isValid": true,
            "isLive": true,
            "timestamp": "2022-02-06T22:01:14Z",
            "isWayBackLink": false,
            "order": 0
        }"#;
        let cross_ref: CrossRef = serde_json::from_str(data).unwrap();
        assert_eq!(
            cross_ref,
            CrossRef {
                match_: String::from("false"),
                url: String::from("https://opensource.org/licenses/MIT"),
                is_valid: true,
                is_live: true,
                timestamp: String::from("2022-02-06T22:01:14Z"),
                is_way_back_link: false,
                order: 0,
            }
        )
    }

    #[tokio::test]
    async fn test_get_details() {
        let url = "https://spdx.org/licenses/MIT.json".into();
        assert!(get_details(url).await.is_ok());
    }
}
