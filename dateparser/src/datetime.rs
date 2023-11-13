#![allow(deprecated)]
use crate::timezone;
use anyhow::{anyhow, Result};
use chrono::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

/// Parse struct has methods implemented parsers for accepted formats.
pub struct Parse<'z, Tz2> {
    tz: &'z Tz2,
    default_time: Option<NaiveTime>,
}

impl<'z, Tz2> Parse<'z, Tz2>
where
    Tz2: TimeZone,
{
    /// Create a new instrance of [`Parse`] with a custom parsing timezone that handles the
    /// datetime string without time offset.
    pub fn new(tz: &'z Tz2, default_time: Option<NaiveTime>) -> Self {
        Self { tz, default_time }
    }

    /// This method tries to parse the input datetime string with a list of accepted formats. See
    /// more exmaples from [`Parse`], [`crate::parse()`] and [`crate::parse_with_timezone()`].
    pub fn parse(&self, input: &str) -> Result<DateTime<Utc>> {
        self.unix_timestamp(input)
            .or_else(|| self.rfc2822(input))
            .or_else(|| self.ymd_family(input))
            .or_else(|| self.hms_family(input))
            .or_else(|| self.month_ymd(input))
            .or_else(|| self.month_mdy_family(input))
            .or_else(|| self.month_dmy_family(input))
            .or_else(|| self.slash_mdy_family(input))
            .or_else(|| self.slash_ymd_family(input))
            .or_else(|| self.dot_mdy_or_ymd(input))
            .or_else(|| self.mysql_log_timestamp(input))
            .or_else(|| self.chinese_ymd_family(input))
            .unwrap_or_else(|| Err(anyhow!("{} did not match any formats.", input)))
    }

    fn ymd_family(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{4}-[0-9]{2}").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }
        self.rfc3339(input)
            .or_else(|| self.postgres_timestamp(input))
            .or_else(|| self.ymd_hms(input))
            .or_else(|| self.ymd_hms_z(input))
            .or_else(|| self.ymd(input))
            .or_else(|| self.ymd_z(input))
    }

    fn hms_family(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{1,2}:[0-9]{2}").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }
        self.hms(input).or_else(|| self.hms_z(input))
    }

    fn month_mdy_family(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[a-zA-Z]{3,9}\.?\s+[0-9]{1,2}").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }
        self.month_md_hms(input)
            .or_else(|| self.month_mdy_hms(input))
            .or_else(|| self.month_mdy_hms_z(input))
            .or_else(|| self.month_mdy(input))
    }

    fn month_dmy_family(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{1,2}\s+[a-zA-Z]{3,9}").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }
        self.month_dmy_hms(input).or_else(|| self.month_dmy(input))
    }

    fn slash_mdy_family(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{1,2}/[0-9]{1,2}").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }
        self.slash_mdy_hms(input).or_else(|| self.slash_mdy(input))
    }

    fn slash_ymd_family(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{4}/[0-9]{1,2}").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }
        self.slash_ymd_hms(input).or_else(|| self.slash_ymd(input))
    }

    fn chinese_ymd_family(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{4}年[0-9]{2}月").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }
        self.chinese_ymd_hms(input)
            .or_else(|| self.chinese_ymd(input))
    }

    // unix timestamp
    // - 1511648546
    // - 1620021848429
    // - 1620024872717915000
    fn unix_timestamp(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{10,19}$").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        input
            .parse::<i64>()
            .ok()
            .and_then(|timestamp| {
                match input.len() {
                    10 => Some(Utc.timestamp(timestamp, 0)),
                    13 => Some(Utc.timestamp_millis(timestamp)),
                    19 => Some(Utc.timestamp_nanos(timestamp)),
                    _ => None,
                }
                .map(|datetime| datetime.with_timezone(&Utc))
            })
            .map(Ok)
    }

    // rfc3339
    // - 2021-05-01T01:17:02.604456Z
    // - 2017-11-25T22:34:50Z
    fn rfc3339(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        DateTime::parse_from_rfc3339(input)
            .ok()
            .map(|parsed| parsed.with_timezone(&Utc))
            .map(Ok)
    }

    // rfc2822
    // - Wed, 02 Jun 2021 06:31:39 GMT
    fn rfc2822(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        DateTime::parse_from_rfc2822(input)
            .ok()
            .map(|parsed| parsed.with_timezone(&Utc))
            .map(Ok)
    }

    // postgres timestamp yyyy-mm-dd hh:mm:ss z
    // - 2019-11-29 08:08-08
    // - 2019-11-29 08:08:05-08
    // - 2021-05-02 23:31:36.0741-07
    // - 2021-05-02 23:31:39.12689-07
    // - 2019-11-29 08:15:47.624504-08
    // - 2017-07-19 03:21:51+00:00
    fn postgres_timestamp(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[0-9]{4}-[0-9]{2}-[0-9]{2}\s+[0-9]{2}:[0-9]{2}(:[0-9]{2})?(\.[0-9]{1,9})?[+-:0-9]{3,6}$",
            )
            .unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        DateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S%#z")
            .or_else(|_| DateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S%.f%#z"))
            .or_else(|_| DateTime::parse_from_str(input, "%Y-%m-%d %H:%M%#z"))
            .ok()
            .map(|parsed| parsed.with_timezone(&Utc))
            .map(Ok)
    }

    // yyyy-mm-dd hh:mm:ss
    // - 2014-04-26 05:24:37 PM
    // - 2021-04-30 21:14
    // - 2021-04-30 21:14:10
    // - 2021-04-30 21:14:10.052282
    // - 2014-04-26 17:24:37.123
    // - 2014-04-26 17:24:37.3186369
    // - 2012-08-03 18:31:59.257000000
    fn ymd_hms(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[0-9]{4}-[0-9]{2}-[0-9]{2}\s+[0-9]{2}:[0-9]{2}(:[0-9]{2})?(\.[0-9]{1,9})?\s*(am|pm|AM|PM)?$",
            )
            .unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        self.tz
            .datetime_from_str(input, "%Y-%m-%d %H:%M:%S")
            .or_else(|_| self.tz.datetime_from_str(input, "%Y-%m-%d %H:%M"))
            .or_else(|_| self.tz.datetime_from_str(input, "%Y-%m-%d %H:%M:%S%.f"))
            .or_else(|_| self.tz.datetime_from_str(input, "%Y-%m-%d %I:%M:%S %P"))
            .or_else(|_| self.tz.datetime_from_str(input, "%Y-%m-%d %I:%M %P"))
            .ok()
            .map(|parsed| parsed.with_timezone(&Utc))
            .map(Ok)
    }

    // yyyy-mm-dd hh:mm:ss z
    // - 2017-11-25 13:31:15 PST
    // - 2017-11-25 13:31 PST
    // - 2014-12-16 06:20:00 UTC
    // - 2014-12-16 06:20:00 GMT
    // - 2014-04-26 13:13:43 +0800
    // - 2014-04-26 13:13:44 +09:00
    // - 2012-08-03 18:31:59.257000000 +0000
    // - 2015-09-30 18:48:56.35272715 UTC
    fn ymd_hms_z(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[0-9]{4}-[0-9]{2}-[0-9]{2}\s+[0-9]{2}:[0-9]{2}(:[0-9]{2})?(\.[0-9]{1,9})?(?P<tz>\s*[+-:a-zA-Z0-9]{3,6})$",
            ).unwrap();
        }

        if !RE.is_match(input) {
            return None;
        }
        if let Some(caps) = RE.captures(input) {
            if let Some(matched_tz) = caps.name("tz") {
                let parse_from_str = NaiveDateTime::parse_from_str;
                return match timezone::parse(matched_tz.as_str().trim()) {
                    Ok(offset) => parse_from_str(input, "%Y-%m-%d %H:%M:%S %Z")
                        .or_else(|_| parse_from_str(input, "%Y-%m-%d %H:%M %Z"))
                        .or_else(|_| parse_from_str(input, "%Y-%m-%d %H:%M:%S%.f %Z"))
                        .ok()
                        .and_then(|parsed| offset.from_local_datetime(&parsed).single())
                        .map(|datetime| datetime.with_timezone(&Utc))
                        .map(Ok),
                    Err(err) => Some(Err(err)),
                };
            }
        }
        None
    }

    // yyyy-mm-dd
    // - 2021-02-21
    fn ymd(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$").unwrap();
        }

        if !RE.is_match(input) {
            return None;
        }

        // set time to use
        let time = match self.default_time {
            Some(v) => v,
            None => Utc::now().with_timezone(self.tz).time(),
        };

        NaiveDate::parse_from_str(input, "%Y-%m-%d")
            .ok()
            .map(|parsed| parsed.and_time(time))
            .and_then(|datetime| self.tz.from_local_datetime(&datetime).single())
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // yyyy-mm-dd z
    // - 2021-02-21 PST
    // - 2021-02-21 UTC
    // - 2020-07-20+08:00 (yyyy-mm-dd-07:00)
    fn ymd_z(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^[0-9]{4}-[0-9]{2}-[0-9]{2}(?P<tz>\s*[+-:a-zA-Z0-9]{3,6})$").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        if let Some(caps) = RE.captures(input) {
            if let Some(matched_tz) = caps.name("tz") {
                return match timezone::parse(matched_tz.as_str().trim()) {
                    Ok(offset) => {
                        // set time to use
                        let time = match self.default_time {
                            Some(v) => v,
                            None => Utc::now().with_timezone(&offset).time(),
                        };
                        NaiveDate::parse_from_str(input, "%Y-%m-%d %Z")
                            .ok()
                            .map(|parsed| parsed.and_time(time))
                            .and_then(|datetime| offset.from_local_datetime(&datetime).single())
                            .map(|at_tz| at_tz.with_timezone(&Utc))
                            .map(Ok)
                    }
                    Err(err) => Some(Err(err)),
                };
            }
        }
        None
    }

    // hh:mm:ss
    // - 01:06:06
    // - 4:00pm
    // - 6:00 AM
    fn hms(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^[0-9]{1,2}:[0-9]{2}(:[0-9]{2})?\s*(am|pm|AM|PM)?$").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        let now = Utc::now().with_timezone(self.tz);
        NaiveTime::parse_from_str(input, "%H:%M:%S")
            .or_else(|_| NaiveTime::parse_from_str(input, "%H:%M"))
            .or_else(|_| NaiveTime::parse_from_str(input, "%I:%M:%S %P"))
            .or_else(|_| NaiveTime::parse_from_str(input, "%I:%M %P"))
            .ok()
            .and_then(|parsed| now.date().and_time(parsed))
            .map(|datetime| datetime.with_timezone(&Utc))
            .map(Ok)
    }

    // hh:mm:ss z
    // - 01:06:06 PST
    // - 4:00pm PST
    // - 6:00 AM PST
    // - 6:00pm UTC
    fn hms_z(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[0-9]{1,2}:[0-9]{2}(:[0-9]{2})?\s*(am|pm|AM|PM)?(?P<tz>\s+[+-:a-zA-Z0-9]{3,6})$",
            )
            .unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        if let Some(caps) = RE.captures(input) {
            if let Some(matched_tz) = caps.name("tz") {
                return match timezone::parse(matched_tz.as_str().trim()) {
                    Ok(offset) => {
                        let now = Utc::now().with_timezone(&offset);
                        NaiveTime::parse_from_str(input, "%H:%M:%S %Z")
                            .or_else(|_| NaiveTime::parse_from_str(input, "%H:%M %Z"))
                            .or_else(|_| NaiveTime::parse_from_str(input, "%I:%M:%S %P %Z"))
                            .or_else(|_| NaiveTime::parse_from_str(input, "%I:%M %P %Z"))
                            .ok()
                            .map(|parsed| now.date().naive_local().and_time(parsed))
                            .and_then(|datetime| offset.from_local_datetime(&datetime).single())
                            .map(|at_tz| at_tz.with_timezone(&Utc))
                            .map(Ok)
                    }
                    Err(err) => Some(Err(err)),
                };
            }
        }
        None
    }

    // yyyy-mon-dd
    // - 2021-Feb-21
    fn month_ymd(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{4}-[a-zA-Z]{3,9}-[0-9]{2}$").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        // set time to use
        let time = match self.default_time {
            Some(v) => v,
            None => Utc::now().with_timezone(self.tz).time(),
        };

        NaiveDate::parse_from_str(input, "%Y-%m-%d")
            .or_else(|_| NaiveDate::parse_from_str(input, "%Y-%b-%d"))
            .ok()
            .map(|parsed| parsed.and_time(time))
            .and_then(|datetime| self.tz.from_local_datetime(&datetime).single())
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // Mon dd hh:mm:ss
    // - May 6 at 9:24 PM
    // - May 27 02:45:27
    fn month_md_hms(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[a-zA-Z]{3}\s+[0-9]{1,2}\s*(at)?\s+[0-9]{1,2}:[0-9]{2}(:[0-9]{2})?\s*(am|pm|AM|PM)?$",
            )
            .unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        let now = Utc::now().with_timezone(self.tz);
        let with_year = format!("{} {}", now.year(), input);
        self.tz
            .datetime_from_str(&with_year, "%Y %b %d at %I:%M %P")
            .or_else(|_| self.tz.datetime_from_str(&with_year, "%Y %b %d %H:%M:%S"))
            .ok()
            .map(|parsed| parsed.with_timezone(&Utc))
            .map(Ok)
    }

    // Mon dd, yyyy, hh:mm:ss
    // - May 8, 2009 5:57:51 PM
    // - September 17, 2012 10:09am
    // - September 17, 2012, 10:10:09
    fn month_mdy_hms(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[a-zA-Z]{3,9}\.?\s+[0-9]{1,2},\s+[0-9]{2,4},?\s+[0-9]{1,2}:[0-9]{2}(:[0-9]{2})?\s*(am|pm|AM|PM)?$",
            ).unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        let dt = input.replace(", ", " ").replace(". ", " ");
        self.tz
            .datetime_from_str(&dt, "%B %d %Y %H:%M:%S")
            .or_else(|_| self.tz.datetime_from_str(&dt, "%B %d %Y %H:%M"))
            .or_else(|_| self.tz.datetime_from_str(&dt, "%B %d %Y %I:%M:%S %P"))
            .or_else(|_| self.tz.datetime_from_str(&dt, "%B %d %Y %I:%M %P"))
            .ok()
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // Mon dd, yyyy hh:mm:ss z
    // - May 02, 2021 15:51:31 UTC
    // - May 02, 2021 15:51 UTC
    // - May 26, 2021, 12:49 AM PDT
    // - September 17, 2012 at 10:09am PST
    fn month_mdy_hms_z(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[a-zA-Z]{3,9}\s+[0-9]{1,2},?\s+[0-9]{4}\s*,?(at)?\s+[0-9]{2}:[0-9]{2}(:[0-9]{2})?\s*(am|pm|AM|PM)?(?P<tz>\s+[+-:a-zA-Z0-9]{3,6})$",
            ).unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        if let Some(caps) = RE.captures(input) {
            if let Some(matched_tz) = caps.name("tz") {
                let parse_from_str = NaiveDateTime::parse_from_str;
                return match timezone::parse(matched_tz.as_str().trim()) {
                    Ok(offset) => {
                        let dt = input.replace(',', "").replace("at", "");
                        parse_from_str(&dt, "%B %d %Y %H:%M:%S %Z")
                            .or_else(|_| parse_from_str(&dt, "%B %d %Y %H:%M %Z"))
                            .or_else(|_| parse_from_str(&dt, "%B %d %Y %I:%M:%S %P %Z"))
                            .or_else(|_| parse_from_str(&dt, "%B %d %Y %I:%M %P %Z"))
                            .ok()
                            .and_then(|parsed| offset.from_local_datetime(&parsed).single())
                            .map(|datetime| datetime.with_timezone(&Utc))
                            .map(Ok)
                    }
                    Err(err) => Some(Err(err)),
                };
            }
        }
        None
    }

    // Mon dd, yyyy
    // - May 25, 2021
    // - oct 7, 1970
    // - oct 7, 70
    // - oct. 7, 1970
    // - oct. 7, 70
    // - October 7, 1970
    fn month_mdy(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^[a-zA-Z]{3,9}\.?\s+[0-9]{1,2},\s+[0-9]{2,4}$").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        // set time to use
        let time = match self.default_time {
            Some(v) => v,
            None => Utc::now().with_timezone(self.tz).time(),
        };

        let dt = input.replace(", ", " ").replace(". ", " ");
        NaiveDate::parse_from_str(&dt, "%B %d %y")
            .or_else(|_| NaiveDate::parse_from_str(&dt, "%B %d %Y"))
            .ok()
            .map(|parsed| parsed.and_time(time))
            .and_then(|datetime| self.tz.from_local_datetime(&datetime).single())
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // dd Mon yyyy hh:mm:ss
    // - 12 Feb 2006, 19:17
    // - 12 Feb 2006 19:17
    // - 14 May 2019 19:11:40.164
    fn month_dmy_hms(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[0-9]{1,2}\s+[a-zA-Z]{3,9}\s+[0-9]{2,4},?\s+[0-9]{1,2}:[0-9]{2}(:[0-9]{2})?(\.[0-9]{1,9})?$",
            ).unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        let dt = input.replace(", ", " ");
        self.tz
            .datetime_from_str(&dt, "%d %B %Y %H:%M:%S")
            .or_else(|_| self.tz.datetime_from_str(&dt, "%d %B %Y %H:%M"))
            .or_else(|_| self.tz.datetime_from_str(&dt, "%d %B %Y %H:%M:%S%.f"))
            .or_else(|_| self.tz.datetime_from_str(&dt, "%d %B %Y %I:%M:%S %P"))
            .or_else(|_| self.tz.datetime_from_str(&dt, "%d %B %Y %I:%M %P"))
            .ok()
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // dd Mon yyyy
    // - 7 oct 70
    // - 7 oct 1970
    // - 03 February 2013
    // - 1 July 2013
    fn month_dmy(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^[0-9]{1,2}\s+[a-zA-Z]{3,9}\s+[0-9]{2,4}$").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        // set time to use
        let time = match self.default_time {
            Some(v) => v,
            None => Utc::now().with_timezone(self.tz).time(),
        };

        NaiveDate::parse_from_str(input, "%d %B %y")
            .or_else(|_| NaiveDate::parse_from_str(input, "%d %B %Y"))
            .ok()
            .map(|parsed| parsed.and_time(time))
            .and_then(|datetime| self.tz.from_local_datetime(&datetime).single())
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // mm/dd/yyyy hh:mm:ss
    // - 4/8/2014 22:05
    // - 04/08/2014 22:05
    // - 4/8/14 22:05
    // - 04/2/2014 03:00:51
    // - 8/8/1965 12:00:00 AM
    // - 8/8/1965 01:00:01 PM
    // - 8/8/1965 01:00 PM
    // - 8/8/1965 1:00 PM
    // - 8/8/1965 12:00 AM
    // - 4/02/2014 03:00:51
    // - 03/19/2012 10:11:59
    // - 03/19/2012 10:11:59.3186369
    fn slash_mdy_hms(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[0-9]{1,2}/[0-9]{1,2}/[0-9]{2,4}\s+[0-9]{1,2}:[0-9]{2}(:[0-9]{2})?(\.[0-9]{1,9})?\s*(am|pm|AM|PM)?$"
            )
            .unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        self.tz
            .datetime_from_str(input, "%m/%d/%y %H:%M:%S")
            .or_else(|_| self.tz.datetime_from_str(input, "%m/%d/%y %H:%M"))
            .or_else(|_| self.tz.datetime_from_str(input, "%m/%d/%y %H:%M:%S%.f"))
            .or_else(|_| self.tz.datetime_from_str(input, "%m/%d/%y %I:%M:%S %P"))
            .or_else(|_| self.tz.datetime_from_str(input, "%m/%d/%y %I:%M %P"))
            .or_else(|_| self.tz.datetime_from_str(input, "%m/%d/%Y %H:%M:%S"))
            .or_else(|_| self.tz.datetime_from_str(input, "%m/%d/%Y %H:%M"))
            .or_else(|_| self.tz.datetime_from_str(input, "%m/%d/%Y %H:%M:%S%.f"))
            .or_else(|_| self.tz.datetime_from_str(input, "%m/%d/%Y %I:%M:%S %P"))
            .or_else(|_| self.tz.datetime_from_str(input, "%m/%d/%Y %I:%M %P"))
            .ok()
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // mm/dd/yyyy
    // - 3/31/2014
    // - 03/31/2014
    // - 08/21/71
    // - 8/1/71
    fn slash_mdy(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{1,2}/[0-9]{1,2}/[0-9]{2,4}$").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        // set time to use
        let time = match self.default_time {
            Some(v) => v,
            None => Utc::now().with_timezone(self.tz).time(),
        };

        NaiveDate::parse_from_str(input, "%m/%d/%y")
            .or_else(|_| NaiveDate::parse_from_str(input, "%m/%d/%Y"))
            .ok()
            .map(|parsed| parsed.and_time(time))
            .and_then(|datetime| self.tz.from_local_datetime(&datetime).single())
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // yyyy/mm/dd hh:mm:ss
    // - 2014/4/8 22:05
    // - 2014/04/08 22:05
    // - 2014/04/2 03:00:51
    // - 2014/4/02 03:00:51
    // - 2012/03/19 10:11:59
    // - 2012/03/19 10:11:59.3186369
    fn slash_ymd_hms(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[0-9]{4}/[0-9]{1,2}/[0-9]{1,2}\s+[0-9]{1,2}:[0-9]{2}(:[0-9]{2})?(\.[0-9]{1,9})?\s*(am|pm|AM|PM)?$"
            )
            .unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        self.tz
            .datetime_from_str(input, "%Y/%m/%d %H:%M:%S")
            .or_else(|_| self.tz.datetime_from_str(input, "%Y/%m/%d %H:%M"))
            .or_else(|_| self.tz.datetime_from_str(input, "%Y/%m/%d %H:%M:%S%.f"))
            .or_else(|_| self.tz.datetime_from_str(input, "%Y/%m/%d %I:%M:%S %P"))
            .or_else(|_| self.tz.datetime_from_str(input, "%Y/%m/%d %I:%M %P"))
            .ok()
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // yyyy/mm/dd
    // - 2014/3/31
    // - 2014/03/31
    fn slash_ymd(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{4}/[0-9]{1,2}/[0-9]{1,2}$").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        // set time to use
        let time = match self.default_time {
            Some(v) => v,
            None => Utc::now().with_timezone(self.tz).time(),
        };

        NaiveDate::parse_from_str(input, "%Y/%m/%d")
            .ok()
            .map(|parsed| parsed.and_time(time))
            .and_then(|datetime| self.tz.from_local_datetime(&datetime).single())
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // mm.dd.yyyy
    // - 3.31.2014
    // - 03.31.2014
    // - 08.21.71
    // yyyy.mm.dd
    // - 2014.03.30
    // - 2014.03
    fn dot_mdy_or_ymd(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[0-9]{1,4}.[0-9]{1,4}[0-9]{1,4}").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        // set time to use
        let time = match self.default_time {
            Some(v) => v,
            None => Utc::now().with_timezone(self.tz).time(),
        };

        NaiveDate::parse_from_str(input, "%m.%d.%y")
            .or_else(|_| NaiveDate::parse_from_str(input, "%m.%d.%Y"))
            .or_else(|_| NaiveDate::parse_from_str(input, "%Y.%m.%d"))
            .or_else(|_| {
                NaiveDate::parse_from_str(&format!("{}.{}", input, Utc::now().day()), "%Y.%m.%d")
            })
            .ok()
            .map(|parsed| parsed.and_time(time))
            .and_then(|datetime| self.tz.from_local_datetime(&datetime).single())
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // yymmdd hh:mm:ss mysql log
    // - 171113 14:14:20
    fn mysql_log_timestamp(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[0-9]{6}\s+[0-9]{2}:[0-9]{2}:[0-9]{2}").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        self.tz
            .datetime_from_str(input, "%y%m%d %H:%M:%S")
            .ok()
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // chinese yyyy mm dd hh mm ss
    // - 2014年04月08日11时25分18秒
    fn chinese_ymd_hms(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^[0-9]{4}年[0-9]{2}月[0-9]{2}日[0-9]{2}时[0-9]{2}分[0-9]{2}秒$")
                    .unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        self.tz
            .datetime_from_str(input, "%Y年%m月%d日%H时%M分%S秒")
            .ok()
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // chinese yyyy mm dd
    // - 2014年04月08日
    fn chinese_ymd(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{4}年[0-9]{2}月[0-9]{2}日$").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        // set time to use
        let time = match self.default_time {
            Some(v) => v,
            None => Utc::now().with_timezone(self.tz).time(),
        };

        NaiveDate::parse_from_str(input, "%Y年%m月%d日")
            .ok()
            .map(|parsed| parsed.and_time(time))
            .and_then(|datetime| self.tz.from_local_datetime(&datetime).single())
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix_timestamp() {
        let parse = Parse::new(&Utc, None);

        let test_cases = [
            ("0000000000", Utc.ymd(1970, 1, 1).and_hms(0, 0, 0)),
            ("0000000000000", Utc.ymd(1970, 1, 1).and_hms(0, 0, 0)),
            ("0000000000000000000", Utc.ymd(1970, 1, 1).and_hms(0, 0, 0)),
            ("1511648546", Utc.ymd(2017, 11, 25).and_hms(22, 22, 26)),
            (
                "1620021848429",
                Utc.ymd(2021, 5, 3).and_hms_milli(6, 4, 8, 429),
            ),
            (
                "1620024872717915000",
                Utc.ymd(2021, 5, 3).and_hms_nano(6, 54, 32, 717915000),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.unix_timestamp(input).unwrap().unwrap(),
                want,
                "unix_timestamp/{}",
                input
            )
        }
        assert!(parse.unix_timestamp("15116").is_none());
        assert!(parse
            .unix_timestamp("16200248727179150001620024872717915000")
            .is_none());
        assert!(parse.unix_timestamp("not-a-ts").is_none());
    }

    #[test]
    fn rfc3339() {
        let parse = Parse::new(&Utc, None);

        let test_cases = [
            (
                "2021-05-01T01:17:02.604456Z",
                Utc.ymd(2021, 5, 1).and_hms_nano(1, 17, 2, 604456000),
            ),
            (
                "2017-11-25T22:34:50Z",
                Utc.ymd(2017, 11, 25).and_hms(22, 34, 50),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.rfc3339(input).unwrap().unwrap(),
                want,
                "rfc3339/{}",
                input
            )
        }
        assert!(parse.rfc3339("2017-11-25 22:34:50").is_none());
        assert!(parse.rfc3339("not-date-time").is_none());
    }

    #[test]
    fn rfc2822() {
        let parse = Parse::new(&Utc, None);

        let test_cases = [
            (
                "Wed, 02 Jun 2021 06:31:39 GMT",
                Utc.ymd(2021, 6, 2).and_hms(6, 31, 39),
            ),
            (
                "Wed, 02 Jun 2021 06:31:39 PDT",
                Utc.ymd(2021, 6, 2).and_hms(13, 31, 39),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.rfc2822(input).unwrap().unwrap(),
                want,
                "rfc2822/{}",
                input
            )
        }
        assert!(parse.rfc2822("02 Jun 2021 06:31:39").is_none());
        assert!(parse.rfc2822("not-date-time").is_none());
    }

    #[test]
    fn postgres_timestamp() {
        let parse = Parse::new(&Utc, None);

        let test_cases = [
            (
                "2019-11-29 08:08-08",
                Utc.ymd(2019, 11, 29).and_hms(16, 8, 0),
            ),
            (
                "2019-11-29 08:08:05-08",
                Utc.ymd(2019, 11, 29).and_hms(16, 8, 5),
            ),
            (
                "2021-05-02 23:31:36.0741-07",
                Utc.ymd(2021, 5, 3).and_hms_micro(6, 31, 36, 74100),
            ),
            (
                "2021-05-02 23:31:39.12689-07",
                Utc.ymd(2021, 5, 3).and_hms_micro(6, 31, 39, 126890),
            ),
            (
                "2019-11-29 08:15:47.624504-08",
                Utc.ymd(2019, 11, 29).and_hms_micro(16, 15, 47, 624504),
            ),
            (
                "2017-07-19 03:21:51+00:00",
                Utc.ymd(2017, 7, 19).and_hms(3, 21, 51),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.postgres_timestamp(input).unwrap().unwrap(),
                want,
                "postgres_timestamp/{}",
                input
            )
        }
        assert!(parse.postgres_timestamp("not-date-time").is_none());
    }

    #[test]
    fn ymd_hms() {
        let parse = Parse::new(&Utc, None);

        let test_cases = vec![
            ("2021-04-30 21:14", Utc.ymd(2021, 4, 30).and_hms(21, 14, 0)),
            (
                "2021-04-30 21:14:10",
                Utc.ymd(2021, 4, 30).and_hms(21, 14, 10),
            ),
            (
                "2021-04-30 21:14:10.052282",
                Utc.ymd(2021, 4, 30).and_hms_micro(21, 14, 10, 52282),
            ),
            (
                "2014-04-26 05:24:37 PM",
                Utc.ymd(2014, 4, 26).and_hms(17, 24, 37),
            ),
            (
                "2014-04-26 17:24:37.123",
                Utc.ymd(2014, 4, 26).and_hms_milli(17, 24, 37, 123),
            ),
            (
                "2014-04-26 17:24:37.3186369",
                Utc.ymd(2014, 4, 26).and_hms_nano(17, 24, 37, 318636900),
            ),
            (
                "2012-08-03 18:31:59.257000000",
                Utc.ymd(2012, 8, 3).and_hms_nano(18, 31, 59, 257000000),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.ymd_hms(input).unwrap().unwrap(),
                want,
                "ymd_hms/{}",
                input
            )
        }
        assert!(parse.ymd_hms("not-date-time").is_none());
    }

    #[test]
    fn ymd_hms_z() {
        let parse = Parse::new(&Utc, None);

        let test_cases = vec![
            (
                "2017-11-25 13:31:15 PST",
                Utc.ymd(2017, 11, 25).and_hms(21, 31, 15),
            ),
            (
                "2017-11-25 13:31 PST",
                Utc.ymd(2017, 11, 25).and_hms(21, 31, 0),
            ),
            (
                "2014-12-16 06:20:00 UTC",
                Utc.ymd(2014, 12, 16).and_hms(6, 20, 0),
            ),
            (
                "2014-12-16 06:20:00 GMT",
                Utc.ymd(2014, 12, 16).and_hms(6, 20, 0),
            ),
            (
                "2014-04-26 13:13:43 +0800",
                Utc.ymd(2014, 4, 26).and_hms(5, 13, 43),
            ),
            (
                "2014-04-26 13:13:44 +09:00",
                Utc.ymd(2014, 4, 26).and_hms(4, 13, 44),
            ),
            (
                "2012-08-03 18:31:59.257000000 +0000",
                Utc.ymd(2012, 8, 3).and_hms_nano(18, 31, 59, 257000000),
            ),
            (
                "2015-09-30 18:48:56.35272715 UTC",
                Utc.ymd(2015, 9, 30).and_hms_nano(18, 48, 56, 352727150),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.ymd_hms_z(input).unwrap().unwrap(),
                want,
                "ymd_hms_z/{}",
                input
            )
        }
        assert!(parse.ymd_hms_z("not-date-time").is_none());
    }

    #[test]
    fn ymd() {
        let parse = Parse::new(&Utc, Some(Utc::now().time()));

        let test_cases = [(
            "2021-02-21",
            Utc.ymd(2021, 2, 21).and_time(Utc::now().time()),
        )];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse
                    .ymd(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "ymd/{}",
                input
            )
        }
        assert!(parse.ymd("not-date-time").is_none());
    }

    #[test]
    fn ymd_z() {
        let parse = Parse::new(&Utc, None);
        let now_at_pst = Utc::now().with_timezone(&FixedOffset::west(8 * 3600));
        let now_at_cst = Utc::now().with_timezone(&FixedOffset::east(8 * 3600));

        let test_cases = [
            (
                "2021-02-21 PST",
                FixedOffset::west(8 * 3600)
                    .ymd(2021, 2, 21)
                    .and_time(now_at_pst.time())
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
            (
                "2021-02-21 UTC",
                FixedOffset::west(0)
                    .ymd(2021, 2, 21)
                    .and_time(Utc::now().time())
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
            (
                "2020-07-20+08:00",
                FixedOffset::east(8 * 3600)
                    .ymd(2020, 7, 20)
                    .and_time(now_at_cst.time())
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse
                    .ymd_z(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "ymd_z/{}",
                input
            )
        }
        assert!(parse.ymd_z("not-date-time").is_none());
    }

    #[test]
    fn hms() {
        let parse = Parse::new(&Utc, None);

        let test_cases = [
            (
                "01:06:06",
                Utc::now().date().and_time(NaiveTime::from_hms(1, 6, 6)),
            ),
            (
                "4:00pm",
                Utc::now().date().and_time(NaiveTime::from_hms(16, 0, 0)),
            ),
            (
                "6:00 AM",
                Utc::now().date().and_time(NaiveTime::from_hms(6, 0, 0)),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.hms(input).unwrap().unwrap(),
                want.unwrap(),
                "hms/{}",
                input
            )
        }
        assert!(parse.hms("not-date-time").is_none());
    }

    #[test]
    fn hms_z() {
        let parse = Parse::new(&Utc, None);
        let now_at_pst = Utc::now().with_timezone(&FixedOffset::west(8 * 3600));

        let test_cases = [
            (
                "01:06:06 PST",
                FixedOffset::west(8 * 3600)
                    .from_local_date(&now_at_pst.date().naive_local())
                    .and_time(NaiveTime::from_hms(1, 6, 6))
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
            (
                "4:00pm PST",
                FixedOffset::west(8 * 3600)
                    .from_local_date(&now_at_pst.date().naive_local())
                    .and_time(NaiveTime::from_hms(16, 0, 0))
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
            (
                "6:00 AM PST",
                FixedOffset::west(8 * 3600)
                    .from_local_date(&now_at_pst.date().naive_local())
                    .and_time(NaiveTime::from_hms(6, 0, 0))
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
            (
                "6:00pm UTC",
                FixedOffset::west(0)
                    .from_local_date(&Utc::now().date().naive_local())
                    .and_time(NaiveTime::from_hms(18, 0, 0))
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.hms_z(input).unwrap().unwrap(),
                want.unwrap(),
                "hms_z/{}",
                input
            )
        }
        assert!(parse.hms_z("not-date-time").is_none());
    }

    #[test]
    fn month_ymd() {
        let parse = Parse::new(&Utc, None);

        let test_cases = [(
            "2021-Feb-21",
            Utc.ymd(2021, 2, 21).and_time(Utc::now().time()),
        )];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse
                    .month_ymd(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "month_ymd/{}",
                input
            )
        }
        assert!(parse.month_ymd("not-date-time").is_none());
    }

    #[test]
    fn month_md_hms() {
        let parse = Parse::new(&Utc, None);

        let test_cases = [
            (
                "May 6 at 9:24 PM",
                Utc.ymd(Utc::now().year(), 5, 6).and_hms(21, 24, 0),
            ),
            (
                "May 27 02:45:27",
                Utc.ymd(Utc::now().year(), 5, 27).and_hms(2, 45, 27),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.month_md_hms(input).unwrap().unwrap(),
                want,
                "month_md_hms/{}",
                input
            )
        }
        assert!(parse.month_md_hms("not-date-time").is_none());
    }

    #[test]
    fn month_mdy_hms() {
        let parse = Parse::new(&Utc, None);

        let test_cases = [
            (
                "May 8, 2009 5:57:51 PM",
                Utc.ymd(2009, 5, 8).and_hms(17, 57, 51),
            ),
            (
                "September 17, 2012 10:09am",
                Utc.ymd(2012, 9, 17).and_hms(10, 9, 0),
            ),
            (
                "September 17, 2012, 10:10:09",
                Utc.ymd(2012, 9, 17).and_hms(10, 10, 9),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.month_mdy_hms(input).unwrap().unwrap(),
                want,
                "month_mdy_hms/{}",
                input
            )
        }
        assert!(parse.month_mdy_hms("not-date-time").is_none());
    }

    #[test]
    fn month_mdy_hms_z() {
        let parse = Parse::new(&Utc, None);

        let test_cases = [
            (
                "May 02, 2021 15:51:31 UTC",
                Utc.ymd(2021, 5, 2).and_hms(15, 51, 31),
            ),
            (
                "May 02, 2021 15:51 UTC",
                Utc.ymd(2021, 5, 2).and_hms(15, 51, 0),
            ),
            (
                "May 26, 2021, 12:49 AM PDT",
                Utc.ymd(2021, 5, 26).and_hms(7, 49, 0),
            ),
            (
                "September 17, 2012 at 10:09am PST",
                Utc.ymd(2012, 9, 17).and_hms(18, 9, 0),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.month_mdy_hms_z(input).unwrap().unwrap(),
                want,
                "month_mdy_hms_z/{}",
                input
            )
        }
        assert!(parse.month_mdy_hms_z("not-date-time").is_none());
    }

    #[test]
    fn month_mdy() {
        let parse = Parse::new(&Utc, None);

        let test_cases = [
            (
                "May 25, 2021",
                Utc.ymd(2021, 5, 25).and_time(Utc::now().time()),
            ),
            (
                "oct 7, 1970",
                Utc.ymd(1970, 10, 7).and_time(Utc::now().time()),
            ),
            (
                "oct 7, 70",
                Utc.ymd(1970, 10, 7).and_time(Utc::now().time()),
            ),
            (
                "oct. 7, 1970",
                Utc.ymd(1970, 10, 7).and_time(Utc::now().time()),
            ),
            (
                "oct. 7, 70",
                Utc.ymd(1970, 10, 7).and_time(Utc::now().time()),
            ),
            (
                "October 7, 1970",
                Utc.ymd(1970, 10, 7).and_time(Utc::now().time()),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse
                    .month_mdy(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "month_mdy/{}",
                input
            )
        }
        assert!(parse.month_mdy("not-date-time").is_none());
    }

    #[test]
    fn month_dmy_hms() {
        let parse = Parse::new(&Utc, None);

        let test_cases = [
            (
                "12 Feb 2006, 19:17",
                Utc.ymd(2006, 2, 12).and_hms(19, 17, 0),
            ),
            ("12 Feb 2006 19:17", Utc.ymd(2006, 2, 12).and_hms(19, 17, 0)),
            (
                "14 May 2019 19:11:40.164",
                Utc.ymd(2019, 5, 14).and_hms_milli(19, 11, 40, 164),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.month_dmy_hms(input).unwrap().unwrap(),
                want,
                "month_dmy_hms/{}",
                input
            )
        }
        assert!(parse.month_dmy_hms("not-date-time").is_none());
    }

    #[test]
    fn month_dmy() {
        let parse = Parse::new(&Utc, None);

        let test_cases = [
            ("7 oct 70", Utc.ymd(1970, 10, 7).and_time(Utc::now().time())),
            (
                "7 oct 1970",
                Utc.ymd(1970, 10, 7).and_time(Utc::now().time()),
            ),
            (
                "03 February 2013",
                Utc.ymd(2013, 2, 3).and_time(Utc::now().time()),
            ),
            (
                "1 July 2013",
                Utc.ymd(2013, 7, 1).and_time(Utc::now().time()),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse
                    .month_dmy(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "month_dmy/{}",
                input
            )
        }
        assert!(parse.month_dmy("not-date-time").is_none());
    }

    #[test]
    fn slash_mdy_hms() {
        let parse = Parse::new(&Utc, None);

        let test_cases = vec![
            ("4/8/2014 22:05", Utc.ymd(2014, 4, 8).and_hms(22, 5, 0)),
            ("04/08/2014 22:05", Utc.ymd(2014, 4, 8).and_hms(22, 5, 0)),
            ("4/8/14 22:05", Utc.ymd(2014, 4, 8).and_hms(22, 5, 0)),
            ("04/2/2014 03:00:51", Utc.ymd(2014, 4, 2).and_hms(3, 0, 51)),
            ("8/8/1965 12:00:00 AM", Utc.ymd(1965, 8, 8).and_hms(0, 0, 0)),
            (
                "8/8/1965 01:00:01 PM",
                Utc.ymd(1965, 8, 8).and_hms(13, 0, 1),
            ),
            ("8/8/1965 01:00 PM", Utc.ymd(1965, 8, 8).and_hms(13, 0, 0)),
            ("8/8/1965 1:00 PM", Utc.ymd(1965, 8, 8).and_hms(13, 0, 0)),
            ("8/8/1965 12:00 AM", Utc.ymd(1965, 8, 8).and_hms(0, 0, 0)),
            ("4/02/2014 03:00:51", Utc.ymd(2014, 4, 2).and_hms(3, 0, 51)),
            (
                "03/19/2012 10:11:59",
                Utc.ymd(2012, 3, 19).and_hms(10, 11, 59),
            ),
            (
                "03/19/2012 10:11:59.3186369",
                Utc.ymd(2012, 3, 19).and_hms_nano(10, 11, 59, 318636900),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.slash_mdy_hms(input).unwrap().unwrap(),
                want,
                "slash_mdy_hms/{}",
                input
            )
        }
        assert!(parse.slash_mdy_hms("not-date-time").is_none());
    }

    #[test]
    fn slash_mdy() {
        let parse = Parse::new(&Utc, None);

        let test_cases = [
            (
                "3/31/2014",
                Utc.ymd(2014, 3, 31).and_time(Utc::now().time()),
            ),
            (
                "03/31/2014",
                Utc.ymd(2014, 3, 31).and_time(Utc::now().time()),
            ),
            ("08/21/71", Utc.ymd(1971, 8, 21).and_time(Utc::now().time())),
            ("8/1/71", Utc.ymd(1971, 8, 1).and_time(Utc::now().time())),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse
                    .slash_mdy(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "slash_mdy/{}",
                input
            )
        }
        assert!(parse.slash_mdy("not-date-time").is_none());
    }

    #[test]
    fn slash_ymd_hms() {
        let parse = Parse::new(&Utc, None);

        let test_cases = [
            ("2014/4/8 22:05", Utc.ymd(2014, 4, 8).and_hms(22, 5, 0)),
            ("2014/04/08 22:05", Utc.ymd(2014, 4, 8).and_hms(22, 5, 0)),
            ("2014/04/2 03:00:51", Utc.ymd(2014, 4, 2).and_hms(3, 0, 51)),
            ("2014/4/02 03:00:51", Utc.ymd(2014, 4, 2).and_hms(3, 0, 51)),
            (
                "2012/03/19 10:11:59",
                Utc.ymd(2012, 3, 19).and_hms(10, 11, 59),
            ),
            (
                "2012/03/19 10:11:59.3186369",
                Utc.ymd(2012, 3, 19).and_hms_nano(10, 11, 59, 318636900),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.slash_ymd_hms(input).unwrap().unwrap(),
                want,
                "slash_ymd_hms/{}",
                input
            )
        }
        assert!(parse.slash_ymd_hms("not-date-time").is_none());
    }

    #[test]
    fn slash_ymd() {
        let parse = Parse::new(&Utc, Some(Utc::now().time()));

        let test_cases = [
            (
                "2014/3/31",
                Utc.ymd(2014, 3, 31).and_time(Utc::now().time()),
            ),
            (
                "2014/03/31",
                Utc.ymd(2014, 3, 31).and_time(Utc::now().time()),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse
                    .slash_ymd(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "slash_ymd/{}",
                input
            )
        }
        assert!(parse.slash_ymd("not-date-time").is_none());
    }

    #[test]
    fn dot_mdy_or_ymd() {
        let parse = Parse::new(&Utc, Some(Utc::now().time()));

        let test_cases = [
            // mm.dd.yyyy
            (
                "3.31.2014",
                Utc.ymd(2014, 3, 31).and_time(Utc::now().time()),
            ),
            (
                "03.31.2014",
                Utc.ymd(2014, 3, 31).and_time(Utc::now().time()),
            ),
            ("08.21.71", Utc.ymd(1971, 8, 21).and_time(Utc::now().time())),
            // yyyy.mm.dd
            (
                "2014.03.30",
                Utc.ymd(2014, 3, 30).and_time(Utc::now().time()),
            ),
            (
                "2014.03",
                Utc.ymd(2014, 3, Utc::now().day())
                    .and_time(Utc::now().time()),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse
                    .dot_mdy_or_ymd(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "dot_mdy_or_ymd/{}",
                input
            )
        }
        assert!(parse.dot_mdy_or_ymd("not-date-time").is_none());
    }

    #[test]
    fn mysql_log_timestamp() {
        let parse = Parse::new(&Utc, None);

        let test_cases = [
            // yymmdd hh:mm:ss mysql log
            ("171113 14:14:20", Utc.ymd(2017, 11, 13).and_hms(14, 14, 20)),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.mysql_log_timestamp(input).unwrap().unwrap(),
                want,
                "mysql_log_timestamp/{}",
                input
            )
        }
        assert!(parse.mysql_log_timestamp("not-date-time").is_none());
    }

    #[test]
    fn chinese_ymd_hms() {
        let parse = Parse::new(&Utc, None);

        let test_cases = [(
            "2014年04月08日11时25分18秒",
            Utc.ymd(2014, 4, 8).and_hms(11, 25, 18),
        )];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.chinese_ymd_hms(input).unwrap().unwrap(),
                want,
                "chinese_ymd_hms/{}",
                input
            )
        }
        assert!(parse.chinese_ymd_hms("not-date-time").is_none());
    }

    #[test]
    fn chinese_ymd() {
        let parse = Parse::new(&Utc, Some(Utc::now().time()));

        let test_cases = [(
            "2014年04月08日",
            Utc.ymd(2014, 4, 8).and_time(Utc::now().time()),
        )];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse
                    .chinese_ymd(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "chinese_ymd/{}",
                input
            )
        }
        assert!(parse.chinese_ymd("not-date-time").is_none());
    }
}
