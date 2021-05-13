use clap::Clap;

#[derive(Clap, Debug)]
#[clap(author, about, version)]
pub struct Opts {
    #[clap(name = "TIME")]
    pub time: Option<String>,
    /// Provide a terse answer, and default to a verbose form
    #[clap(short, long)]
    pub short: bool,

    /// Name of the config
    #[clap(short, long, name = "NAME", default_value = "belt")]
    pub app: String,

    #[clap(subcommand)]
    pub subcommands: Option<Subcommands>,
}

#[derive(Clap, Debug)]
pub enum Subcommands {
    /// Configure time zones list
    Config(OptsConfig),
}

#[derive(Clap, Debug)]
pub struct OptsConfig {
    /// List existing time zones
    #[clap(short, long)]
    pub list: bool,
    /// Reset to default list of time zones
    #[clap(short, long)]
    pub reset: bool,
    /// Add a new time zone to the list
    #[clap(short, long, name = "timezone_to_add")]
    pub add: Option<String>,
    /// Delete a time zone from the list
    #[clap(short, long, name = "timezone_to_delete")]
    pub delete: Option<String>,
}

impl Opts {
    pub fn new() -> Self {
        Self::parse()
    }
}
