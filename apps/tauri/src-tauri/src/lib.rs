use crate::config::ensure_config_exist;
use std::sync::Mutex;


mod binaries;
mod commands;
mod config;
mod tasks;

struct AppState {
    launch_path: Mutex<Option<String>>,
}
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello2, {}! You've been greeted from Rust!", name)

}

#[tauri::command]
fn get_launch_path(state: tauri::State<AppState>) -> Option<String> {

    let mut path = state.launch_path.lock().unwrap();
    path.take() // solo se devuelve una vez, luego desaparece
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            launch_path: Mutex::new(std::env::args().nth(1)),
        })
        .setup(|_| {
            ensure_config_exist().expect("NO se pudo crear/cargar la config");

            Ok(())
        })
        .plugin(tauri_plugin_updater::Builder::new().build())
        // .on_page_load(|window, _payload| {
        //     // let args: Vec<String> = std::env::args().collect();
        //     // println!("{}", window.label());
        //     // if args.len() > 1 {
        //     //     let file_path = &args[1];
        //     //     let simple = file_path.replace("\\\\", "\\");
        //     //     println!("Archivo abierto: {}", simple);
        //     //     window.emit("open-filee", simple).unwrap();
        //     // }
        // })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_keyring::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_launch_path,
            commands::backup,
            commands::drop,
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
            commands::backup_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
