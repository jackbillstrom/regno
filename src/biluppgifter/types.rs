/*
 * This file is part of the 'biluppgifter' library.
 * Responsible for defining the types used in the project.
 */

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
    pub(crate) chassi: String,  // Chassi of the car
    pub(crate) history: Vec<String>, // History of the car
    pub(crate) color: String, // Color of the car
}