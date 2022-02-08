mod cli;
mod spdx;

use clap::Parser;
use cli::{Cli, Commands};
use colored::*;
use spdx::list::get_licenses;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let licenses = get_licenses().await?;
    let cli = Cli::parse();

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
                eprintln!("Unknown license id: {}.", options.name.blue().bold());
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
    }

    Ok(())
}
