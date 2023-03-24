#![allow(deprecated)]
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
//!     assert_eq!(
//!         parse("6:15pm UTC")?,
//!         Utc::now().date().and_time(
//!             NaiveTime::from_hms(18, 15, 0),
//!         ).unwrap(),
//!     );
//!     Ok(())
//! }
//! ```
//!
//! Use `str`'s `parse` method:
//!
//! ```
//! use chrono::prelude::*;
//! use dateparser::DateTimeUtc;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     assert_eq!(
//!         "2021-05-14 18:51 PDT".parse::<DateTimeUtc>()?.0,
//!         Utc.ymd(2021, 5, 15).and_hms(1, 51, 0),
//!     );
//!     Ok(())
//! }
//! ```
//!
//! Parse using a custom timezone offset for a datetime string that doesn't come with a specific
//! timezone:
//!
//! ```
//! use dateparser::parse_with_timezone;
//! use chrono::offset::Utc;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let parsed_in_utc = parse_with_timezone("6:15pm", &Utc)?;
//!     assert_eq!(
//!         parsed_in_utc,
//!         Utc::now().date().and_hms(18, 15, 0),
//!     );
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
//!     // unix timestamp
//!     "1511648546",
//!     "1620021848429",
//!     "1620024872717915000",
//!     // rfc3339
//!     "2021-05-01T01:17:02.604456Z",
//!     "2017-11-25T22:34:50Z",
//!     // rfc2822
//!     "Wed, 02 Jun 2021 06:31:39 GMT",
//!     // postgres timestamp yyyy-mm-dd hh:mm:ss z
//!     "2019-11-29 08:08-08",
//!     "2019-11-29 08:08:05-08",
//!     "2021-05-02 23:31:36.0741-07",
//!     "2021-05-02 23:31:39.12689-07",
//!     "2019-11-29 08:15:47.624504-08",
//!     "2017-07-19 03:21:51+00:00",
//!     // yyyy-mm-dd hh:mm:ss
//!     "2014-04-26 05:24:37 PM",
//!     "2021-04-30 21:14",
//!     "2021-04-30 21:14:10",
//!     "2021-04-30 21:14:10.052282",
//!     "2014-04-26 17:24:37.123",
//!     "2014-04-26 17:24:37.3186369",
//!     "2012-08-03 18:31:59.257000000",
//!     // yyyy-mm-dd hh:mm:ss z
//!     "2017-11-25 13:31:15 PST",
//!     "2017-11-25 13:31 PST",
//!     "2014-12-16 06:20:00 UTC",
//!     "2014-12-16 06:20:00 GMT",
//!     "2014-04-26 13:13:43 +0800",
//!     "2014-04-26 13:13:44 +09:00",
//!     "2012-08-03 18:31:59.257000000 +0000",
//!     "2015-09-30 18:48:56.35272715 UTC",
//!     // yyyy-mm-dd
//!     "2021-02-21",
//!     // yyyy-mm-dd z
//!     "2021-02-21 PST",
//!     "2021-02-21 UTC",
//!     "2020-07-20+08:00",
//!     // hh:mm:ss
//!     "01:06:06",
//!     "4:00pm",
//!     "6:00 AM",
//!     // hh:mm:ss z
//!     "01:06:06 PST",
//!     "4:00pm PST",
//!     "6:00 AM PST",
//!     "6:00pm UTC",
//!     // Mon dd hh:mm:ss
//!     "May 6 at 9:24 PM",
//!     "May 27 02:45:27",
//!     // Mon dd, yyyy, hh:mm:ss
//!     "May 8, 2009 5:57:51 PM",
//!     "September 17, 2012 10:09am",
//!     "September 17, 2012, 10:10:09",
//!     // Mon dd, yyyy hh:mm:ss z
//!     "May 02, 2021 15:51:31 UTC",
//!     "May 02, 2021 15:51 UTC",
//!     "May 26, 2021, 12:49 AM PDT",
//!     "September 17, 2012 at 10:09am PST",
//!     // yyyy-mon-dd
//!     "2021-Feb-21",
//!     // Mon dd, yyyy
//!     "May 25, 2021",
//!     "oct 7, 1970",
//!     "oct 7, 70",
//!     "oct. 7, 1970",
//!     "oct. 7, 70",
//!     "October 7, 1970",
//!     // dd Mon yyyy hh:mm:ss
//!     "12 Feb 2006, 19:17",
//!     "12 Feb 2006 19:17",
//!     "14 May 2019 19:11:40.164",
//!     // dd Mon yyyy
//!     "7 oct 70",
//!     "7 oct 1970",
//!     "03 February 2013",
//!     "1 July 2013",
//!     // mm/dd/yyyy hh:mm:ss
//!     "4/8/2014 22:05",
//!     "04/08/2014 22:05",
//!     "4/8/14 22:05",
//!     "04/2/2014 03:00:51",
//!     "8/8/1965 12:00:00 AM",
//!     "8/8/1965 01:00:01 PM",
//!     "8/8/1965 01:00 PM",
//!     "8/8/1965 1:00 PM",
//!     "8/8/1965 12:00 AM",
//!     "4/02/2014 03:00:51",
//!     "03/19/2012 10:11:59",
//!     "03/19/2012 10:11:59.3186369",
//!     // mm/dd/yyyy
//!     "3/31/2014",
//!     "03/31/2014",
//!     "08/21/71",
//!     "8/1/71",
//!     // yyyy/mm/dd hh:mm:ss
//!     "2014/4/8 22:05",
//!     "2014/04/08 22:05",
//!     "2014/04/2 03:00:51",
//!     "2014/4/02 03:00:51",
//!     "2012/03/19 10:11:59",
//!     "2012/03/19 10:11:59.3186369",
//!     // yyyy/mm/dd
//!     "2014/3/31",
//!     "2014/03/31",
//!     // mm.dd.yyyy
//!     "3.31.2014",
//!     "03.31.2014",
//!     "08.21.71",
//!     // yyyy.mm.dd
//!     "2014.03.30",
//!     "2014.03",
//!     // yymmdd hh:mm:ss mysql log
//!     "171113 14:14:20",
//!     // chinese yyyy mm dd hh mm ss
//!     "2014年04月08日11时25分18秒",
//!     // chinese yyyy mm dd
//!     "2014年04月08日",
//! ];
//!
//! for date_str in accepted {
//!     let result = date_str.parse::<DateTimeUtc>();
//!     assert!(result.is_ok())
//! }
//! ```

