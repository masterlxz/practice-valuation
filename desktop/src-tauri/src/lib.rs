use std::sync::atomic::AtomicBool;

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
        .manage(AtomicBool::new(false))
        .invoke_handler(tauri::generate_handler![
            commands::bazin::calculate_bazin,
            commands::graham::calculate_graham,
            commands::gordon::calculate_gordon,
            commands::dcf::calculate_dcf,
            commands::banks::calculate_banks,
            commands::rnav::calculate_rnav,
            commands::projected_ceiling::calculate_projected_ceiling,
            commands::crypto_indicator::record_crypto_indicator,
            commands::crypto_indicator::list_crypto_indicators,
            commands::valuation::list_valuations,
            commands::valuation::get_valuation_inputs,
            commands::valuation::update_valuation,
            commands::valuation::delete_valuation,
            commands::collector::run_stock_collector,
            commands::collector::list_stock_quotes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
