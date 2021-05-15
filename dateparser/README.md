# dateparser

[![Build Status][actions-badge]][actions-url]
[![MIT licensed][mit-badge]][mit-url]
[![Crate][crate-badge]][crate-url]

[actions-badge]: https://github.com/waltzofpearls/belt/workflows/ci/badge.svg
[actions-url]: https://github.com/waltzofpearls/belt/actions?query=workflow%3Aci+branch%3Amain
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/waltzofpearls/belt/blob/main/LICENSE
[crate-badge]: https://img.shields.io/crates/v/dateparser.svg
[crate-url]: https://crates.io/crates/dateparser

A rust library for parsing date strings in commonly used formats. Parsed date will be returned as `chrono`'s
`DateTime<Utc>`.

## Examples

Add to your `Cargo.toml`:

```toml
[dependencies]
dateparser = "0.1.0"
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
chrono-tz = "0.5.3"
dateparser = "0.1.0"
```

```rust
use chrono_tz::US::Pacific;
use dateparser::DateTimeUtc;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let parsed = "2021-05-14 18:51 PDT".parse::<DateTimeUtc>()?.0;
    println!("{:#?}", parsed.with_timezone(&Pacific));
    Ok(())
}
```

### Accepted date formats

```
1511648546
1620021848429
1620024872717915000
2021-05-01T01:17:02.604456Z
2017-11-25T22:34:50Z
Wed, 02 Jun 2021 06:31:39 GMT
2019-11-29 08:08:05-08
2021-05-02 23:31:36.0741-07
2021-05-02 23:31:39.12689-07
2019-11-29 08:15:47.624504-08
2021-04-30 21:14:10
2021-04-30 21:14:10.052282
2017-11-25 13:31:15 PST
2017-11-25 13:31 PST
2021-02-21
2021-02-21 PST
01:06:06
4:00pm
6:00 AM
01:06:06 PST
4:00pm PST
6:00 AM PST
May 02, 2021 15:51:31 UTC
May 02, 2021 15:51 UTC
```
