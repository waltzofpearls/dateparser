use dateparser::DateTimeUtc;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let parsed = "2021-05-14 18:51 PDT".parse::<DateTimeUtc>()?.0;
    println!("{:#?}", parsed);
    Ok(())
}
