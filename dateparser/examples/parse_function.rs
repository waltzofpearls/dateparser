use dateparser::parse;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let parsed = parse("6:15pm")?;
    println!("{:#?}", parsed);
    Ok(())
}
