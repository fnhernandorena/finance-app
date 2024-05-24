use crate::vehicles::vehicles_model::Vehicle;

use serde_json::{Result, Value};
use std::fs;
use std::path::PathBuf;
use tauri::api::path::app_dir;

fn get_data_path(filename: &str) -> PathBuf {
    let mut path = app_dir().unwrap();
    path.push("data");
    path.push(filename);
    path
}

#[tauri::command]
pub fn create_vehicle(brand: String, model: String, year: u32, value: f64) -> Result<(), String> {
    let mut vehicles = load_vehicles().unwrap_or_else(|_| vec![]);
    let new_id = vehicles.iter().map(|v| v.id).max().unwrap_or(0) + 1;
    let vehicle = Vehicle { id: new_id, brand, model, year, value };
    vehicles.push(vehicle);
    save_vehicles(vehicles)
}

#[tauri::command]
pub fn read_vehicles() -> Result<Vec<Vehicle>, String> {
    load_vehicles()
}

#[tauri::command]
pub fn update_vehicle(id: u32, brand: String, model: String, year: u32, value: f64) -> Result<(), String> {
    let mut vehicles = load_vehicles()?;
    if let Some(vehicle) = vehicles.iter_mut().find(|v| v.id == id) {
        vehicle.brand = brand;
        vehicle.model = model;
        vehicle.year = year;
        vehicle.value = value;
        save_vehicles(vehicles)
    } else {
        Err("Vehicle not found".into())
    }
}

#[tauri::command]
pub fn delete_vehicle(id: u32) -> Result<(), String> {
    let mut vehicles = load_vehicles()?;
    let len_before = vehicles.len();
    vehicles.retain(|v| v.id != id);
    if vehicles.len() < len_before {
        save_vehicles(vehicles)
    } else {
        Err("Vehicle not found".into())
    }
}

fn save_vehicles(vehicles: Vec<Vehicle>) -> Result<(), String> {
    let data_path = get_data_path("vehicles.json");
    let data_json = serde_json::to_string(&vehicles).map_err(|e| e.to_string())?;
    fs::write(data_path, data_json).map_err(|e| e.to_string())
}

fn load_vehicles() -> Result<Vec<Vehicle>, String> {
    let data_path = get_data_path("vehicles.json");
    let data_json = fs::read_to_string(data_path).map_err(|e| e.to_string())?;
    let vehicles: Vec<Vehicle> = serde_json::from_str(&data_json).map_err(|e| e.to_string())?;
    Ok(vehicles)
}