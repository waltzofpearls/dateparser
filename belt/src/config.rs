use anyhow::{Error, Result};
use chrono::prelude::*;
use chrono_tz::{OffsetComponents, OffsetName, Tz};
use colored::*;
use directories::ProjectDirs;
use prettytable::{cell, row, Table};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub timezones: Vec<String>,
    pub app: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timezones: vec![
                "UTC".to_string(),
                "America/Vancouver".to_string(),
                "America/New_York".to_string(),
                "Europe/London".to_string(),
            ],
            app: "belt".to_string(),
        }
    }
}

impl Config {
    pub fn new(app: &str) -> Result<Self> {
        let mut config: Config = confy::load(app)?;
        config.app = app.to_string();
        Ok(config)
    }

    pub fn path(&self) -> String {
        ProjectDirs::from("rs", "", &self.app)
            .and_then(|project| project.config_dir().to_str().map(|s: &str| s.to_string()))
            .unwrap_or_else(|| "".to_string())
    }

    pub fn list(&self) -> Result<()> {
        let now_utc = Local::now().naive_utc();
        let mut table = Table::new();
        table.set_titles(row!["Zone", "Abbr.", "Offset"]);
        for timezone in &self.timezones {
            let tz: Tz = timezone.parse().map_err(Error::msg)?;
            let offset = tz.offset_from_utc_datetime(&now_utc);
            table.add_row(row![
                timezone,
                offset.abbreviation(),
                match offset.base_utc_offset().num_hours() {
                    0 => "0 hour".to_string(),
                    hours => format!("{} hours", hours),
                }
            ]);
        }
        table.printstd();
        Ok(())
    }

    pub fn add(&mut self, to_add: &str) {
        match to_add.parse::<Tz>().and_then(|_| {
            self.timezones.push(to_add.to_string());
            confy::store(&self.app, &self).map_err(|err| format!("{}", err))
        }) {
            Ok(_) => println!(
                "{}",
                format!("Added '{}' to config.", to_add).green().bold()
            ),
            Err(err) => println!(
                "{}",
                format!("Could not add time zone: {}.", err).red().bold()
            ),
        };
    }

    pub fn delete(&mut self, to_delete: &str) {
        self.timezones.retain(|tz| tz != to_delete);
        match confy::store(&self.app, &self) {
            Ok(_) => println!(
                "{}",
                format!("Deleted '{}' from config.", to_delete)
                    .green()
                    .bold()
            ),
            Err(err) => println!(
                "{}",
                format!("Could not delete time zone: {}.", err).red().bold()
            ),
        }
    }

    pub fn reset(&self) {
        match confy::store(&self.app, Config::default()) {
            Ok(_) => println!("{}", "Config has been reset to default.".green().bold()),
            Err(err) => println!(
                "{}",
                format!("Could not reset time zones: {}", err).red().bold()
            ),
        }
    }
}
