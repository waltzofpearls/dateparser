use anyhow::{anyhow, Result};
use chrono::offset::FixedOffset;

const ERR_OUT_OF_RANGE: &str = "input is out of range";
const ERR_INVALID: &str = "input contains invalid characters";
const ERR_TOO_SHORT: &str = "premature end of input";

/// Tries to parse `[-+]\d\d` continued by `\d\d`. Return FixedOffset if possible.
/// It can parse RFC 2822 legacy timezones. If offset cannot be determined, -0000 will be returned.
///
/// The additional `colon` may be used to parse a mandatory or optional `:` between hours and minutes,
/// and should return a valid FixedOffset or `Err` when parsing fails.
pub fn parse(s: &str) -> Result<FixedOffset> {
    let offset = if s.contains(':') {
        parse_offset_internal(s, colon_or_space, false)?
    } else {
        parse_offset_2822(s)?
    };
    FixedOffset::east_opt(offset).ok_or_else(|| anyhow!(ERR_OUT_OF_RANGE))
}

fn parse_offset_2822(s: &str) -> Result<i32> {
    // tries to parse legacy time zone names
    let upto = s
        .as_bytes()
        .iter()
        .position(|&c| !c.is_ascii_alphabetic())
        .unwrap_or(s.len());
    if upto > 0 {
        let name = &s[..upto];
        let offset_hours = |o| Ok(o * 3600);
        if equals(name, "gmt") || equals(name, "ut") || equals(name, "utc") {
            offset_hours(0)
        } else if equals(name, "edt") {
            offset_hours(-4)
        } else if equals(name, "est") || equals(name, "cdt") {
            offset_hours(-5)
        } else if equals(name, "cst") || equals(name, "mdt") {
            offset_hours(-6)
        } else if equals(name, "mst") || equals(name, "pdt") {
            offset_hours(-7)
        } else if equals(name, "pst") {
            offset_hours(-8)
        } else {
            Ok(0) // recommended by RFC 2822: consume but treat it as -0000
        }
    } else {
        let offset = parse_offset_internal(s, |s| Ok(s), false)?;
        Ok(offset)
    }
}

fn parse_offset_internal<F>(
    mut s: &str,
    mut consume_colon: F,
    allow_missing_minutes: bool,
) -> Result<i32>
where
    F: FnMut(&str) -> Result<&str>,
{
    let digits = |s: &str| -> Result<(u8, u8)> {
        let b = s.as_bytes();
        if b.len() < 2 {
            Err(anyhow!(ERR_TOO_SHORT))
        } else {
            Ok((b[0], b[1]))
        }
    };
    let negative = match s.as_bytes().first() {
        Some(&b'+') => false,
        Some(&b'-') => true,
        Some(_) => return Err(anyhow!(ERR_INVALID)),
        None => return Err(anyhow!(ERR_TOO_SHORT)),
    };
    s = &s[1..];

    // hours (00--99)
    let hours = match digits(s)? {
        (h1 @ b'0'..=b'9', h2 @ b'0'..=b'9') => i32::from((h1 - b'0') * 10 + (h2 - b'0')),
        _ => return Err(anyhow!(ERR_INVALID)),
    };
    s = &s[2..];

    // colons (and possibly other separators)
    s = consume_colon(s)?;

    // minutes (00--59)
    // if the next two items are digits then we have to add minutes
    let minutes = if let Ok(ds) = digits(s) {
        match ds {
            (m1 @ b'0'..=b'5', m2 @ b'0'..=b'9') => i32::from((m1 - b'0') * 10 + (m2 - b'0')),
            (b'6'..=b'9', b'0'..=b'9') => return Err(anyhow!(ERR_OUT_OF_RANGE)),
            _ => return Err(anyhow!(ERR_INVALID)),
        }
    } else if allow_missing_minutes {
        0
    } else {
        return Err(anyhow!(ERR_TOO_SHORT));
    };

    let seconds = hours * 3600 + minutes * 60;
    Ok(if negative { -seconds } else { seconds })
}

/// Returns true when two slices are equal case-insensitively (in ASCII).
/// Assumes that the `pattern` is already converted to lower case.
fn equals(s: &str, pattern: &str) -> bool {
    let mut xs = s.as_bytes().iter().map(|&c| match c {
        b'A'..=b'Z' => c + 32,
        _ => c,
    });
    let mut ys = pattern.as_bytes().iter().cloned();
    loop {
        match (xs.next(), ys.next()) {
            (None, None) => return true,
            (None, _) | (_, None) => return false,
            (Some(x), Some(y)) if x != y => return false,
            _ => (),
        }
    }
}

/// Consumes any number (including zero) of colon or spaces.
fn colon_or_space(s: &str) -> Result<&str> {
    Ok(s.trim_start_matches(|c: char| c == ':' || c.is_whitespace()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let test_cases = [
            ("-0800", FixedOffset::west(8 * 3600)),
            ("+10:00", FixedOffset::east(10 * 3600)),
            ("PST", FixedOffset::west(8 * 3600)),
            ("PDT", FixedOffset::west(7 * 3600)),
            ("UTC", FixedOffset::west(0)),
            ("GMT", FixedOffset::west(0)),
            // Seconds are ignored
            ("+23:59:59", FixedOffset::east(86400 - 60)),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(super::parse(input).unwrap(), want, "parse/{}", input);
        }

        let should_fail = [
            ("", ERR_TOO_SHORT),
            ("+", ERR_TOO_SHORT),
            ("10:00", ERR_INVALID),
            ("+24:00", ERR_OUT_OF_RANGE),
            ("-24:00", ERR_OUT_OF_RANGE),
            ("-99:00", ERR_OUT_OF_RANGE),
            ("+990000000000000000:00", ERR_OUT_OF_RANGE),
            ("-0160", ERR_OUT_OF_RANGE),
            ("-01600", ERR_OUT_OF_RANGE),
            ("+25", ERR_TOO_SHORT),
            ("+255", ERR_TOO_SHORT),
        ];
        for &(input, error_text) in should_fail.iter() {
            let res = super::parse(input);
            assert!(res.is_err(), "parse/{}", input);
            if let Err(err) = res {
                let s = format!("{err}");
                assert_eq!(s, error_text, "parse/{}", input);
            }
        }
    }
}