/// Datetime string parser
///
/// ```
/// use chrono::prelude::*;
/// use dateparser::datetime::Parse;
/// use std::error::Error;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let parse_with_local = Parse::new(&Local, None);
///     assert_eq!(
///         parse_with_local.parse("2021-06-05 06:19 PM")?,
///         Local.ymd(2021, 6, 5).and_hms(18, 19, 0).with_timezone(&Utc),
///     );
///
///     let parse_with_utc = Parse::new(&Utc, None);
///     assert_eq!(
///         parse_with_utc.parse("2021-06-05 06:19 PM")?,
///         Utc.ymd(2021, 6, 5).and_hms(18, 19, 0),
///     );
///
///     Ok(())
/// }
/// ```
pub mod datetime;

/// Timezone offset string parser
///
/// ```
/// use chrono::prelude::*;
/// use dateparser::timezone::parse;
/// use std::error::Error;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     assert_eq!(parse("-0800")?, FixedOffset::west(8 * 3600));
///     assert_eq!(parse("+10:00")?, FixedOffset::east(10 * 3600));
///     assert_eq!(parse("PST")?, FixedOffset::west(8 * 3600));
///     assert_eq!(parse("PDT")?, FixedOffset::west(7 * 3600));
///     assert_eq!(parse("UTC")?, FixedOffset::west(0));
///     assert_eq!(parse("GMT")?, FixedOffset::west(0));
///
///     Ok(())
/// }
/// ```
pub mod timezone;

use crate::datetime::Parse;
use anyhow::{Error, Result};
use chrono::prelude::*;

