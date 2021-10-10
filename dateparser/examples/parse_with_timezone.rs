use chrono::offset::{Local, Utc};
use chrono_tz::US::Pacific;
use dateparser::parse_with_timezone;
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
