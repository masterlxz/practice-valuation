mod commands;
mod db;
mod domain;
mod entity;
mod error;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db = tauri::async_runtime::block_on(db::connect()).expect("failed to connect to database");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            commands::bazin::calculate_bazin,
            commands::graham::calculate_graham,
            commands::gordon::calculate_gordon
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