/// DateTimeUtc is an alias for `chrono`'s `DateTime<UTC>`. It implements `std::str::FromStr`'s
/// `from_str` method, and it makes `str`'s `parse` method to understand the accepted date formats
/// from this crate.
///
/// ```
/// use dateparser::DateTimeUtc;
///
/// // parsed is DateTimeUTC and parsed.0 is chrono's DateTime<Utc>
/// match "May 02, 2021 15:51:31 UTC".parse::<DateTimeUtc>() {
///     Ok(parsed) => println!("PARSED into UTC datetime {:?}", parsed.0),
///     Err(err) => println!("ERROR from parsing datetime string: {}", err)
/// }
/// ```
#[derive(Clone, Debug)]
pub struct DateTimeUtc(pub DateTime<Utc>);

impl std::str::FromStr for DateTimeUtc {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        parse(s).map(DateTimeUtc)
    }
}

/// This function tries to recognize the input datetime string with a list of accepted formats.
/// When timezone is not provided, this function assumes it's a [`chrono::Local`] datetime. For
/// custom timezone, use [`parse_with_timezone()`] instead.If all options are exhausted,
/// [`parse()`] will return an error to let the caller know that no formats were matched.
///
/// ```
/// use dateparser::parse;
/// use chrono::offset::{Local, Utc};
/// use chrono_tz::US::Pacific;
///
/// let parsed = parse("6:15pm").unwrap();
///
/// assert_eq!(
///     parsed,
///     Local::now().date().and_hms(18, 15, 0).with_timezone(&Utc),
/// );
///
/// assert_eq!(
///     parsed.with_timezone(&Pacific),
///     Local::now().date().and_hms(18, 15, 0).with_timezone(&Utc).with_timezone(&Pacific),
/// );
/// ```
pub fn parse(input: &str) -> Result<DateTime<Utc>> {
    Parse::new(&Local, None).parse(input)
}

/// Similar to [`parse()`], this function takes a datetime string and a custom [`chrono::TimeZone`],
/// and tries to parse the datetime string. When timezone is not given in the string, this function
/// will assume and parse the datetime by the custom timezone provided in this function's arguments.
///
/// ```
/// use dateparser::parse_with_timezone;
/// use chrono::offset::{Local, Utc};
/// use chrono_tz::US::Pacific;
///
/// let parsed_in_local = parse_with_timezone("6:15pm", &Local).unwrap();
/// assert_eq!(
///     parsed_in_local,
///     Local::now().date().and_hms(18, 15, 0).with_timezone(&Utc),
/// );
///
/// let parsed_in_utc = parse_with_timezone("6:15pm", &Utc).unwrap();
/// assert_eq!(
///     parsed_in_utc,
///     Utc::now().date().and_hms(18, 15, 0),
/// );
///
/// let parsed_in_pacific = parse_with_timezone("6:15pm", &Pacific).unwrap();
/// assert_eq!(
///     parsed_in_pacific,
///     Utc::now().with_timezone(&Pacific).date().and_hms(18, 15, 0).with_timezone(&Utc),
/// );
/// ```
pub fn parse_with_timezone<Tz2: TimeZone>(input: &str, tz: &Tz2) -> Result<DateTime<Utc>> {
    Parse::new(tz, None).parse(input)
}

