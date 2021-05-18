mod app;
mod config;
mod opts;

use crate::{app::App, config::Config, opts::Opts};
use anyhow::Result;

fn main() -> Result<()> {
    let opts = Opts::new();
    let mut out = std::io::stdout();
    let mut config = Config::new(&opts.app, &mut out)?;
    let mut app = App::new(&opts, &mut config);

    app.show_datetime()?;
    app.handle_subcommands()?;

    Ok(())
}
