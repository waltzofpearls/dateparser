use chrono::prelude::*;
use regex::Regex;

pub fn parse(input: &str) -> Option<DateTime<Utc>> {
    parse_unix_timestamp(input)
        .or_else(|| parse_unix_timestamp_millis(input))
        .or_else(|| parse_unix_timestamp_nanos(input))
        .or_else(|| parse_rfc3339(input))
        .or_else(|| parse_rfc2822(input))
        .or_else(|| parse_postgres_timestamp(input))
        .or_else(|| parse_postgres_timestamp_nanos(input))
        .or_else(|| parse_ymd_hms(input))
        .or_else(|| parse_ymd_hms_nanos(input))
        .or_else(|| parse_ymd_hms_hm_z(input))
        .or_else(|| parse_ymd(input))
        .or_else(|| parse_ymd_z(input))
        .or_else(|| parse_hms_imp(input))
        .or_else(|| parse_hms_imp_z(input))
}

// 1511648546
fn parse_unix_timestamp(input: &str) -> Option<DateTime<Utc>> {
    let re = Regex::new(r"^[0-9]{10}$").unwrap();
    if re.is_match(input) {
        return input
            .parse::<i64>()
            .ok()
            .map(|timestamp| Utc.timestamp(timestamp, 0).with_timezone(&Utc));
    }
    None
}

// 1620021848429
fn parse_unix_timestamp_millis(input: &str) -> Option<DateTime<Utc>> {
    let re = Regex::new(r"^[0-9]{13}$").unwrap();
    if re.is_match(input) {
        return input
            .parse::<i64>()
            .ok()
            .map(|timestamp| Utc.timestamp_millis(timestamp).with_timezone(&Utc));
    }
    None
}

// 1620024872717915000
fn parse_unix_timestamp_nanos(input: &str) -> Option<DateTime<Utc>> {
    let re = Regex::new(r"^[0-9]{19}$").unwrap();
    if re.is_match(input) {
        return input
            .parse::<i64>()
            .ok()
            .map(|timestamp| Utc.timestamp_nanos(timestamp).with_timezone(&Utc));
    }
    None
}

// 2021-05-01T01:17:02.604456Z
// 2017-11-25T22:34:50Z
fn parse_rfc3339(input: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(input)
        .ok()
        .map(|parsed| parsed.with_timezone(&Utc))
}

// Wed, 02 Jun 2021 06:31:39 GMT
fn parse_rfc2822(input: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc2822(input)
        .ok()
        .map(|parsed| parsed.with_timezone(&Utc))
}

// 2019-11-29 08:08:05-08
fn parse_postgres_timestamp(input: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S%#z")
        .ok()
        .map(|parsed| parsed.with_timezone(&Utc))
}

// 2021-05-02 23:31:36.0741-07
// 2021-05-02 23:31:39.12689-07
// 2019-11-29 08:15:47.624504-08
fn parse_postgres_timestamp_nanos(input: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S.%f%#z")
        .ok()
        .map(|parsed| parsed.with_timezone(&Utc))
}

// 2021-04-30 21:14:10
fn parse_ymd_hms(input: &str) -> Option<DateTime<Utc>> {
    Local
        .datetime_from_str(input, "%Y-%m-%d %H:%M:%S")
        .ok()
        .map(|parsed| parsed.with_timezone(&Utc))
}

// 2021-04-30 21:14:10.052282
fn parse_ymd_hms_nanos(input: &str) -> Option<DateTime<Utc>> {
    Local
        .datetime_from_str(input, "%Y-%m-%d %H:%M:%S.%f")
        .ok()
        .map(|parsed| parsed.with_timezone(&Utc))
}

// 2017-11-25 13:31:15 PST
// 2017-11-25 13:31 PST
fn parse_ymd_hms_hm_z(input: &str) -> Option<DateTime<Utc>> {
    let re =
        Regex::new(r"^(?P<dt>[0-9]{4}-[0-9]{2}-[0-9]{2}\s+[0-9]{2}:[0-9]{2}(:[0-9]{2})?)\s+(?P<tz>[a-zA-Z0-9]{3,4})$").unwrap();
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
                    .map(|datetime| datetime.with_timezone(&Utc));
            }
        }
    }
    None
}

// 2021-02-21
fn parse_ymd(input: &str) -> Option<DateTime<Utc>> {
    NaiveDate::parse_from_str(input, "%Y-%m-%d")
        .ok()
        .map(|parsed| parsed.and_time(Local::now().time()))
        .and_then(|datetime| Local.from_local_datetime(&datetime).single())
        .map(|local| local.with_timezone(&Utc))
}

// 2021-02-21 PST
fn parse_ymd_z(input: &str) -> Option<DateTime<Utc>> {
    let re =
        Regex::new(r"^(?P<date>[0-9]{4}-[0-9]{2}-[0-9]{2})\s+(?P<tz>[a-zA-Z0-9]{3,4})$").unwrap();
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
                    .map(|datetime| datetime.with_timezone(&Utc));
            }
        }
    }
    None
}

// 01:06:06
// 4:00pm
// 6:00 AM
fn parse_hms_imp(input: &str) -> Option<DateTime<Utc>> {
    NaiveTime::parse_from_str(input, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(input, "%I:%M%P"))
        .or_else(|_| NaiveTime::parse_from_str(input, "%I:%M %P"))
        .ok()
        .and_then(|parsed| Local::now().date().and_time(parsed))
        .map(|datetime| datetime.with_timezone(&Utc))
}

// 01:06:06 PST
// 4:00pm PST
// 6:00 AM PST
fn parse_hms_imp_z(input: &str) -> Option<DateTime<Utc>> {
    let re = Regex::new(
        r"^(?P<time>[0-9]{1,2}:[0-9]{2}(:[0-9]{2})?(\s*(am|pm|AM|PM)?))\s+(?P<tz>[a-zA-Z0-9]{3,4})$",
    )
    .unwrap();
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
                    .map(|datetime| datetime.with_timezone(&Utc));
            }
        }
    }
    None
}

fn tz_2822(tz: &str) -> String {
    let upper = tz.to_uppercase();
    match upper.as_ref() {
        "UT" | "UTC" => "GMT".to_string(),
        _ => upper,
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
