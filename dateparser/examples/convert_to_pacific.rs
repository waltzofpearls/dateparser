use chrono_tz::US::Pacific;
use dateparser::DateTimeUtc;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let parsed = "Wed, 02 Jun 2021 06:31:39 GMT".parse::<DateTimeUtc>()?.0;
    println!("{:#?}", parsed.with_timezone(&Pacific));
    Ok(())
}
