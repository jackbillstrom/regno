use lazy_static::lazy_static;
use regex::Regex;
use scraper::Selector;
use crate::types::CarInfo;


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


// Helper function to print the car info
pub(crate) fn print_car_info(car_info: CarInfo) {
    println!("🆔 PLATE:     {}", car_info.registration_number);
    println!("🏭 MAKE:      {}", car_info.make);
    println!("🔧 MODEL:     {}", car_info.model);
    println!("📅 YEAR:      {}", car_info.year);
    println!("⛽  FUEL:      {:?}", car_info.fuel_type);
    println!("🔠 VIN:       {}", car_info.vin);
    println!("🟢 STATUS:    {}", car_info.vehicle_status);
    println!("⚙️  HP:       {}", if car_info.horsepower.is_empty() { "N/A" } else { &car_info.horsepower });
    println!("👤 OWNER:     {}", car_info.owner);
    println!("🚦 MSG:       {}", car_info.special_note.unwrap_or("N/A".to_string()));
    println!("🌍 ECO:       {}", car_info.environmental_class.unwrap_or("N/A".to_string()));
    println!("📆 NEXT INS:  {}", car_info.next_inspection_date.unwrap_or("N/A".to_string()));
}
