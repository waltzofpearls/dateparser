//! A rust library for parsing date strings in commonly used formats. Parsed date will be returned
//! as `chrono`'s `DateTime<Utc>`.
//!
//! # Quick Start
//!
//! ```
//! use chrono::prelude::*;
//! use dateparser::parse;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let parsed = parse("6:15pm UTC")?;
//!     let utc_now = Local::now().with_timezone(&Utc);
//!
//!     assert_eq!(
//!         parsed.format("%Y-%m-%d %H:%M:%S %z").to_string(),
//!         format!("{} 18:15:00 +0000", utc_now.format("%Y-%m-%d"))
//!     );
//!
//!     Ok(())
//! }
//! ```
//!
//! Use `str`'s `parse` method:
//!
//! ```
//! use dateparser::DateTimeUtc;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let parsed = "2021-05-14 18:51 PDT".parse::<DateTimeUtc>()?.0;
//!
//!     assert_eq!(parsed.format("%Y-%m-%d %H:%M:%S %z").to_string(), "2021-05-15 01:51:00 +0000");
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Accepted date formats
//!
//! ```
//! use dateparser::DateTimeUtc;
//!
//! let accepted = vec![
//!     "1511648546",
//!     "1620021848429",
//!     "1620024872717915000",
//!     "2021-05-01T01:17:02.604456Z",
//!     "2017-11-25T22:34:50Z",
//!     "Wed, 02 Jun 2021 06:31:39 GMT",
//!     "2019-11-29 08:08:05-08",
//!     "2021-05-02 23:31:36.0741-07",
//!     "2021-05-02 23:31:39.12689-07",
//!     "2019-11-29 08:15:47.624504-08",
//!     "2021-04-30 21:14:10",
//!     "2021-04-30 21:14:10.052282",
//!     "2017-11-25 13:31:15 PST",
//!     "2017-11-25 13:31 PST",
//!     "2021-02-21",
//!     "2021-02-21 PST",
//!     "01:06:06",
//!     "4:00pm",
//!     "6:00 AM",
//!     "01:06:06 PST",
//!     "4:00pm PST",
//!     "6:00 AM PST",
//!     "May 02, 2021 15:51:31 UTC",
//!     "May 02, 2021 15:51 UTC",
//! ];
//!
//! for date_str in accepted {
//!     let result = date_str.parse::<DateTimeUtc>();
//!     assert!(result.is_ok())
//! }
//! ```

use anyhow::{anyhow, Error, Result};
use chrono::prelude::*;
use regex::Regex;

/// DateTimeUtc is an alias for `chrono`'s `DateTime<UTC>`. It implements `std::str::FromStr`'s
/// `from_str` method, and it makes `str`'s `parse` method to understand the accepted date formats
/// from this crate.
///
/// ```
/// use dateparser::DateTimeUtc;
///
/// let accepted = vec![
///     "1511648546",
///     "1620021848429",
///     "1620024872717915000",
///     "2021-05-01T01:17:02.604456Z",
///     "2017-11-25T22:34:50Z",
///     "Wed, 02 Jun 2021 06:31:39 GMT",
///     "2019-11-29 08:08:05-08",
///     "2021-05-02 23:31:36.0741-07",
///     "2021-05-02 23:31:39.12689-07",
///     "2019-11-29 08:15:47.624504-08",
///     "2021-04-30 21:14:10",
///     "2021-04-30 21:14:10.052282",
///     "2017-11-25 13:31:15 PST",
///     "2017-11-25 13:31 PST",
///     "2021-02-21",
///     "2021-02-21 PST",
///     "01:06:06",
///     "4:00pm",
///     "6:00 AM",
///     "01:06:06 PST",
///     "4:00pm PST",
///     "6:00 AM PST",
///     "May 02, 2021 15:51:31 UTC",
///     "May 02, 2021 15:51 UTC",
/// ];
///
/// for date_str in accepted {
///     // parsed is DateTimeUTC and parsed.0 is chrono's DateTime<Utc>
///     match date_str.parse::<DateTimeUtc>() {
///         Ok(parsed) => println!("PARSED {} into UTC datetime {:?}", date_str, parsed.0),
///         Err(err) => println!("ERROR from parsing {}: {}", date_str, err)
///     }
/// }
/// ```
pub struct DateTimeUtc(pub DateTime<Utc>);

