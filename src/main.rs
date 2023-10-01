mod biluppgifter;
mod ncts;
mod utils;
mod types;

use reqwest::Error;
use crate::types::CarInfo;

#[derive(Debug, PartialEq)]
pub enum Api {
    Biluppgifter,
    Ncts,
    Unsupported,
}

// A simple function to determine which API to use based on the license plate
pub fn determine_api(plate: &str) -> Api {
    if utils::SWEDEN_PATTERN.is_match(plate) {
        Api::Biluppgifter
    } else if utils::IRELAND_PATTERN.is_match(plate) || utils::IRELAND_VETERAN_PATTERN.is_match(plate) {
        Api::Ncts
    } else {
        Api::Unsupported
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_api() {
        assert_eq!(determine_api("ABC123"), Api::Biluppgifter);
        assert_eq!(determine_api("ZV275163"), Api::Ncts);
        assert_eq!(determine_api("INVALID"), Api::Unsupported);
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <license plate>", args[0]);
        std::process::exit(1);
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    let data: Result<CarInfo, Error>;
    let plate = args[1].to_uppercase();

    match determine_api(&plate) {
        Api::Biluppgifter => {
            println!("{} Found Swedish license plate", "\u{1F1F8}\u{1F1EA}");
            data = rt.block_on(biluppgifter::get(&plate));
        },
        Api::Ncts => {
            println!("{} Found Irish license plate", "\u{1F1EE}\u{1F1EA}");
            data = rt.block_on(ncts::get(&plate));
        },
        Api::Unsupported => {
            println!("🛑 Error: Unsupported / Invalid license plate");
            std::process::exit(1);
        }
    }

    // Output (kan också brytas ut till en egen funktion för att förbättra läsbarheten)
    match data {
        Ok(car_info) => print_car_info(car_info),
        Err(e) => println!("🛑 Error: {}", e),
    }
}

// Ny funktion för att skriva ut bilinformation
fn print_car_info(car_info: CarInfo) {
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
