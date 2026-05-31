use tauri::{Emitter, Manager};

mod binaries;
mod commands;
mod config;
mod tasks;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello2, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .on_page_load(|window, _payload| {
            let args: Vec<String> = std::env::args().collect();

            if args.len() > 1 {
                let file_path = &args[1];

                println!("Archivo abierto: {}", file_path);
                window.emit("open-file", file_path).unwrap();
            }
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::backup,
            commands::list_db,
            commands::get_binaries,
            commands::list_db_schemas,
            commands::list_data_sources,
            commands::add_ds,
            commands::edit_ds,
            commands::delete_ds,
            commands::fetch_bins,
            commands::download_bin,
            commands::remove_bin,
            commands::restore,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