impl std::str::FromStr for DateTimeUtc {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        parse(s).map(DateTimeUtc)
    }
}

/// parse tries to interpret the input date and/or time string with a list of parsing functions.
/// Each function can understand a specific date format. When all options are exhausted, parse will
/// return an error to let the caller know that no formats were matched.
///
/// ```
/// use dateparser::parse;
/// use chrono_tz::US::Pacific;
///
/// let parsed = parse("6:15pm").unwrap();
///
/// // print out parsed datetime in UTC
/// println!("{:?}", parsed);
///
/// // print parsed datetime in pacific time
/// println!("{:?}", parsed.with_timezone(&Pacific));
/// ```
pub fn parse(input: &str) -> Result<DateTime<Utc>> {
    parse_unix_timestamp(input)
        .or_else(|| parse_unix_timestamp_millis(input))
        .or_else(|| parse_unix_timestamp_nanos(input))
        .or_else(|| parse_rfc3339(input))
        .or_else(|| parse_rfc2822(input))
        .or_else(|| parse_postgres_timestamp(input))
        .or_else(|| parse_postgres_timestamp_nanos(input))
        .or_else(|| parse_ymd_hms(input))
        .or_else(|| parse_ymd_hms_nanos(input))
        .or_else(|| parse_ymd_hms_z(input))
        .or_else(|| parse_ymd(input))
        .or_else(|| parse_ymd_z(input))
        .or_else(|| parse_hms_imp(input))
        .or_else(|| parse_hms_imp_z(input))
        .or_else(|| parse_bey_hms_z(input))
        .unwrap_or_else(|| Err(anyhow!("{} did not match any formats.", input)))
}

// 1511648546
fn parse_unix_timestamp(input: &str) -> Option<Result<DateTime<Utc>>> {
    match Regex::new(r"^[0-9]{10}$").map_err(Error::msg) {
        Ok(re) => {
            if re.is_match(input) {
                return input
                    .parse::<i64>()
                    .ok()
                    .map(|timestamp| Utc.timestamp(timestamp, 0).with_timezone(&Utc))
                    .map(Ok);
            }
            None
        }
        Err(err) => Some(Err(err)),
    }
}

// 1620021848429
fn parse_unix_timestamp_millis(input: &str) -> Option<Result<DateTime<Utc>>> {
    match Regex::new(r"^[0-9]{13}$").map_err(Error::msg) {
        Ok(re) => {
            if re.is_match(input) {
                return input
                    .parse::<i64>()
                    .ok()
                    .map(|timestamp| Utc.timestamp_millis(timestamp).with_timezone(&Utc))
                    .map(Ok);
            }
            None
        }
        Err(err) => Some(Err(err)),
    }
}

// 1620024872717915000
fn parse_unix_timestamp_nanos(input: &str) -> Option<Result<DateTime<Utc>>> {
    match Regex::new(r"^[0-9]{19}$").map_err(Error::msg) {
        Ok(re) => {
            if re.is_match(input) {
                return input
                    .parse::<i64>()
                    .ok()
                    .map(|timestamp| Utc.timestamp_nanos(timestamp).with_timezone(&Utc))
                    .map(Ok);
            }
            None
        }
        Err(err) => Some(Err(err)),
    }
}

// 2021-05-01T01:17:02.604456Z
// 2017-11-25T22:34:50Z
fn parse_rfc3339(input: &str) -> Option<Result<DateTime<Utc>>> {
    DateTime::parse_from_rfc3339(input)
        .ok()
        .map(|parsed| parsed.with_timezone(&Utc))
        .map(Ok)
}

// Wed, 02 Jun 2021 06:31:39 GMT
fn parse_rfc2822(input: &str) -> Option<Result<DateTime<Utc>>> {
    DateTime::parse_from_rfc2822(input)
        .ok()
        .map(|parsed| parsed.with_timezone(&Utc))
        .map(Ok)
}

// 2019-11-29 08:08:05-08
fn parse_postgres_timestamp(input: &str) -> Option<Result<DateTime<Utc>>> {
    DateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S%#z")
        .ok()
        .map(|parsed| parsed.with_timezone(&Utc))
        .map(Ok)
}

// 2021-05-02 23:31:36.0741-07
// 2021-05-02 23:31:39.12689-07
// 2019-11-29 08:15:47.624504-08
fn parse_postgres_timestamp_nanos(input: &str) -> Option<Result<DateTime<Utc>>> {
    DateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S.%f%#z")
        .ok()
        .map(|parsed| parsed.with_timezone(&Utc))
        .map(Ok)
}

