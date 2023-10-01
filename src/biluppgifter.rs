/*
 * This file is part of the `biluppgifter` library.
 * Responsible for fetching data from biluppgifter.se and returning it.
*/

pub(crate) mod types;

use scraper::{Html, Selector};

use crate::types::{CarInfo, FuelType, Transmission};
use crate::utils::extract_text;

// get is an async function that returns a Result containing either a CarInfo struct or a reqwest::Error
pub async fn get(plate: &str) -> Result<CarInfo, reqwest::Error> {
    // Grab the HTML from biluppgifter.se
    let url = format!("https://biluppgifter.se/fordon/{}", plate);
    let body = reqwest::get(&url)
        .await?
        .text()
        .await?;

    // Scrape for desired data
    let doc = Html::parse_document(&body);

    let card_owner_selector = Selector::parse(".card-owner").unwrap();
    let enlarge_selector = Selector::parse(".enlarge").unwrap();
    let history_selector = Selector::parse("section.history > div.row").unwrap();
    let data_selector = Selector::parse("div.card.card-body.card-data ul.list-data li").unwrap();
    let technical_selector = Selector::parse("div.card-technical ul.list-data li").unwrap();
    let vehicle_events_selector = Selector::parse("div#box-history section#history > div").unwrap();

    // Init CarInfo struct
    let mut car_info = CarInfo::default();

    // Alert
    let alert_selector = Selector::parse(".alert.alert-danger.text-center").unwrap();
    if let Some(alert_element) = doc.select(&alert_selector).next() {
        let text = alert_element.text().collect::<Vec<_>>().join(" ");
        car_info.special_note = Some(text.trim().replace("\n", " ").replace("\r", " ").to_string());
    }

    // Make, model, year, registration number, VIN etc
    for element in doc.select(&data_selector) {
        let label = extract_text(&element, "span.label");
        let value = extract_text(&element, "span.value");

        match label.as_str() {
            "Fabrikat" => car_info.make = value,
            "Modell" => car_info.model = value,
            "Fordonsår / Modellår" => {
                let years: Vec<&str> = value.split("/").collect();
                car_info.year = years[0].trim().parse::<u32>().unwrap_or(0) as u16;
            }
            "Registreringsnummer" => car_info.registration_number = value,
            "Chassinr / VIN" => car_info.vin = value,
            "Status" => car_info.vehicle_status = value,
            "Först registrerad" => car_info.first_registration_date = Some(value),
            "Senast besiktigad" => car_info.last_inspection_date = Some(value),
            "Nästa besiktning senast" => car_info.next_inspection_date = Some(value),
            "Stöldstatus Sverige" => car_info.stolen = value,
            "Mätarställning (besiktning)" => car_info.annual_mileage = value,
            "Miljöklass" => car_info.environmental_class = Some(value),
            "Årlig skatt" => car_info.annual_tax = value,
            _ => continue,
        }
    }

    // Owner
    if let Some(card_owner_element) = doc.select(&card_owner_selector).next() {
        let mut i = 0;
        for element in card_owner_element.select(&enlarge_selector) {
            // We're only interested in the first element
            if i > 0 {
                break;
            }
            i += 1;
            let text = element.text().collect::<Vec<_>>().join(" ");
            car_info.owner = text.trim().replace("\n", " ").replace("\r", " ").to_string();
        }

        let mut num_owners = 0;
        for element in card_owner_element.select(&history_selector) {
            let title = element.select(&Selector::parse(".title").unwrap()).next().unwrap().text().collect::<String>().trim().to_string();

            // To count the number of owners, we only need to check for the title "Ägarbyte"
            if title == "Ägarbyte" {
                num_owners += 1;
            }
        }
        car_info.number_of_owners = num_owners;
    }

    // TODO: Estimated value

    // History
    for element in doc.select(&vehicle_events_selector) {
        let title = extract_text(&element, ".title");
        let date = extract_text(&element, ".date");
        let text = extract_text(&element, ".content");

        car_info.history.push(format!("{} - {} - {}", title, date, text));
    }

    // Color, model, make, etc.
    for element in doc.select(&technical_selector) {
        let label = extract_text(&element, "span.label");
        let value = extract_text(&element, "span.value");

        match label.as_str() {
            "Motoreffekt" => car_info.horsepower = value,
            "Drivmedel" => {
                car_info.fuel_type = match value.as_str() {
                    "Diesel" => FuelType::Diesel,
                    "Bensin" => FuelType::Petrol,
                    "El" => FuelType::Electric,
                    _ => FuelType::Unknown,
                }
            }
            "Växellåda" => {
                car_info.transmission = match value.as_str() {
                    "Manuell" => Transmission::Manual,
                    "Automatisk" => Transmission::Automatic,
                    _ => Transmission::Unknown,
                }
            }
            "Bränsleförbrukning (NEDC)" => car_info.fuel_consumption = value,
            "Färg" => car_info.color = value,
            "Kaross" => car_info.chassi = value,
            _ => continue,
        }
    }

    Ok(car_info)
}