/// Similar to [`parse()`] and [`parse_with_timezone()`], this function takes a datetime string, a
/// custom [`chrono::TimeZone`] and a default naive time. In addition to assuming timezone when
/// it's not given in datetime string, this function also use provided default naive time in parsed
/// [`chrono::DateTime`].
///
/// ```
/// use dateparser::parse_with;
/// use chrono::prelude::*;
///
/// let utc_now = Utc::now().time().trunc_subsecs(0);
/// let local_now = Local::now().time().trunc_subsecs(0);
/// let midnight_naive = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
/// let before_midnight_naive = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
///
/// let parsed_with_local_now = parse_with("2021-10-09", &Local, local_now);
/// let parsed_with_local_midnight = parse_with("2021-10-09", &Local, midnight_naive);
/// let parsed_with_local_before_midnight = parse_with("2021-10-09", &Local, before_midnight_naive);
/// let parsed_with_utc_now = parse_with("2021-10-09", &Utc, utc_now);
/// let parsed_with_utc_midnight = parse_with("2021-10-09", &Utc, midnight_naive);
///
/// assert_eq!(
///     parsed_with_local_now.unwrap(),
///     Local.ymd(2021, 10, 9).and_time(local_now).unwrap().with_timezone(&Utc),
///     "parsed_with_local_now"
/// );
/// assert_eq!(
///     parsed_with_local_midnight.unwrap(),
///     Local.ymd(2021, 10, 9).and_time(midnight_naive).unwrap().with_timezone(&Utc),
///     "parsed_with_local_midnight"
/// );
/// assert_eq!(
///     parsed_with_local_before_midnight.unwrap(),
///     Local.ymd(2021, 10, 9).and_time(before_midnight_naive).unwrap().with_timezone(&Utc),
///     "parsed_with_local_before_midnight"
/// );
/// assert_eq!(
///     parsed_with_utc_now.unwrap(),
///     Utc.ymd(2021, 10, 9).and_time(utc_now).unwrap(),
///     "parsed_with_utc_now"
/// );
/// assert_eq!(
///     parsed_with_utc_midnight.unwrap(),
///     Utc.ymd(2021, 10, 9).and_hms(0, 0, 0),
///     "parsed_with_utc_midnight"
/// );
/// ```
pub fn parse_with<Tz2: TimeZone>(
    input: &str,
    tz: &Tz2,
    default_time: NaiveTime,
) -> Result<DateTime<Utc>> {
    Parse::new(tz, Some(default_time)).parse(input)
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
    fn parse_in_local() {
        let test_cases = vec![
            (
                "unix_timestamp",
                "1511648546",
                Utc.ymd(2017, 11, 25).and_hms(22, 22, 26),
                Trunc::None,
            ),
            (
                "rfc3339",
                "2017-11-25T22:34:50Z",
                Utc.ymd(2017, 11, 25).and_hms(22, 34, 50),
                Trunc::None,
            ),
            (
                "rfc2822",
                "Wed, 02 Jun 2021 06:31:39 GMT",
                Utc.ymd(2021, 6, 2).and_hms(6, 31, 39),
                Trunc::None,
            ),
            (
                "postgres_timestamp",
                "2019-11-29 08:08:05-08",
                Utc.ymd(2019, 11, 29).and_hms(16, 8, 5),
                Trunc::None,
            ),
            (
                "ymd_hms",
                "2021-04-30 21:14:10",
                Local
                    .ymd(2021, 4, 30)
                    .and_hms(21, 14, 10)
                    .with_timezone(&Utc),
                Trunc::None,
            ),
            (
                "ymd_hms_z",
                "2017-11-25 13:31:15 PST",
                Utc.ymd(2017, 11, 25).and_hms(21, 31, 15),
                Trunc::None,
            ),
            (
                "ymd",
                "2021-02-21",
                Local
                    .ymd(2021, 2, 21)
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
                    .and_time(
                        Utc::now()
                            .with_timezone(&FixedOffset::west(8 * 3600))
                            .time(),
                    )
                    .unwrap()
                    .with_timezone(&Utc),
                Trunc::Seconds,
            ),
            (
                "hms",
                "4:00pm",
                Local::now()
                    .date()
                    .and_time(NaiveTime::from_hms(16, 0, 0))
                    .unwrap()
                    .with_timezone(&Utc),
                Trunc::None,
            ),
            (
                "hms_z",
                "6:00 AM PST",
                Utc::now()
                    .with_timezone(&FixedOffset::west(8 * 3600))
                    .date()
                    .and_time(NaiveTime::from_hms(6, 0, 0))
                    .unwrap()
                    .with_timezone(&Utc),
                Trunc::None,
            ),
            (
                "month_ymd",
                "2021-Feb-21",
                Local
                    .ymd(2021, 2, 21)
                    .and_time(Local::now().time())
                    .unwrap()
                    .with_timezone(&Utc),
                Trunc::Seconds,
            ),
            (
                "month_md_hms",
                "May 27 02:45:27",
                Local
                    .ymd(Local::now().year(), 5, 27)
                    .and_hms(2, 45, 27)
                    .with_timezone(&Utc),
                Trunc::None,
            ),
            (
                "month_mdy_hms",
                "May 8, 2009 5:57:51 PM",
                Local
                    .ymd(2009, 5, 8)
                    .and_hms(17, 57, 51)
                    .with_timezone(&Utc),
                Trunc::None,
            ),
            (
                "month_mdy_hms_z",
                "May 02, 2021 15:51 UTC",
                Utc.ymd(2021, 5, 2).and_hms(15, 51, 0),
                Trunc::None,
            ),
            (
                "month_mdy",
                "May 25, 2021",
                Local
                    .ymd(2021, 5, 25)
                    .and_time(Local::now().time())
                    .unwrap()
                    .with_timezone(&Utc),
                Trunc::Seconds,
            ),
            (
                "month_dmy_hms",
                "14 May 2019 19:11:40.164",
                Local
                    .ymd(2019, 5, 14)
                    .and_hms_milli(19, 11, 40, 164)
                    .with_timezone(&Utc),
                Trunc::None,
            ),
            (
                "month_dmy",
                "1 July 2013",
                Local
                    .ymd(2013, 7, 1)
                    .and_time(Local::now().time())
                    .unwrap()
                    .with_timezone(&Utc),
                Trunc::Seconds,
            ),
            (
                "slash_mdy_hms",
                "03/19/2012 10:11:59",
                Local
                    .ymd(2012, 3, 19)
                    .and_hms(10, 11, 59)
                    .with_timezone(&Utc),
                Trunc::None,
            ),
            (
                "slash_mdy",
                "08/21/71",
                Local
                    .ymd(1971, 8, 21)
                    .and_time(Local::now().time())
                    .unwrap()
                    .with_timezone(&Utc),
                Trunc::Seconds,
            ),
            (
                "slash_ymd_hms",
                "2012/03/19 10:11:59",
                Local
                    .ymd(2012, 3, 19)
                    .and_hms(10, 11, 59)
                    .with_timezone(&Utc),
                Trunc::None,
            ),
            (
                "slash_ymd",
                "2014/3/31",
                Local
                    .ymd(2014, 3, 31)
                    .and_time(Local::now().time())
                    .unwrap()
                    .with_timezone(&Utc),
                Trunc::Seconds,
            ),
            (
                "dot_mdy_or_ymd",
                "2014.03.30",
                Local
                    .ymd(2014, 3, 30)
                    .and_time(Local::now().time())
                    .unwrap()
                    .with_timezone(&Utc),
                Trunc::Seconds,
            ),
            (
                "mysql_log_timestamp",
                "171113 14:14:20",
                Local
                    .ymd(2017, 11, 13)
                    .and_hms(14, 14, 20)
                    .with_timezone(&Utc),
                Trunc::None,
            ),
            (
                "chinese_ymd_hms",
                "2014年04月08日11时25分18秒",
                Local
                    .ymd(2014, 4, 8)
                    .and_hms(11, 25, 18)
                    .with_timezone(&Utc),
                Trunc::None,
            ),
            (
                "chinese_ymd",
                "2014年04月08日",
                Local
                    .ymd(2014, 4, 8)
                    .and_time(Local::now().time())
                    .unwrap()
                    .with_timezone(&Utc),
                Trunc::Seconds,
            ),
        ];

        for &(test, input, want, trunc) in test_cases.iter() {
            match trunc {
                Trunc::None => {
                    assert_eq!(
                        super::parse(input).unwrap(),
                        want,
                        "parse_in_local/{}/{}",
                        test,
                        input
                    )
                }
                Trunc::Seconds => assert_eq!(
                    super::parse(input)
                        .unwrap()
                        .trunc_subsecs(0)
                        .with_second(0)
                        .unwrap(),
                    want.trunc_subsecs(0).with_second(0).unwrap(),
                    "parse_in_local/{}/{}",
                    test,
                    input
                ),
            };
        }
    }

    #[test]
    fn parse_with_timezone_in_utc() {
        let test_cases = vec![
            (
                "unix_timestamp",
                "1511648546",
                Utc.ymd(2017, 11, 25).and_hms(22, 22, 26),
                Trunc::None,
            ),
            (
                "rfc3339",
                "2017-11-25T22:34:50Z",
                Utc.ymd(2017, 11, 25).and_hms(22, 34, 50),
                Trunc::None,
            ),
            (
                "rfc2822",
                "Wed, 02 Jun 2021 06:31:39 GMT",
                Utc.ymd(2021, 6, 2).and_hms(6, 31, 39),
                Trunc::None,
            ),
            (
                "postgres_timestamp",
                "2019-11-29 08:08:05-08",
                Utc.ymd(2019, 11, 29).and_hms(16, 8, 5),
                Trunc::None,
            ),
            (
                "ymd_hms",
                "2021-04-30 21:14:10",
                Utc.ymd(2021, 4, 30).and_hms(21, 14, 10),
                Trunc::None,
            ),
            (
                "ymd_hms_z",
                "2017-11-25 13:31:15 PST",
                Utc.ymd(2017, 11, 25).and_hms(21, 31, 15),
                Trunc::None,
            ),
            (
                "ymd",
                "2021-02-21",
                Utc.ymd(2021, 2, 21).and_time(Utc::now().time()).unwrap(),
                Trunc::Seconds,
            ),
            (
                "ymd_z",
                "2021-02-21 PST",
                FixedOffset::west(8 * 3600)
                    .ymd(2021, 2, 21)
                    .and_time(
                        Utc::now()
                            .with_timezone(&FixedOffset::west(8 * 3600))
                            .time(),
                    )
                    .unwrap()
                    .with_timezone(&Utc),
                Trunc::Seconds,
            ),
            (
                "hms",
                "4:00pm",
                Utc::now()
                    .date()
                    .and_time(NaiveTime::from_hms(16, 0, 0))
                    .unwrap(),
                Trunc::None,
            ),
            (
                "hms_z",
                "6:00 AM PST",
                FixedOffset::west(8 * 3600)
                    .from_local_date(
                        &Utc::now()
                            .with_timezone(&FixedOffset::west(8 * 3600))
                            .date()
                            .naive_local(),
                    )
                    .and_time(NaiveTime::from_hms(6, 0, 0))
                    .unwrap()
                    .with_timezone(&Utc),
                Trunc::None,
            ),
            (
                "month_ymd",
                "2021-Feb-21",
                Utc.ymd(2021, 2, 21).and_time(Utc::now().time()).unwrap(),
                Trunc::Seconds,
            ),
            (
                "month_md_hms",
                "May 27 02:45:27",
                Utc.ymd(Utc::now().year(), 5, 27).and_hms(2, 45, 27),
                Trunc::None,
            ),
            (
                "month_mdy_hms",
                "May 8, 2009 5:57:51 PM",
                Utc.ymd(2009, 5, 8).and_hms(17, 57, 51),
                Trunc::None,
            ),
            (
                "month_mdy_hms_z",
                "May 02, 2021 15:51 UTC",
                Utc.ymd(2021, 5, 2).and_hms(15, 51, 0),
                Trunc::None,
            ),
            (
                "month_mdy",
                "May 25, 2021",
                Utc.ymd(2021, 5, 25).and_time(Utc::now().time()).unwrap(),
                Trunc::Seconds,
            ),
            (
                "month_dmy_hms",
                "14 May 2019 19:11:40.164",
                Utc.ymd(2019, 5, 14).and_hms_milli(19, 11, 40, 164),
                Trunc::None,
            ),
            (
                "month_dmy",
                "1 July 2013",
                Utc.ymd(2013, 7, 1).and_time(Utc::now().time()).unwrap(),
                Trunc::Seconds,
            ),
            (
                "slash_mdy_hms",
                "03/19/2012 10:11:59",
                Utc.ymd(2012, 3, 19).and_hms(10, 11, 59),
                Trunc::None,
            ),
            (
                "slash_mdy",
                "08/21/71",
                Utc.ymd(1971, 8, 21).and_time(Utc::now().time()).unwrap(),
                Trunc::Seconds,
            ),
            (
                "slash_ymd_hms",
                "2012/03/19 10:11:59",
                Utc.ymd(2012, 3, 19).and_hms(10, 11, 59),
                Trunc::None,
            ),
            (
                "slash_ymd",
                "2014/3/31",
                Utc.ymd(2014, 3, 31).and_time(Utc::now().time()).unwrap(),
                Trunc::Seconds,
            ),
            (
                "dot_mdy_or_ymd",
                "2014.03.30",
                Utc.ymd(2014, 3, 30).and_time(Utc::now().time()).unwrap(),
                Trunc::Seconds,
            ),
            (
                "mysql_log_timestamp",
                "171113 14:14:20",
                Utc.ymd(2017, 11, 13).and_hms(14, 14, 20),
                Trunc::None,
            ),
            (
                "chinese_ymd_hms",
                "2014年04月08日11时25分18秒",
                Utc.ymd(2014, 4, 8).and_hms(11, 25, 18),
                Trunc::None,
            ),
            (
                "chinese_ymd",
                "2014年04月08日",
                Utc.ymd(2014, 4, 8).and_time(Utc::now().time()).unwrap(),
                Trunc::Seconds,
            ),
        ];

        for &(test, input, want, trunc) in test_cases.iter() {
            match trunc {
                Trunc::None => {
                    assert_eq!(
                        super::parse_with_timezone(input, &Utc).unwrap(),
                        want,
                        "parse_with_timezone_in_utc/{}/{}",
                        test,
                        input
                    )
                }
                Trunc::Seconds => assert_eq!(
                    super::parse_with_timezone(input, &Utc)
                        .unwrap()
                        .trunc_subsecs(0)
                        .with_second(0)
                        .unwrap(),
                    want.trunc_subsecs(0).with_second(0).unwrap(),
                    "parse_with_timezone_in_utc/{}/{}",
                    test,
                    input
                ),
            };
        }
    }

    // test parse_with() with various timezones and times

    #[test]
    fn parse_with_edt() {
        // Eastern Daylight Time (EDT) is from (as of 2023) 2nd Sun in Mar to 1st Sun in Nov
        // It is UTC -4

        let midnight_naive = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let before_midnight_naive = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
        let us_edt = &FixedOffset::west_opt(4 * 3600).unwrap();

        let edt_test_cases = vec![
            ("ymd", "2023-04-21"),
            ("ymd_z", "2023-04-21 EDT"),
            ("month_ymd", "2023-Apr-21"),
            ("month_mdy", "April 21, 2023"),
            ("month_dmy", "21 April 2023"),
            ("slash_mdy", "04/21/23"),
            ("slash_ymd", "2023/4/21"),
            ("dot_mdy_or_ymd", "2023.04.21"),
            ("chinese_ymd", "2023年04月21日"),
        ];

        // test us_edt at midnight
        let us_edt_midnight_as_utc = Utc.ymd(2023, 4, 21).and_hms(4, 0, 0);

        for &(test, input) in edt_test_cases.iter() {
            assert_eq!(
                super::parse_with(input, us_edt, midnight_naive).unwrap(),
                us_edt_midnight_as_utc,
                "parse_with/{test}/{input}",
            )
        }

        // test us_edt at 23:59:59 - UTC will be one day ahead
        let us_edt_before_midnight_as_utc = Utc.ymd(2023, 4, 22).and_hms(3, 59, 59);
        for &(test, input) in edt_test_cases.iter() {
            assert_eq!(
                super::parse_with(input, us_edt, before_midnight_naive).unwrap(),
                us_edt_before_midnight_as_utc,
                "parse_with/{test}/{input}",
            )
        }
    }

    #[test]
    fn parse_with_est() {
        // Eastern Standard Time (EST) is from (as of 2023) 1st Sun in Nov to 2nd Sun in Mar
        // It is UTC -5

        let midnight_naive = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let before_midnight_naive = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
        let us_est = &FixedOffset::west(5 * 3600);

        let est_test_cases = vec![
            ("ymd", "2023-12-21"),
            ("ymd_z", "2023-12-21 EST"),
            ("month_ymd", "2023-Dec-21"),
            ("month_mdy", "December 21, 2023"),
            ("month_dmy", "21 December 2023"),
            ("slash_mdy", "12/21/23"),
            ("slash_ymd", "2023/12/21"),
            ("dot_mdy_or_ymd", "2023.12.21"),
            ("chinese_ymd", "2023年12月21日"),
        ];

        // test us_est at midnight
        let us_est_midnight_as_utc = Utc.ymd(2023, 12, 21).and_hms(5, 0, 0);

        for &(test, input) in est_test_cases.iter() {
            assert_eq!(
                super::parse_with(input, us_est, midnight_naive).unwrap(),
                us_est_midnight_as_utc,
                "parse_with/{test}/{input}",
            )
        }

        // test us_est at 23:59:59 - UTC will be one day ahead
        let us_est_before_midnight_as_utc = Utc.ymd(2023, 12, 22).and_hms(4, 59, 59);
        for &(test, input) in est_test_cases.iter() {
            assert_eq!(
                super::parse_with(input, us_est, before_midnight_naive).unwrap(),
                us_est_before_midnight_as_utc,
                "parse_with/{test}/{input}",
            )
        }
    }

    #[test]
    fn parse_with_utc() {
        let midnight_naive = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let before_midnight_naive = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
        let utc_test_cases = vec![
            ("ymd", "2023-12-21"),
            ("ymd_z", "2023-12-21 UTC"),
            ("month_ymd", "2023-Dec-21"),
            ("month_mdy", "December 21, 2023"),
            ("month_dmy", "21 December 2023"),
            ("slash_mdy", "12/21/23"),
            ("slash_ymd", "2023/12/21"),
            ("dot_mdy_or_ymd", "2023.12.21"),
            ("chinese_ymd", "2023年12月21日"),
        ];
        // test utc at midnight
        let utc_midnight = Utc.ymd(2023, 12, 21).and_hms(0, 0, 0);

        for &(test, input) in utc_test_cases.iter() {
            assert_eq!(
                super::parse_with(input, &Utc, midnight_naive).unwrap(),
                utc_midnight,
                "parse_with/{test}/{input}",
            )
        }

        // test utc at 23:59:59
        let utc_before_midnight = Utc.ymd(2023, 12, 21).and_hms(23, 59, 59);
        for &(test, input) in utc_test_cases.iter() {
            assert_eq!(
                super::parse_with(input, &Utc, before_midnight_naive).unwrap(),
                utc_before_midnight,
                "parse_with/{test}/{input}",
            )
        }
    }

    #[test]
    fn parse_with_local() {
        let midnight_naive = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let before_midnight_naive = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
        let local_test_cases = vec![
            ("ymd", "2023-12-21"),
            ("month_ymd", "2023-Dec-21"),
            ("month_mdy", "December 21, 2023"),
            ("month_dmy", "21 December 2023"),
            ("slash_mdy", "12/21/23"),
            ("slash_ymd", "2023/12/21"),
            ("dot_mdy_or_ymd", "2023.12.21"),
            ("chinese_ymd", "2023年12月21日"),
        ];

        // test local at midnight
        let local_midnight_as_utc = Local.ymd(2023, 12, 21).and_hms(0, 0, 0).with_timezone(&Utc);

        for &(test, input) in local_test_cases.iter() {
            assert_eq!(
                super::parse_with(input, &Local, midnight_naive).unwrap(),
                local_midnight_as_utc,
                "parse_with/{test}/{input}",
            )
        }

        // test local at 23:59:59
        let local_before_midnight_as_utc = Local
            .ymd(2023, 12, 21)
            .and_hms(23, 59, 59)
            .with_timezone(&Utc);

        for &(test, input) in local_test_cases.iter() {
            assert_eq!(
                super::parse_with(input, &Local, before_midnight_naive).unwrap(),
                local_before_midnight_as_utc,
                "parse_with/{test}/{input}",
            )
        }
    }
}