// 2021-04-30 21:14:10
fn parse_ymd_hms(input: &str) -> Option<Result<DateTime<Utc>>> {
    Local
        .datetime_from_str(input, "%Y-%m-%d %H:%M:%S")
        .ok()
        .map(|parsed| parsed.with_timezone(&Utc))
        .map(Ok)
}

// 2021-04-30 21:14:10.052282
fn parse_ymd_hms_nanos(input: &str) -> Option<Result<DateTime<Utc>>> {
    Local
        .datetime_from_str(input, "%Y-%m-%d %H:%M:%S.%f")
        .ok()
        .map(|parsed| parsed.with_timezone(&Utc))
        .map(Ok)
}

// 2017-11-25 13:31:15 PST
// 2017-11-25 13:31 PST
fn parse_ymd_hms_z(input: &str) -> Option<Result<DateTime<Utc>>> {
    match Regex::new(
        r"^(?P<dt>[0-9]{4}-[0-9]{2}-[0-9]{2}\s+[0-9]{2}:[0-9]{2}(:[0-9]{2})?)\s+(?P<tz>[a-zA-Z0-9]{3,4})$",
    ).map_err(Error::msg) {
        Ok(re) => {
            if let Some(caps) = re.captures(input) {
                if let Some(matched_dt) = caps.name("dt") {
                    if let Some(matched_tz) = caps.name("tz") {
                        return NaiveDateTime::parse_from_str(matched_dt.as_str(), "%Y-%m-%d %H:%M:%S")
                            .or_else(|_| {
                                NaiveDateTime::parse_from_str(matched_dt.as_str(), "%Y-%m-%d %H:%M")
                            })
                            .ok()
                            .and_then(|parsed| {
                                DateTime::parse_from_rfc2822(
                                    (parsed.format("%a, %d %b %Y %H:%M:%S").to_string()
                                        + " "
                                        + tz_2822(matched_tz.as_str()).as_ref())
                                    .as_ref(),
                                )
                                .ok()
                            })
                            .map(|datetime| datetime.with_timezone(&Utc))
                            .map(Ok)
                    }
                }
            }
            None
        }
        Err(err) => Some(Err(err)),
    }
}

// 2021-02-21
fn parse_ymd(input: &str) -> Option<Result<DateTime<Utc>>> {
    NaiveDate::parse_from_str(input, "%Y-%m-%d")
        .ok()
        .map(|parsed| parsed.and_time(Local::now().time()))
        .and_then(|datetime| Local.from_local_datetime(&datetime).single())
        .map(|local| local.with_timezone(&Utc))
        .map(Ok)
}

// 2021-02-21 PST
fn parse_ymd_z(input: &str) -> Option<Result<DateTime<Utc>>> {
    match Regex::new(r"^(?P<date>[0-9]{4}-[0-9]{2}-[0-9]{2})\s+(?P<tz>[a-zA-Z0-9]{3,4})$")
        .map_err(Error::msg)
    {
        Ok(re) => {
            if let Some(caps) = re.captures(input) {
                if let Some(matched_date) = caps.name("date") {
                    if let Some(matched_tz) = caps.name("tz") {
                        return NaiveDate::parse_from_str(matched_date.as_str(), "%Y-%m-%d")
                            .ok()
                            .and_then(|parsed| {
                                DateTime::parse_from_rfc2822(
                                    (parsed
                                        .and_time(Local::now().time())
                                        .format("%a, %d %b %Y %H:%M:%S")
                                        .to_string()
                                        + " "
                                        + tz_2822(matched_tz.as_str()).as_ref())
                                    .as_ref(),
                                )
                                .ok()
                            })
                            .map(|datetime| datetime.with_timezone(&Utc))
                            .map(Ok);
                    }
                }
            }
            None
        }
        Err(err) => Some(Err(err)),
    }
}

// 01:06:06
// 4:00pm
// 6:00 AM
fn parse_hms_imp(input: &str) -> Option<Result<DateTime<Utc>>> {
    NaiveTime::parse_from_str(input, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(input, "%I:%M%P"))
        .or_else(|_| NaiveTime::parse_from_str(input, "%I:%M %P"))
        .ok()
        .and_then(|parsed| Local::now().date().and_time(parsed))
        .map(|datetime| datetime.with_timezone(&Utc))
        .map(Ok)
}

