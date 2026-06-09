mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_meta,
            commands::generate_artifact,
            commands::enhance_artifact,
            commands::generate_relic,
            commands::enhance_relic,
            commands::simulate_graduation,
            commands::summarize_graduation_average,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
