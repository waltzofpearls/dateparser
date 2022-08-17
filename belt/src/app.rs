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
use std::io;

pub struct App<'a, T> {
    pub opts: &'a Opts,
    pub config: &'a mut Config<'a, T>,
}

impl<'a, T> App<'a, T>
where
    T: io::Write,
{
    pub fn new(opts: &'a Opts, config: &'a mut Config<'a, T>) -> Self {
        Self { opts, config }
    }

    pub fn show_datetime(&mut self) -> Result<()> {
        if self.opts.subcommands.is_some() {
            // skip showing datetime when there is a subcommand
            return Ok(());
        }

        let mut to_show = Utc::now();
        if let Some(time) = &self.opts.time {
            to_show = time.parse::<DateTimeUtc>()?.0;
        }

        let local = to_show.with_timezone(&Local);
        let ymd_hms_z = "%Y-%m-%d %H:%M:%S %z";
        let ymd_hm_z = "%Y-%m-%d %H:%M %Z";

        if self.opts.short {
            writeln!(self.config.out, "{}", local.format(ymd_hms_z))?;
        } else {
            let mut table = Table::new();
            table.set_titles(row!["Zone", "Date & Time"]);
            table.add_row(row![
                "Local",
                format!("{}\n{}", local.format(ymd_hms_z), local.format("%s"))
            ]);
            for timezone in &self.config.store.timezones {
                let tz: Tz = timezone.parse().map_err(Error::msg)?;
                let dtz = to_show.with_timezone(&tz);
                table.add_row(row![
                    timezone,
                    format!("{}\n{}", dtz.format(ymd_hms_z), dtz.format(ymd_hm_z))
                ]);
            }
            table.print(&mut self.config.out)?;
        }

        Ok(())
    }

    pub fn handle_subcommands(&mut self) -> Result<()> {
        if let Some(subcommands) = &self.opts.subcommands {
            match subcommands {
                Subcommands::Config(c) => {
                    if c.list {
                        let path = self.config.path();
                        writeln!(self.config.out, "{}", path.cyan().bold())?;
                        self.config.list()?;
                    } else if c.reset {
                        self.config.reset()?;
                        self.config.list()?;
                    } else if let Some(add) = &c.add {
                        self.config.add(add)?;
                        self.config.list()?;
                    } else if let Some(delete) = &c.delete {
                        self.config.delete(delete)?;
                        self.config.list()?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opts::OptsConfig;
    use rand::{thread_rng, Rng};
    use regex::Regex;
    use std::{thread::sleep, time::Duration};

    #[test]
    fn test_app_show_datetime() {
        let mut opts = Opts::new();
        opts.app = "unit-test".to_string();
        let mut buf = vec![0u8];
        let mut config = match Config::new(&opts.app, &mut buf) {
            Ok(config) => config,
            Err(_) => {
                sleep(Duration::from_millis(thread_rng().gen_range(100..500)));
                Config::new(&opts.app, &mut buf).expect("failed to create config")
            }
        };
        let timezones = config.store.timezones.clone();
        let num_timezones = timezones.len();
        let mut app = App::new(&opts, &mut config);

        app.show_datetime().expect("failed showing time");

        let printed = String::from_utf8_lossy(&buf);
        for tz in timezones {
            assert!(printed.contains(&tz));
        }
        let re = Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2} [0-9-+]{5}")
            .expect("failed to parse regex");
        assert_eq!(re.find_iter(&printed).count(), num_timezones + 1); // num_timezones + local
    }

    #[test]
    fn test_app_handle_subcommands() {
        let mut opts = Opts::new();
        opts.app = "unit-test".to_string();
        let mut buf = vec![0u8];
        let mut config = match Config::new(&opts.app, &mut buf) {
            Ok(config) => config,
            Err(_) => {
                sleep(Duration::from_millis(thread_rng().gen_range(100..500)));
                Config::new(&opts.app, &mut buf).expect("failed to create config")
            }
        };
        let timezones = config.store.timezones.clone();
        let mut app = App::new(&opts, &mut config);

        let opts = Opts {
            subcommands: Some(Subcommands::Config(OptsConfig {
                list: true,
                reset: false,
                add: None,
                delete: None,
            })),
            time: None,
            short: false,
            app: opts.app.to_owned(),
        };
        app.opts = &opts;
        app.handle_subcommands()
            .expect("failed handling subcommands");

        let printed = String::from_utf8_lossy(&buf);
        for tz in timezones {
            assert!(printed.contains(&tz));
        }
    }
}
