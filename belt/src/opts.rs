use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, about, version)]
pub struct Opts {
    #[arg(name = "TIME")]
    pub time: Option<String>,
    /// Provide a terse answer, and default to a verbose form
    #[arg(short, long)]
    pub short: bool,

    /// Name of the config
    #[arg(short, long, name = "NAME", default_value = "belt")]
    pub app: String,

    #[command(subcommand)]
    pub subcommands: Option<Subcommands>,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    /// Configure time zones list
    Config(OptsConfig),
}

#[derive(Parser, Debug)]
pub struct OptsConfig {
    /// List existing time zones
    #[arg(short, long)]
    pub list: bool,
    /// Reset to default list of time zones
    #[arg(short, long)]
    pub reset: bool,
    /// Add a new time zone to the list
    #[arg(short, long, name = "timezone_to_add")]
    pub add: Option<String>,
    /// Delete a time zone from the list
    #[arg(short, long, name = "timezone_to_delete")]
    pub delete: Option<String>,
}

impl Opts {
    pub fn new() -> Self {
        Self::parse()
    }
}
