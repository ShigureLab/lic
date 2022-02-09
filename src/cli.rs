use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser, Debug, PartialEq)]
#[clap(name = "lic")]
#[clap(author = "Nyakku Shigure <sigure.qaq@gmail.com>")]
#[clap(version = "0.2.1")]
#[clap(about = "A SPDX license generator.")]
#[clap(global_setting(AppSettings::AllowNegativeNumbers))]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    New(OptionsNew),
    Auto(OptionsAuto),
}

#[derive(Parser, Debug, PartialEq)]
pub struct OptionsNew {
    pub name: String,

    #[clap(short, long)]
    pub width: Option<usize>,
}

#[derive(Parser, Debug, PartialEq)]
pub struct OptionsAuto {
    #[clap(short, long)]
    pub width: Option<usize>,

    #[clap(short, long)]
    pub force: bool,
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

    #[test]
    fn test_auto() {
        let cli = Cli::parse_from(&["lic", "auto", "-w", "80"]);
        assert_eq!(
            cli.command,
            Commands::Auto(OptionsAuto {
                width: Some(80),
                force: false
            })
        );
    }
}
