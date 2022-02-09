mod cli;
mod manifest;
mod spdx;

use clap::Parser;
use cli::{Cli, Commands};
use colored::*;
use manifest::{CargoToml, Manifest, PackageJson, PyprojectToml};
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
        Commands::New(options) => match licenses.get_license_case_insensitive(&options.name) {
            Some(lic) => {
                let mut text = lic.get_details().await?.license_text;
                if let Some(max_width) = options.width {
                    text = textwrap::fill(&text, max_width)
                }
                print!("{}", text);
            }
            None => {
                eprintln!(
                    "{badge_error} Unknown license id: {}.",
                    options.name.blue().bold()
                );
                eprintln!(
                    "Did you mean {}?",
                    licenses
                        .similar_licenses_id(&options.name, 5)
                        .iter()
                        .map(|id| id.green().bold().to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        },
        Commands::Auto(options) => {
            let lic = if let Some(text) = CargoToml::read() {
                let manifest = CargoToml::from_str(&text);
                Some(manifest.license())
            } else if let Some(text) = PyprojectToml::read() {
                let manifest = PyprojectToml::from_str(&text);
                Some(manifest.license())
            } else if let Some(text) = PackageJson::read() {
                let manifest = PackageJson::from_str(&text);
                Some(manifest.license())
            } else {
                None
            };

            match lic {
                Some(lic) => match licenses.get_license_case_insensitive(&lic) {
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
                None => println!("{badge_error} Cannot find the manifest file."),
            }
        }
    }

    Ok(())
}
