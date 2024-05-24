#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod vehicles;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Vehículos
            vehicles::create_vehicle,
            vehicles::read_vehicles,
            vehicles::update_vehicle,
            vehicles::delete_vehicle
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
