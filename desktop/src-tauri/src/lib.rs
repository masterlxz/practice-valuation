use std::sync::atomic::AtomicBool;

mod alert_checker;
mod commands;
mod db;
mod domain;
mod entity;
mod error;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db = tauri::async_runtime::block_on(db::connect()).expect("failed to connect to database");
    let checker_db = db.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .manage(db)
        .manage(AtomicBool::new(false))
        .setup(move |app| {
            alert_checker::spawn_periodic_check(app.handle().clone(), checker_db.clone());
            Ok(())
        })
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
            commands::alert_rule::create_alert_rule,
            commands::alert_rule::list_alert_rules,
            commands::alert_rule::set_alert_rule_active,
            commands::alert_rule::delete_alert_rule,
            commands::collector::run_stock_collector,
            commands::collector::run_crypto_collector,
            commands::collector::list_stock_quotes,
            commands::collector::list_stock_fundamentals,
            commands::collector::list_stock_dividends_avg,
            commands::collector::list_stock_dcf_fundamentals
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