// 01:06:06 PST
// 4:00pm PST
// 6:00 AM PST
fn parse_hms_imp_z(input: &str) -> Option<Result<DateTime<Utc>>> {
    match Regex::new(
        r"^(?P<time>[0-9]{1,2}:[0-9]{2}(:[0-9]{2})?(\s*(am|pm|AM|PM)?))\s+(?P<tz>[a-zA-Z0-9]{3,4})$",
    ).map_err(Error::msg) {
        Ok(re) => {
            if let Some(caps) = re.captures(input) {
                if let Some(matched_time) = caps.name("time") {
                    if let Some(matched_tz) = caps.name("tz") {
                        return NaiveTime::parse_from_str(matched_time.as_str(), "%H:%M:%S")
                            .or_else(|_| NaiveTime::parse_from_str(matched_time.as_str(), "%I:%M%P"))
                            .or_else(|_| NaiveTime::parse_from_str(matched_time.as_str(), "%I:%M %P"))
                            .ok()
                            .and_then(|parsed| {
                                DateTime::parse_from_rfc2822(
                                    (Local::now()
                                        .date()
                                        .naive_local()
                                        .and_time(parsed)
                                        .format("%a, %d %b %Y %H:%M:%S")
                                        .to_string()
                                        + " "
                                        + tz_2822(matched_tz.as_str()).as_ref())
                                    .as_ref(),
                                )
                                .ok()
                            })
                            .map(|datetime| datetime.with_timezone(&Utc))
                            .map(Ok);
                    }
                }
            }
            None
        },
        Err(err) => Some(Err(err)),
    }
}

// May 02, 2021 15:51:31 UTC
// May 02, 2021 15:51 UTC
fn parse_bey_hms_z(input: &str) -> Option<Result<DateTime<Utc>>> {
    match Regex::new(
        r"^(?P<dt>[a-zA-Z]{3}\s+[0-9]{1,2},\s+[0-9]{4}\s+[0-9]{2}:[0-9]{2}(:[0-9]{2})?)\s+(?P<tz>[a-zA-Z0-9]{3,4})$",
    ).map_err(Error::msg) {
        Ok(re) => {
            if let Some(caps) = re.captures(input) {
                if let Some(matched_dt) = caps.name("dt") {
                    if let Some(matched_tz) = caps.name("tz") {
                        return NaiveDateTime::parse_from_str(matched_dt.as_str(), "%b %e, %Y %H:%M:%S")
                            .or_else(|_| {
                                NaiveDateTime::parse_from_str(matched_dt.as_str(), "%b %e, %Y %H:%M")
                            })
                            .ok()
                            .and_then(|parsed| {
                                DateTime::parse_from_rfc2822(
                                    (parsed.format("%a, %d %b %Y %H:%M:%S").to_string()
                                        + " "
                                        + tz_2822(matched_tz.as_str()).as_ref())
                                    .as_ref(),
                                )
                                .ok()
                            })
                            .map(|datetime| datetime.with_timezone(&Utc))
                            .map(Ok)
                    }
                }
            }
            None
        }
        Err(err) => Some(Err(err)),
    }
}

