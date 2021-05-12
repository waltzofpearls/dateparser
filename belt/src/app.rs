use crate::{
    config::Config,
    opts::{Opts, Subcommands},
};
use anyhow::{Error, Result};
use chrono::prelude::*;
use chrono_tz::Tz;
use colored::*;
use dateparser::DateTimeUtc;
use prettytable::{cell, row, Table};

pub struct App<'a> {
    opts: &'a Opts,
    config: &'a mut Config,
}

impl<'a> App<'a> {
    pub fn new(opts: &'a Opts, config: &'a mut Config) -> Self {
        Self { opts, config }
    }

    pub fn show_datetime(&self) -> Result<()> {
        if self.opts.subcommands.is_some() {
            // skip showing datetime: subcommand given,
            // and it will be handle in another method
            return Ok(());
        }

        let mut to_show = Utc::now();
        if let Some(time) = &self.opts.time {
            to_show = time.parse::<DateTimeUtc>()?.0;
        }

        let mut table = Table::new();
        let local = to_show.with_timezone(&Local);
        let ymd_hms_z = "%Y-%m-%d %H:%M:%S %z";
        let ymd_hm_z = "%Y-%m-%d %H:%M %Z";
        table.set_titles(row!["Zone", "Date & Time"]);
        table.add_row(row![
            "Local",
            format!("{}\n{}", local.format(ymd_hms_z), local.format("%s"))
        ]);
        for timezone in &self.config.timezones {
            let tz: Tz = timezone.parse().map_err(Error::msg)?;
            let dtz = to_show.with_timezone(&tz);
            table.add_row(row![
                timezone,
                format!("{}\n{}", dtz.format(ymd_hms_z), dtz.format(ymd_hm_z))
            ]);
        }
        table.printstd();

        Ok(())
    }

    pub fn handle_subcommands(&mut self) -> Result<()> {
        if let Some(subcommands) = &self.opts.subcommands {
            match subcommands {
                Subcommands::Config(c) => {
                    if c.list {
                        println!(
                            "{}",
                            format!("{}/{}.toml", self.config.path(), self.config.app)
                                .cyan()
                                .bold()
                        );
                        self.config.list()?;
                    } else if c.reset {
                        self.config.reset();
                        self.config.list()?;
                    } else if let Some(add) = &c.add {
                        self.config.add(&add);
                        self.config.list()?;
                    } else if let Some(delete) = &c.delete {
                        self.config.delete(&delete);
                        self.config.list()?;
                    }
                }
            }
        }
        Ok(())
    }
}
