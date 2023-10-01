use reqwest::Error;
use scraper::{ElementRef, Html, Selector};
use crate::types::{CarInfo, FuelType};

// This function is used to grab the car information from the regnr.no website
pub async fn get(plate: &str) -> Result<CarInfo, Error> {
    // Grab the HTML from the regnr.no website
    let url = format!("https://regnr.no/{}", plate);
    let body = reqwest::get(&url)
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&body);
    let mut car_info = CarInfo::default();

    let selector_for_regnr = Selector::parse(r#"input[name="regnr"]"#).unwrap(); // Selector for the registration number
    // let selector_for_make = Selector::parse(".main-grid > div:nth-child(1)").unwrap(); // Selector for the make
    let selector_for_flex_h = Selector::parse(".flex-h.slim").unwrap(); // Selector for each row
    let selector_for_icon_info = Selector::parse(".flex-icon-info").unwrap(); // Selector for the icons
    // let selector_for_estimated_dealer_price = Selector::parse(".container-title:contains('Bilforhandlerpris') + div .text-price").unwrap();
    // let selector_for_estimated_private_price = Selector::parse(".container-title:contains('Privatpris') + div .text-price").unwrap();
    // let selector_for_special_text = Selector::parse(".container-title:contains('Økonomiske heftelser') + div .text-price").unwrap();

    for flex_h in document.select(&selector_for_flex_h) {
        let text_elements: Vec<ElementRef> = flex_h.select(&Selector::parse("div").unwrap()).collect();

        if text_elements.len() >= 2 {
            let label = text_elements[0].inner_html().trim().to_string();
            let value = text_elements[1].inner_html().trim().to_string();

            match label.as_str() {
                "Registrert første gang" => car_info.first_registration_date = Some(value),
                "Understells-/chassisnr. (VIN)" => car_info.vin = value,
                "Miljøklasse" => car_info.environmental_class = Some(value),
                "Neste frist for godkjent EU-kontroll" => car_info.next_inspection_date = Some(value),
                "Sist EU-godkjent" => car_info.last_inspection_date = Some(value),
                _ => {}
            }
        }
    }

    // Plate
    if let Some(element) = document.select(&selector_for_regnr).next() {
        if let Some(regnr) = element.value().attr("value") {
            car_info.registration_number = regnr.to_string();
        }
    }

    // Consumption, fuel type, engine power
    for icon_info in document.select(&selector_for_icon_info) {
        let text_elements: Vec<ElementRef> = icon_info.select(&Selector::parse("div").unwrap()).collect();

        if text_elements.len() >= 2 {
            let combined_text = text_elements[0].text().chain(text_elements[1].text()).collect::<String>();
            let trimmed_text = combined_text.trim();
            let parts: Vec<&str> = trimmed_text.split_whitespace().collect();

            let label = parts[0];  // "Drivstofforbruk"
            let value = parts[1];  // "0.66 liter per mil"

            match label {
                "Drivstofforbruk" => car_info.fuel_consumption = value.to_string(),
                "Drivstoff" => {
                    car_info.fuel_type = match value.replace(label, "").trim() {
                        "Diesel" => FuelType::Diesel,
                        "Bensin" => FuelType::Petrol,
                        "El" => FuelType::Electric,
                        _ => FuelType::Unknown,
                    }
                }
                "Motor" => car_info.horsepower = value.to_string(),
                _ => {}
            }
        }
    }

    Ok(car_info)
}