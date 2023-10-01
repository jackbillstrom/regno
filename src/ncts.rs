use reqwest::Error;
use scraper::{Html, Selector};
use crate::types::CarInfo;

pub async fn get(plate: &str) -> Result<CarInfo, Error> {
    // Grab the HTML from the NCTS website
    let url = format!("https://www.cartell.ie/ssl/servlet/beginStarLookup?registration={}", plate);
    let body = reqwest::get(&url)
        .await?
        .text()
        .await?;

    // Scrape for desired data
    let document = Html::parse_document(&body);

    let table_selector = Selector::parse("table").unwrap();
    let th_selector = Selector::parse("th").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    let mut car_info = CarInfo::default();

    if let Some(table_element) = document.select(&table_selector).next() {
        let mut headers: Vec<String> = Vec::new();
        let mut values: Vec<String> = Vec::new();

        for th in table_element.select(&th_selector) {
            headers.push(th.inner_html());
        }

        for td in table_element.select(&td_selector) {
            values.push(td.inner_html());
        }

        for (i, header) in headers.iter().enumerate() {
            match header.as_str() {
                "Make &amp; Model" => car_info.make = values[i].clone(),
                "Description" => car_info.model = values[i].clone(),
                "Engine Capacity" => car_info.horsepower = values[i].clone(),
                _ => {}
            }
        }
    }

    Ok(car_info)
}