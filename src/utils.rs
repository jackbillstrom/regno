use lazy_static::lazy_static;
use regex::Regex;
use scraper::Selector;
use crate::types::CarInfo;


lazy_static! {
    pub static ref SWEDEN_PATTERN: Regex = Regex::new(r"^[A-Z]{3}[0-9]{2}[A-Z0-9]$").unwrap();
    pub static ref NORWAY_PATTERN: Regex = Regex::new(r"^[A-Z]{2}[0-9]{5}$").unwrap(); // See: https://regnr.no/DP82347
    pub static ref IRELAND_PATTERN: Regex = Regex::new(r"^\d{2}-?[A-Z]{1,2}-?\d{1,4}$").unwrap(); // See: https://www.ncts.ie/
    pub static ref IRELAND_VETERAN_PATTERN: Regex = Regex::new(r"^ZV\d{4,6}$").unwrap(); // See: https://www.ncts.ie/
    pub static ref UK_PATTERN: Regex = Regex::new(r"^(?:[A-Z]{3}[0-9]{1,3})|(?:[A-Z]{1,2}[0-9]{1,4})|(?:[A-Z]{1,2}[0-9][A-Z])|(?:[A-Z]{1,3}[0-9]{1,3}[A-Z])$").unwrap();
    // USA https://tagnap.com/plates/KUQG86-FL
    // UK https://www.check-mot.service.gov.uk/
    // NZ https://www.carjam.co.nz/car/?plate=GGD182&__cf_chl_tk=l5ikhfMnzE7AAndYkFbezyxgjojd1NygcOftwFZ.KvU-1696181861-0-gaNycGzNCns (HARD?)
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

    if !car_info.make.is_empty() {
        println!("🏭 MAKE:      {}", car_info.make);
    }

    if !car_info.model.is_empty() {
        println!("🔧 MODEL:     {}", car_info.model);
    }

    if car_info.year != 0 {
        println!("📅 YEAR:      {}", car_info.year);
    }

    if !car_info.stolen.is_empty() {
        println!("🚔 STOLEN:    {}", car_info.stolen);
    }

    if !car_info.vin.is_empty() {
        println!("🔠 VIN:       {}", car_info.vin);
    }

    if !car_info.owner.is_empty() {
        println!("👤 OWNER:     {}", car_info.owner);
    }

    if car_info.number_of_owners != 0 {
        println!("👥 OWNERS:    {}", car_info.number_of_owners);
    }

    if !car_info.annual_mileage.is_empty() {
        println!("🛣️ MILEAGE:  {}", car_info.annual_mileage);
    }

    println!("⛽  FUEL:      {:?}", car_info.fuel_type);
    println!("🔀 TRANS:     {:?}", car_info.transmission);

    if let Some(env_class) = &car_info.environmental_class {
        println!("🌍 ECO:       {}", env_class);
    }

    if let Some(first_reg) = &car_info.first_registration_date {
        println!("📆 FIRST REG: {}", first_reg);
    }

    if !car_info.horsepower.is_empty() {
        println!("⚙️  HP:       {}", car_info.horsepower);
    }

    if !car_info.fuel_consumption.is_empty() {
        println!("🛢️ FUEL CONS: {}", car_info.fuel_consumption);
    }

    if let Some(last_ins) = &car_info.last_inspection_date {
        println!("🔍 LAST INS:  {}", last_ins);
    }

    if let Some(next_ins) = &car_info.next_inspection_date {
        println!("📆 NEXT INS:  {}", next_ins);
    }

    if !car_info.annual_tax.is_empty() {
        println!("💰 TAX:       {}", car_info.annual_tax);
    }

    if !car_info.vehicle_status.is_empty() {
        println!("🟢 STATUS:    {}", car_info.vehicle_status);
    }

    if !car_info.chassi.is_empty() {
        println!("🛠️ CHASSI:   {}", car_info.chassi);
    }

    if !car_info.history.is_empty() {
        println!("📜 HISTORY:   {:?}", car_info.history.join(", "));
    }

    if !car_info.color.is_empty() {
        println!("🎨 COLOR:     {}", car_info.color);
    }

    if let Some(note) = &car_info.special_note {
        println!("🚦 MSG:       {}", note);
    }
}
