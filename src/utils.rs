use lazy_static::lazy_static;
use regex::Regex;
use scraper::Selector;


lazy_static! {
    pub static ref SWEDEN_PATTERN: Regex = Regex::new(r"^[A-Z]{3}[0-9]{2}[A-Z0-9]$").unwrap();
    pub static ref NORWAY_PATTERN: Regex = Regex::new(r"^[A-Z]{2}[0-9]{5}$").unwrap(); // See: https://www.vegvesen.no/kjoretoy/kjop-og-salg/kjoretoyopplysninger/sjekk-kjoretoyopplysninger/?registreringsnummer=DN47282
    pub static ref IRELAND_PATTERN: Regex = Regex::new(r"^\d{2}-?[A-Z]{1,2}-?\d{1,4}$").unwrap(); // See: https://www.ncts.ie/
    pub static ref IRELAND_VETERAN_PATTERN: Regex = Regex::new(r"^ZV\d{4,6}$").unwrap(); // See: https://www.ncts.ie/
    pub static ref UK_PATTERN: Regex = Regex::new(r"^(?:[A-Z]{3}[0-9]{1,3})|(?:[A-Z]{1,2}[0-9]{1,4})|(?:[A-Z]{1,2}[0-9][A-Z])|(?:[A-Z]{1,3}[0-9]{1,3}[A-Z])$").unwrap();
}

// A helper function to extract text based on a selector
pub fn extract_text(element: &scraper::ElementRef, selector: &str) -> String {
    element.select(&Selector::parse(selector).unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<String>()
        .trim()
        .to_string()
}