use anyhow::{Error, Result};
use chrono::prelude::*;
use chrono_tz::{OffsetComponents, OffsetName, Tz};
use colored::*;
use directories::ProjectDirs;
use prettytable::{row, Table};
use serde::{Deserialize, Serialize};
use std::io;

pub struct Config<'a, T> {
    pub store: Store,
    pub out: &'a mut T,
    pub app: String,
}

#[derive(Serialize, Deserialize)]
pub struct Store {
    pub timezones: Vec<String>,
}

impl ::std::default::Default for Store {
    fn default() -> Self {
        Self {
            timezones: vec![
                "UTC".to_string(),
                "America/Vancouver".to_string(),
                "America/New_York".to_string(),
                "Europe/London".to_string(),
            ],
        }
    }
}

impl<'a, T> Config<'a, T>
where
    T: io::Write,
{
    pub fn new(app: &str, out: &'a mut T) -> Result<Self> {
        let store: Store = confy::load(app, None)?;
        Ok(Self {
            store,
            out,
            app: app.to_string(),
        })
    }

    pub fn path(&self) -> String {
        ProjectDirs::from("rs", "", &self.app)
            .and_then(|project| project.config_dir().to_str().map(|s: &str| s.to_string()))
            .map(|s| format!("{}/{}.toml", s, self.app))
            .unwrap_or_default()
    }

    pub fn list(&mut self) -> Result<()> {
        let now_utc = Local::now().naive_utc();
        let mut table = Table::new();
        table.set_titles(row![l -> "Zone", l -> "Abbr.", r -> "Offset"]);
        for timezone in &self.store.timezones {
            let tz: Tz = timezone.parse().map_err(Error::msg)?;
            let offset = tz.offset_from_utc_datetime(&now_utc);
            table.add_row(row![
                l -> timezone,
                l -> offset.abbreviation(),
                r -> match offset.base_utc_offset().num_hours() {
                    0 => "0 hour ".to_string(),
                    hours => format!("{} hours", hours),
                }
            ]);
        }
        table.print(self.out)?;
        Ok(())
    }

    pub fn add(&mut self, to_add: &str) -> Result<()> {
        let result = to_add.parse::<Tz>().and_then(|_| {
            self.store.timezones.push(to_add.to_string());
            confy::store(&self.app, None, &self.store).map_err(|err| format!("{}", err))
        });

        match result {
            Ok(_) => writeln!(
                self.out,
                "{}",
                format!("Added '{}' to config.", to_add).green().bold()
            )?,
            Err(err) => writeln!(
                self.out,
                "{}",
                format!("Could not add time zone: {}.", err).red().bold()
            )?,
        };
        Ok(())
    }

    pub fn delete(&mut self, to_delete: &str) -> Result<()> {
        self.store.timezones.retain(|tz| tz != to_delete);
        match confy::store(&self.app, None, &self.store) {
            Ok(_) => writeln!(
                self.out,
                "{}",
                format!("Deleted '{}' from config.", to_delete)
                    .green()
                    .bold()
            )?,
            Err(err) => writeln!(
                self.out,
                "{}",
                format!("Could not delete time zone: {}.", err).red().bold()
            )?,
        };
        Ok(())
    }

    pub fn reset(&mut self) -> Result<()> {
        self.store.timezones = Store::default().timezones;
        match confy::store(&self.app, None, &self.store) {
            Ok(_) => writeln!(
                self.out,
                "{}",
                "Config has been reset to default.".green().bold()
            )?,
            Err(err) => writeln!(
                self.out,
                "{}",
                format!("Could not reset time zones: {}", err).red().bold()
            )?,
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};
    use std::{thread::sleep, time::Duration};

    #[test]
    fn test_config_path() {
        let mut buf = vec![0u8];
        let app = "unit-test";
        let config = match Config::new(app, &mut buf) {
            Ok(config) => config,
            Err(_) => {
                sleep(Duration::from_millis(thread_rng().gen_range(100..500)));
                Config::new(app, &mut buf).expect("failed to create config")
            }
        };
        let path = config.path();
        if !path.contains(app) {
            panic!("path [{}] does not contain [unit-test]", path);
        }
    }

    #[test]
    fn test_config_list() {
        let mut buf = vec![0u8];
        let app = "unit-test";
        let mut config = match Config::new(app, &mut buf) {
            Ok(config) => config,
            Err(_) => {
                sleep(Duration::from_millis(thread_rng().gen_range(100..500)));
                Config::new(app, &mut buf).expect("failed to create config")
            }
        };
        config.reset().expect("failed to reset config store");
        config.out.clear();

        config.list().expect("failed to list configured timezons");
        let listed = String::from_utf8_lossy(&buf);
        for tz in Store::default().timezones {
            assert!(listed.contains(&tz));
        }
    }

    #[test]
    fn test_config_add() {
        let mut buf = vec![0u8];
        let app = "unit-test";
        let mut config = match Config::new(app, &mut buf) {
            Ok(config) => config,
            Err(_) => {
                sleep(Duration::from_millis(thread_rng().gen_range(100..500)));
                Config::new(app, &mut buf).expect("failed to create config")
            }
        };
        config.reset().expect("failed to reset config store");
        config
            .add("Europe/Berlin")
            .expect("failed to add Europe/Berlin");
        config.out.clear();

        config.list().expect("failed to list configured timezons");
        let listed = String::from_utf8_lossy(&buf);
        assert!(listed.contains("Europe/Berlin"));
    }

    #[test]
    fn test_config_delete() {
        let mut buf = vec![0u8];
        let app = "unit-test";
        let mut config = match Config::new(app, &mut buf) {
            Ok(config) => config,
            Err(_) => {
                sleep(Duration::from_millis(thread_rng().gen_range(100..500)));
                Config::new(app, &mut buf).expect("failed to create config")
            }
        };
        config.reset().expect("failed to reset config store");
        config.delete("UTC").expect("failed to delete UTC");
        config.out.clear();

        config.list().expect("failed to list configured timezons");
        let listed = String::from_utf8_lossy(&buf);
        assert!(!listed.contains("UTC"));
    }

    #[test]
    fn test_config_reset() {
        let mut buf = vec![0u8];
        let app = "unit-test";
        let mut config = match Config::new(app, &mut buf) {
            Ok(config) => config,
            Err(_) => {
                sleep(Duration::from_millis(thread_rng().gen_range(100..500)));
                Config::new(app, &mut buf).expect("failed to create config")
            }
        };
        config.reset().expect("failed to reset config store");
        config
            .add("Europe/Berlin")
            .expect("failed to add Europe/Berlin");
        config.delete("UTC").expect("failed to delete UTC");
        config.reset().expect("failed to reset config store");
        config.out.clear();

        config.list().expect("failed to list configured timezons");
        let listed = String::from_utf8_lossy(&buf);
        for tz in Store::default().timezones {
            assert!(listed.contains(&tz));
        }
        assert!(!listed.contains("Europe/Berlin"));
    }
}
