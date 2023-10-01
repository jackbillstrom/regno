mod biluppgifter;
mod utils;

use reqwest::Error;
use biluppgifter::types::CarInfo;

fn main() {
    // Get the command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <license plate>", args[0]);
        std::process::exit(1);
    }

    // Init the tokio runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    let data: Result<CarInfo, Error>;

    // Identify the license plate
    let plate = args[1].to_uppercase();
    match plate {
        // Get data for Swedish license plates
        _ if utils::SWEDEN_PATTERN.is_match(&plate) => {
            println!("{} Found Swedish license plate", "\u{1F1F8}\u{1F1EA}");

            data = rt.block_on(biluppgifter::get(&plate));
        },
        // TODO: Add matches for more countries
        _ => {
            println!("🛑 Error: Unsupported / Invalid license plate");
            std::process::exit(1);
        }
    }

    // Output
    match data {
        Ok(car_info) => {
            println!("🆔 PLATE:     {}", plate);
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
        },
        Err(e) => {
            println!("🛑 Error: {}", e);
        }
    }
}
