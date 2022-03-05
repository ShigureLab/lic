use clap::{Parser, Subcommand};

#[derive(Parser, Debug, PartialEq)]
#[clap(name = "lic")]
#[clap(author = "Nyakku Shigure <sigure.qaq@gmail.com>")]
#[clap(version = "0.3.0")]
#[clap(about = "A SPDX license generator.")]
#[clap(allow_negative_numbers = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    New(OptionsNew),
    Auto(OptionsAuto),
    Search(OptionsSearch),
}

#[derive(Parser, Debug, PartialEq)]
pub struct OptionsNew {
    pub id: String,

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

#[derive(Parser, Debug, PartialEq)]
pub struct OptionsSearch {
    pub id: String,

    #[clap(short, long, default_value_t = 20)]
    pub number: usize,
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
                id: "MIT".into(),
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

    #[test]
    fn test_search() {
        let cli = Cli::parse_from(&["lic", "search", "gpl", "-n", "50"]);
        assert_eq!(
            cli.command,
            Commands::Search(OptionsSearch {
                id: String::from("gpl"),
                number: 50
            })
        );
    }
}
