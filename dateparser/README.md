# [dateparser](https://crates.io/crates/dateparser)

[![Build Status][actions-badge]][actions-url]
[![MIT licensed][mit-badge]][mit-url]
[![Crates.io][cratesio-badge]][cratesio-url]
[![Doc.rs][docrs-badge]][docrs-url]

[actions-badge]: https://github.com/waltzofpearls/dateparser/workflows/ci/badge.svg
[actions-url]: https://github.com/waltzofpearls/dateparser/actions?query=workflow%3Aci+branch%3Amain
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/waltzofpearls/dateparser/blob/main/LICENSE
[cratesio-badge]: https://img.shields.io/crates/v/dateparser.svg
[cratesio-url]: https://crates.io/crates/dateparser
[docrs-badge]: https://docs.rs/dateparser/badge.svg
[docrs-url]: https://docs.rs/crate/dateparser/

A rust library for parsing date strings in commonly used formats. Parsed date will be returned as `chrono`'s
`DateTime<Utc>`.

## Examples

Add to your `Cargo.toml`:

```toml
[dependencies]
dateparser = "0.2.1"
```

And then use `dateparser` in your code:

```rust
use dateparser::parse;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let parsed = parse("6:15pm")?;
    println!("{:#?}", parsed);
    Ok(())
}
```

Or use `str`'s `parse` method:

```rust
use dateparser::DateTimeUtc;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let parsed = "2021-05-14 18:51 PDT".parse::<DateTimeUtc>()?.0;
    println!("{:#?}", parsed);
    Ok(())
}
```

Convert returned `DateTime<Utc>` to pacific time zone datetime with `chrono-tz`:

```toml
[dependencies]
chrono-tz = "0.6.3"
dateparser = "0.2.1"
```

```rust
use chrono_tz::US::Pacific;
use dateparser::DateTimeUtc;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let parsed = "Wed, 02 Jun 2021 06:31:39 GMT".parse::<DateTimeUtc>()?.0;
    println!("{:#?}", parsed.with_timezone(&Pacific));
    Ok(())
}
```

Parse using a custom timezone offset for a datetime string that doesn't come with a specific timezone:

```rust
use dateparser::parse_with_timezone;
use chrono::offset::{Local, Utc};
use chrono_tz::US::Pacific;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let parsed_in_local = parse_with_timezone("6:15pm", &Local)?;
    println!("{:#?}", parsed_in_local);

    let parsed_in_utc = parse_with_timezone("6:15pm", &Utc)?;
    println!("{:#?}", parsed_in_utc);

    let parsed_in_pacific = parse_with_timezone("6:15pm", &Pacific)?;
    println!("{:#?}", parsed_in_pacific);

    Ok(())
}
```

Parse with a custom timezone offset and default time when those are not given in datetime string.
By default, `parse` and `parse_with_timezone` uses `Utc::now().time()` as `default_time`.

```rust
use dateparser::parse_with;
use chrono::{
    offset::{Local, Utc},
    naive::NaiveTime,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let parsed_in_local = parse_with("2021-10-09", &Local, NaiveTime::from_hms(0, 0, 0))?;
    println!("{:#?}", parsed_in_local);

    let parsed_in_utc = parse_with("2021-10-09", &Utc, NaiveTime::from_hms(0, 0, 0))?;
    println!("{:#?}", parsed_in_utc);

    Ok(())
}
```

## Accepted date formats

```rust
// unix timestamp
"1511648546",
"1620021848429",
"1620024872717915000",
// rfc3339
"2021-05-01T01:17:02.604456Z",
"2017-11-25T22:34:50Z",
// rfc2822
"Wed, 02 Jun 2021 06:31:39 GMT",
// postgres timestamp yyyy-mm-dd hh:mm:ss z
"2019-11-29 08:08-08",
"2019-11-29 08:08:05-08",
"2021-05-02 23:31:36.0741-07",
"2021-05-02 23:31:39.12689-07",
"2019-11-29 08:15:47.624504-08",
"2017-07-19 03:21:51+00:00",
// yyyy-mm-dd hh:mm:ss
"2014-04-26 05:24:37 PM",
"2021-04-30 21:14",
"2021-04-30 21:14:10",
"2021-04-30 21:14:10.052282",
"2014-04-26 17:24:37.123",
"2014-04-26 17:24:37.3186369",
"2012-08-03 18:31:59.257000000",
// yyyy-mm-dd hh:mm:ss z
"2017-11-25 13:31:15 PST",
"2017-11-25 13:31 PST",
"2014-12-16 06:20:00 UTC",
"2014-12-16 06:20:00 GMT",
"2014-04-26 13:13:43 +0800",
"2014-04-26 13:13:44 +09:00",
"2012-08-03 18:31:59.257000000 +0000",
"2015-09-30 18:48:56.35272715 UTC",
// yyyy-mm-dd
"2021-02-21",
// yyyy-mm-dd z
"2021-02-21 PST",
"2021-02-21 UTC",
"2020-07-20+08:00",
// hh:mm:ss
"01:06:06",
"4:00pm",
"6:00 AM",
// hh:mm:ss z
"01:06:06 PST",
"4:00pm PST",
"6:00 AM PST",
"6:00pm UTC",
// Mon dd hh:mm:ss
"May 6 at 9:24 PM",
"May 27 02:45:27",
// Mon dd, yyyy, hh:mm:ss
"May 8, 2009 5:57:51 PM",
"September 17, 2012 10:09am",
"September 17, 2012, 10:10:09",
// Mon dd, yyyy hh:mm:ss z
"May 02, 2021 15:51:31 UTC",
"May 02, 2021 15:51 UTC",
"May 26, 2021, 12:49 AM PDT",
"September 17, 2012 at 10:09am PST",
// yyyy-mon-dd
"2021-Feb-21",
// Mon dd, yyyy
"May 25, 2021",
"oct 7, 1970",
"oct 7, 70",
"oct. 7, 1970",
"oct. 7, 70",
"October 7, 1970",
// dd Mon yyyy hh:mm:ss
"12 Feb 2006, 19:17",
"12 Feb 2006 19:17",
"14 May 2019 19:11:40.164",
// dd Mon yyyy
"7 oct 70",
"7 oct 1970",
"03 February 2013",
"1 July 2013",
// mm/dd/yyyy hh:mm:ss
"4/8/2014 22:05",
"04/08/2014 22:05",
"4/8/14 22:05",
"04/2/2014 03:00:51",
"8/8/1965 12:00:00 AM",
"8/8/1965 01:00:01 PM",
"8/8/1965 01:00 PM",
"8/8/1965 1:00 PM",
"8/8/1965 12:00 AM",
"4/02/2014 03:00:51",
"03/19/2012 10:11:59",
"03/19/2012 10:11:59.3186369",
// mm/dd/yyyy
"3/31/2014",
"03/31/2014",
"08/21/71",
"8/1/71",
// yyyy/mm/dd hh:mm:ss
"2014/4/8 22:05",
"2014/04/08 22:05",
"2014/04/2 03:00:51",
"2014/4/02 03:00:51",
"2012/03/19 10:11:59",
"2012/03/19 10:11:59.3186369",
// yyyy/mm/dd
"2014/3/31",
"2014/03/31",
// mm.dd.yyyy
"3.31.2014",
"03.31.2014",
"08.21.71",
// yyyy.mm.dd
"2014.03.30",
"2014.03",
// yymmdd hh:mm:ss mysql log
"171113 14:14:20",
// chinese yyyy mm dd hh mm ss
"2014年04月08日11时25分18秒",
// chinese yyyy mm dd
"2014年04月08日",
```