fn tz_2822(tz: &str) -> String {
    let upper = tz.to_uppercase();
    match upper.as_ref() {
        "UT" | "UTC" => "GMT".to_string(),
        _ => upper,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    enum Trunc {
        Seconds,
        None,
    }

    #[test]
    fn test_parse() {
        let test_cases = vec![
            (
                "unix timestamp",
                "1511648546",
                Utc::ymd(&Utc, 2017, 11, 25).and_hms(22, 22, 26),
                Trunc::None,
            ),
            (
                "unix timestamp millis",
                "1620021848429",
                Utc::ymd(&Utc, 2021, 5, 3).and_hms_milli(6, 4, 8, 429),
                Trunc::None,
            ),
            (
                "unix_timestamp_nanos",
                "1620024872717915000",
                Utc::ymd(&Utc, 2021, 5, 3).and_hms_nano(6, 54, 32, 717915000),
                Trunc::None,
            ),
            (
                "rfc3339",
                "2017-11-25T22:34:50Z",
                Utc::ymd(&Utc, 2017, 11, 25).and_hms(22, 34, 50),
                Trunc::None,
            ),
            (
                "rfc2822",
                "Wed, 02 Jun 2021 06:31:39 GMT",
                Utc::ymd(&Utc, 2021, 6, 2).and_hms(6, 31, 39),
                Trunc::None,
            ),
            (
                "postgres_timestamp",
                "2019-11-29 08:08:05-08",
                Utc::ymd(&Utc, 2019, 11, 29).and_hms(16, 8, 5),
                Trunc::None,
            ),
            (
                "postgres_timestamp_nanos",
                "2019-11-29 08:15:47.624504-08",
                Utc::ymd(&Utc, 2019, 11, 29).and_hms_nano(16, 15, 47, 624504),
                Trunc::None,
            ),
            (
                "ymd_hms",
                "2021-04-30 21:14:10",
                Local::ymd(&Local, 2021, 4, 30)
                    .and_hms(21, 14, 10)
                    .with_timezone(&Utc),
                Trunc::None,
            ),
            (
                "ymd_hms_nanos",
                "2021-04-30 21:14:10.052282",
                Local::ymd(&Local, 2021, 4, 30)
                    .and_hms_nano(21, 14, 10, 52282)
                    .with_timezone(&Utc),
                Trunc::None,
            ),
            (
                "ymd_hms_z",
                "2017-11-25 13:31:15 PST",
                Utc::ymd(&Utc, 2017, 11, 25).and_hms(21, 31, 15),
                Trunc::None,
            ),
            (
                "ymd",
                "2021-02-21",
                Local::ymd(&Local, 2021, 2, 21)
                    .and_time(Local::now().time())
                    .unwrap()
                    .with_timezone(&Utc),
                Trunc::Seconds,
            ),
            (
                "ymd_z",
                "2021-02-21 PST",
                FixedOffset::west(8 * 3600)
                    .ymd(2021, 2, 21)
                    .and_time(Local::now().time())
                    .unwrap()
                    .with_timezone(&Utc),
                Trunc::Seconds,
            ),
            (
                "hms_imp",
                "4:00pm",
                Local::now()
                    .date()
                    .and_time(NaiveTime::from_hms(16, 0, 0))
                    .unwrap()
                    .with_timezone(&Utc),
                Trunc::None,
            ),
            (
                "hms_imp_z",
                "6:00 AM PST",
                FixedOffset::west(8 * 3600)
                    .from_local_date(&Local::now().date().naive_local())
                    .and_time(NaiveTime::from_hms(6, 0, 0))
                    .unwrap()
                    .with_timezone(&Utc),
                Trunc::None,
            ),
            (
                "bey_hms_z",
                "May 02, 2021 15:51 UTC",
                Utc::ymd(&Utc, 2021, 5, 2).and_hms(15, 51, 0),
                Trunc::None,
            ),
        ];

        for &(test, input, want, trunc) in test_cases.iter() {
            match trunc {
                Trunc::None => {
                    assert_eq!(parse(input).unwrap(), want, "test_parse/{}/{}", test, input)
                }
                Trunc::Seconds => assert_eq!(
                    parse(input)
                        .unwrap()
                        .trunc_subsecs(0)
                        .with_second(0)
                        .unwrap(),
                    want.trunc_subsecs(0).with_second(0).unwrap(),
                    "test_parse/{}/{}",
                    test,
                    input
                ),
            };
        }
    }

    #[test]
    fn test_parse_unix_timestamp() {
        let test_cases = vec![
            ("0000000000", Utc::ymd(&Utc, 1970, 1, 1).and_hms(0, 0, 0)),
            (
                "1511648546",
                Utc::ymd(&Utc, 2017, 11, 25).and_hms(22, 22, 26),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse_unix_timestamp(input).unwrap().unwrap(),
                want,
                "parse_unix_timestamp/{}",
                input
            )
        }
        assert!(parse_unix_timestamp("15116485461").is_none());
        assert!(parse_unix_timestamp("not-a-ts").is_none());
    }

    #[test]
    fn test_parse_unix_timestamp_millis() {
        let test_cases = vec![
            ("0000000000000", Utc::ymd(&Utc, 1970, 1, 1).and_hms(0, 0, 0)),
            (
                "1620021848429",
                Utc::ymd(&Utc, 2021, 5, 3).and_hms_milli(6, 4, 8, 429),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse_unix_timestamp_millis(input).unwrap().unwrap(),
                want,
                "parse_unix_timestamp_millis/{}",
                input
            )
        }
        assert!(parse_unix_timestamp_millis("16200218484291").is_none());
        assert!(parse_unix_timestamp_millis("not-a-ts").is_none());
    }

    #[test]
    fn test_parse_unix_timestamp_nanos() {
        let test_cases = vec![
            (
                "0000000000000000000",
                Utc::ymd(&Utc, 1970, 1, 1).and_hms(0, 0, 0),
            ),
            (
                "1620024872717915000",
                Utc::ymd(&Utc, 2021, 5, 3).and_hms_nano(6, 54, 32, 717915000),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse_unix_timestamp_nanos(input).unwrap().unwrap(),
                want,
                "parse_unix_timestamp_nanos/{}",
                input
            )
        }
        assert!(parse_unix_timestamp_nanos("16200248727179150001").is_none());
        assert!(parse_unix_timestamp_nanos("not-a-ts").is_none());
    }

    #[test]
    fn test_parse_rfc3339() {
        let test_cases = vec![
            (
                "2021-05-01T01:17:02.604456Z",
                Utc::ymd(&Utc, 2021, 5, 1).and_hms_nano(1, 17, 2, 604456000),
            ),
            (
                "2017-11-25T22:34:50Z",
                Utc::ymd(&Utc, 2017, 11, 25).and_hms(22, 34, 50),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse_rfc3339(input).unwrap().unwrap(),
                want,
                "parse_rfc3339/{}",
                input
            )
        }
        assert!(parse_rfc3339("2017-11-25 22:34:50").is_none());
        assert!(parse_rfc3339("not-date-time").is_none());
    }

    #[test]
    fn test_parse_rfc2822() {
        let test_cases = vec![
            (
                "Wed, 02 Jun 2021 06:31:39 GMT",
                Utc::ymd(&Utc, 2021, 6, 2).and_hms(6, 31, 39),
            ),
            (
                "Wed, 02 Jun 2021 06:31:39 PDT",
                Utc::ymd(&Utc, 2021, 6, 2).and_hms(13, 31, 39),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse_rfc2822(input).unwrap().unwrap(),
                want,
                "parse_rfc2822/{}",
                input
            )
        }
        assert!(parse_rfc2822("02 Jun 2021 06:31:39").is_none());
        assert!(parse_rfc2822("not-date-time").is_none());
    }

    #[test]
    fn test_parse_postgres_timestamp() {
        let test_cases = vec![(
            "2019-11-29 08:08:05-08",
            Utc::ymd(&Utc, 2019, 11, 29).and_hms(16, 8, 5),
        )];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse_postgres_timestamp(input).unwrap().unwrap(),
                want,
                "parse_postgres_timestamp/{}",
                input
            )
        }
        assert!(parse_postgres_timestamp("not-date-time").is_none());
    }

    #[test]
    fn test_parse_postgres_timestamp_nanos() {
        let test_cases = vec![
            (
                "2021-05-02 23:31:36.0741-07",
                Utc::ymd(&Utc, 2021, 5, 3).and_hms_nano(6, 31, 36, 741),
            ),
            (
                "2021-05-02 23:31:39.12689-07",
                Utc::ymd(&Utc, 2021, 5, 3).and_hms_nano(6, 31, 39, 12689),
            ),
            (
                "2019-11-29 08:15:47.624504-08",
                Utc::ymd(&Utc, 2019, 11, 29).and_hms_nano(16, 15, 47, 624504),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse_postgres_timestamp_nanos(input).unwrap().unwrap(),
                want,
                "parse_postgres_timestamp_nanos/{}",
                input
            )
        }
        assert!(parse_postgres_timestamp_nanos("not-date-time").is_none());
    }

    #[test]
    fn test_parse_ymd_hms() {
        let test_cases = vec![(
            "2021-04-30 21:14:10",
            Local::ymd(&Local, 2021, 4, 30)
                .and_hms(21, 14, 10)
                .with_timezone(&Utc),
        )];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse_ymd_hms(input).unwrap().unwrap(),
                want,
                "parse_ymd_hms/{}",
                input
            )
        }
        assert!(parse_ymd_hms("not-date-time").is_none());
    }

    #[test]
    fn test_parse_ymd_hms_nanos() {
        let test_cases = vec![(
            "2021-04-30 21:14:10.052282",
            Local::ymd(&Local, 2021, 4, 30)
                .and_hms_nano(21, 14, 10, 52282)
                .with_timezone(&Utc),
        )];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse_ymd_hms_nanos(input).unwrap().unwrap(),
                want,
                "parse_ymd_hms_nanos/{}",
                input
            )
        }
        assert!(parse_ymd_hms_nanos("not-date-time").is_none());
    }

    #[test]
    fn test_parse_ymd_hms_z() {
        let test_cases = vec![
            (
                "2017-11-25 13:31:15 PST",
                Utc::ymd(&Utc, 2017, 11, 25).and_hms(21, 31, 15),
            ),
            (
                "2017-11-25 13:31 PST",
                Utc::ymd(&Utc, 2017, 11, 25).and_hms(21, 31, 0),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse_ymd_hms_z(input).unwrap().unwrap(),
                want,
                "parse_ymd_hms_z/{}",
                input
            )
        }
        assert!(parse_ymd_hms_z("not-date-time").is_none());
    }

    #[test]
    fn test_parse_ymd() {
        let test_cases = vec![(
            "2021-02-21",
            Local::ymd(&Local, 2021, 2, 21)
                .and_time(Local::now().time())
                .map(|dt| dt.with_timezone(&Utc)),
        )];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse_ymd(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "parse_ymd/{}",
                input
            )
        }
        assert!(parse_ymd("not-date-time").is_none());
    }

    #[test]
    fn test_parse_ymd_z() {
        let test_cases = vec![(
            "2021-02-21 PST",
            FixedOffset::west(8 * 3600)
                .ymd(2021, 2, 21)
                .and_time(Local::now().time())
                .map(|dt| dt.with_timezone(&Utc)),
        )];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse_ymd_z(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "parse_ymd_z/{}",
                input
            )
        }
        assert!(parse_ymd_z("not-date-time").is_none());
    }

    #[test]
    fn test_parse_hms_imp() {
        let test_cases = vec![
            (
                "01:06:06",
                Local::now()
                    .date()
                    .and_time(NaiveTime::from_hms(1, 6, 6))
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
            (
                "4:00pm",
                Local::now()
                    .date()
                    .and_time(NaiveTime::from_hms(16, 0, 0))
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
            (
                "6:00 AM",
                Local::now()
                    .date()
                    .and_time(NaiveTime::from_hms(6, 0, 0))
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse_hms_imp(input).unwrap().unwrap(),
                want.unwrap(),
                "parse_hms_imp/{}",
                input
            )
        }
        assert!(parse_hms_imp("not-date-time").is_none());
    }

    #[test]
    fn test_parse_hms_imp_z() {
        let test_cases = vec![
            (
                "01:06:06 PST",
                FixedOffset::west(8 * 3600)
                    .from_local_date(&Local::now().date().naive_local())
                    .and_time(NaiveTime::from_hms(1, 6, 6))
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
            (
                "4:00pm PST",
                FixedOffset::west(8 * 3600)
                    .from_local_date(&Local::now().date().naive_local())
                    .and_time(NaiveTime::from_hms(16, 0, 0))
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
            (
                "6:00 AM PST",
                FixedOffset::west(8 * 3600)
                    .from_local_date(&Local::now().date().naive_local())
                    .and_time(NaiveTime::from_hms(6, 0, 0))
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse_hms_imp_z(input).unwrap().unwrap(),
                want.unwrap(),
                "parse_hms_imp_z/{}",
                input
            )
        }
        assert!(parse_hms_imp_z("not-date-time").is_none());
    }

    #[test]
    fn test_parse_bey_hms_z() {
        assert_eq!(
            parse_bey_hms_z("May 02, 2021 15:51:31 UTC")
                .unwrap()
                .unwrap(),
            Utc::ymd(&Utc, 2021, 5, 2).and_hms(15, 51, 31)
        );
        assert_eq!(
            parse_bey_hms_z("May 02, 2021 15:51 UTC").unwrap().unwrap(),
            Utc::ymd(&Utc, 2021, 5, 2).and_hms(15, 51, 0)
        );
        assert!(parse_bey_hms_z("not-date-time").is_none());
    }

    #[test]
    fn test_tz_2822() {
        assert_eq!(tz_2822("UT"), "GMT");
        assert_eq!(tz_2822("UTC"), "GMT");
        assert_eq!(tz_2822("utc"), "GMT");
        assert_eq!(tz_2822("EST"), "EST");
        assert_eq!(tz_2822("pdt"), "PDT");
    }
}
