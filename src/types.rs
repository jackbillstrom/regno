// src/types.rs

#[derive(Debug)]
pub(crate) enum FuelType {
    Unknown,
    Diesel,
    Petrol,
    Electric,
}

#[derive(Debug)]
pub(crate) enum Transmission {
    Unknown,
    Automatic,
    Manual,
}

#[derive(Debug)]
pub struct CarInfo {
    pub(crate) special_note: Option<String>, // Warnings from biluppgifter.se
    pub(crate) make: String, // Make of the car
    pub(crate) model: String, // Model of the car
    pub(crate) year: u16, // Year of the car
    pub(crate) registration_number: String, // Registration number of the car
    pub(crate) stolen: String, // Is the car stolen?
    pub(crate) vin: String, // VIN of the car
    pub(crate) owner: String, // Owner of the car
    pub(crate) number_of_owners: u16, // Number of owners of the car
    pub(crate) annual_mileage: String, // Annual mileage of the car
    pub(crate) fuel_type: FuelType, // Fuel type of the car
    pub(crate) transmission: Transmission, // Transmission of the car
    pub(crate) environmental_class: Option<String>, // Environmental class of the car
    pub(crate) first_registration_date: Option<String>, // First registration date of the car
    pub(crate) horsepower: String, // Horsepower of the car
    pub(crate) fuel_consumption: String, // Fuel consumption of the car
    pub(crate) last_inspection_date: Option<String>, // Last inspection date of the car
    pub(crate) next_inspection_date: Option<String>, // Next inspection date of the car
    pub(crate) annual_tax: String, // Annual tax of the car
    pub(crate) vehicle_status: String, // Vehicle status of the car
    pub(crate) estimated_dealer_price: Option<String>, // Estimated car price from a dealer
    pub(crate) estimated_private_price: Option<String>, // Estimated car price from a private seller
    pub(crate) chassi: String,  // Chassi of the car
    pub(crate) history: Vec<String>, // History of the car
    pub(crate) color: String, // Color of the car
}

impl Default for CarInfo {
    fn default() -> Self {
        Self {
            special_note: None,
            make: String::new(),
            model: String::new(),
            year: 0,
            registration_number: String::new(),
            stolen: String::new(),
            vin: String::new(),
            owner: String::new(),
            number_of_owners: 0,
            annual_mileage: String::new(),
            fuel_type: FuelType::Unknown,
            transmission: Transmission::Unknown,
            environmental_class: None,
            first_registration_date: None,
            horsepower: String::new(),
            fuel_consumption: String::new(),
            last_inspection_date: None,
            next_inspection_date: None,
            annual_tax: String::new(),
            vehicle_status: String::new(),
            estimated_dealer_price: None,
            estimated_private_price: None,
            chassi: String::new(),
            history: Vec::new(),
            color: String::new(),
        }
    }
}