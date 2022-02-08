use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser, Debug, PartialEq)]
#[clap(name = "lic")]
#[clap(author = "Nyakku Shigure <sigure.qaq@gmail.com>")]
#[clap(version = "0.1.1")]
#[clap(about = "A SPDX license generator.")]
#[clap(global_setting(AppSettings::AllowNegativeNumbers))]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    New(OptionsNew),
}

#[derive(Parser, Debug, PartialEq)]
pub struct OptionsNew {
    pub name: String,

    #[clap(short, long)]
    pub width: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cli = Cli::parse_from(&["lic", "new", "MIT"]);
        assert_eq!(
            cli.command,
            Commands::New(OptionsNew {
                name: "MIT".into(),
                width: None
            })
        );
    }
}
