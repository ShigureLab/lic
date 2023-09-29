mod cli;
mod manifest;
mod spdx;

use clap::Parser;
use cli::{Cli, Commands};
use colored::*;
use manifest::{CargoToml, Manifest, ManifestError, PackageJson, PyprojectToml};
use spdx::list::get_licenses;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let licenses = get_licenses().await?;
    let cli = Cli::parse();
    let badge_error = " ERROR ".black().on_red().bold();
    let badge_warning = " WARN ".black().on_yellow();

    match cli.command {
        Commands::New(options) => match licenses.get_license_case_insensitive(&options.id) {
            Some(lic) => {
                let mut text = lic.get_details().await?.license_text;
                if let Some(max_width) = options.width {
                    text = textwrap::fill(&text, max_width)
                }
                if lic.is_deprecated_license_id {
                    eprintln!("{badge_warning} This license id has been deprecated.");
                }
                print!("{}", text);
            }
            None => {
                eprintln!(
                    "{badge_error} Unknown license id: {}.",
                    options.id.blue().bold()
                );
                eprintln!(
                    "Did you mean {}?",
                    licenses
                        .similar_licenses(&options.id, 5)
                        .iter()
                        .map(|license| if license.is_deprecated_license_id {
                            license.license_id.yellow().to_string()
                        } else {
                            license.license_id.green().bold().to_string()
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        },
        Commands::Auto(options) => {
            let lic = if let Some(text) = CargoToml::read() {
                let manifest = CargoToml::from_str(&text);
                manifest.and_then(|manifest| manifest.license())
            } else if let Some(text) = PyprojectToml::read() {
                let manifest = PyprojectToml::from_str(&text);
                manifest.and_then(|manifest| manifest.license())
            } else if let Some(text) = PackageJson::read() {
                let manifest = PackageJson::from_str(&text);
                manifest.and_then(|manifest| manifest.license())
            } else {
                Err(ManifestError::FileNotFound)
            };

            match lic {
                Ok(lic) => match licenses.get_license_case_insensitive(&lic) {
                    Some(lic) => {
                        let mut text = lic.get_details().await?.license_text;
                        if let Some(max_width) = options.width {
                            text = textwrap::fill(&text, max_width)
                        }
                        let license_path = Path::new("LICENSE");
                        if license_path.exists() && !options.force {
                            println!("{badge_warning} The file LICENSE already exists, you may need the {} option to force an override.", "--force".blue())
                        } else {
                            let mut file = File::create(license_path)?;
                            file.write_all(text.as_bytes())?;
                        }
                    }
                    None => println!("{badge_error} Unknown license id: {}.", lic.blue().bold()),
                },
                Err(e) => match e {
                    ManifestError::ParseError => {
                        println!("{badge_error} Cannot parse the manifest file.")
                    }
                    ManifestError::LicenseNotFound => {
                        println!("{badge_error} Cannot find the license id in the manifest file.")
                    }
                    ManifestError::FileNotFound => {
                        println!("{badge_error} Cannot find the manifest file.")
                    }
                },
            }
        }
        Commands::Search(options) => {
            let similar_licenses = licenses.similar_licenses(&options.id, options.number);
            let license_id_width = similar_licenses
                .iter()
                .map(|lic| lic.license_id.len())
                .max()
                .unwrap()
                + 2;
            let license_name_width = similar_licenses
                .iter()
                .map(|lic| lic.name.len())
                .max()
                .unwrap()
                + 2;
            println!(
                "{:^license_id_width$} {:^license_name_width$} {:^12} {:^12}",
                "License id".green(),
                "License name".blue(),
                "Deprecated".yellow(),
                "OSI Approved".purple(),
                license_id_width = license_id_width,
                license_name_width = license_name_width,
            );
            for license in similar_licenses {
                println!(
                    "{:license_id_width$} {:license_name_width$} {:^12} {:^12}",
                    license.license_id.green(),
                    license.name.blue(),
                    if license.is_deprecated_license_id {
                        "✔"
                    } else {
                        ""
                    },
                    if license.is_osi_approved { "✔" } else { "" },
                    license_id_width = license_id_width,
                    license_name_width = license_name_width,
                );
            }
        }
    }

    Ok(())
}
