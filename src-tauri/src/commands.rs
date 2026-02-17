use std::fs;
use crate::binaries::{
    download_with_progress, extract_tar_gz, get_downloaded_binaries, github_client, Release,
};
use crate::config::{
    find_binaries, load_config, save_config, BinaryInfo, Config, DataSource, DataSourceConfig,
};
use crate::tasks::{LocalTasker, RestoreOptions, Tasker};
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Serialize;
use std::path::Path;
use tauri::ipc::Channel;

#[derive(Clone, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "event",
    content = "data"
)]
pub enum DownloadEvent {
    Started { msg: String },
    Progress { msg: String },
    Finished { msg: String },
}

/// Construye un Command listo para usar, sin ejecutarlo.
/// - `bin`: ruta al ejecutable
/// - `args`: lista de argumentos
/// - `cwd`: directorio de trabajo opcional
/// - `envs`: pares clave/valor de variables de entorno opcionales

#[tauri::command]
pub async fn backup(
    ds_name: String,
    db_name: String,
    schema_name: String,
    on_event: Channel<DownloadEvent>,
) {
    let config: Config = load_config();
    let ds = config
        .datasources
        .iter()
        .find(|d| d.name == ds_name)
        .unwrap();
    let tasker = LocalTasker::new(ds);
    tasker.backup(db_name, schema_name, &on_event);
}

#[tauri::command]
pub async fn restore(
    ds_name: String,
    restore_options: RestoreOptions,
    on_event: Channel<DownloadEvent>,
) {
    let config: Config = load_config();
    let ds = config
        .datasources
        .iter()
        .find(|d| d.name == ds_name)
        .unwrap();
    let tasker = LocalTasker::new(ds);
    tasker.restore(restore_options, &on_event);
}

#[tauri::command]
pub fn list_db(name: String) -> Result<Vec<String>, String> {
    let config: Config = load_config();
    let ds = config.datasources.iter().find(|d| d.name == name).unwrap();
    let tasker = LocalTasker::new(ds);
    match tasker.list_db() {
        Ok(dbs) => Ok(dbs),
        Err(_) => Err("Error genérico al listar bases de datos".to_string()),
    }
}
#[tauri::command]
pub fn list_db_schemas(db_name: String, ds_name: String) -> Vec<String> {
    let config: Config = load_config();
    let ds = config
        .datasources
        .iter()
        .find(|d| d.name == ds_name)
        .unwrap();
    let tasker = LocalTasker::new(ds);
    tasker.list_db_schemas(db_name)
}
#[tauri::command]
pub fn list_data_sources() -> DataSourceConfig {
    let config: Config = load_config();
    DataSourceConfig {
        datasources: config.datasources,
        active_ds: config.general.active_ds,
    }
}

#[tauri::command]
pub fn get_binaries() -> Vec<BinaryInfo> {
    let base_paths_arch = vec![
        Path::new("C:/Program Files/PostgreSQL"),
        Path::new("C:/Program Files (x86)/PostgreSQL"),
    ];
    let bins = get_downloaded_binaries();
    let installed: Vec<BinaryInfo> = bins
        .iter()
        .map(|bin| BinaryInfo {
            arq: "64".to_string(),
            version: bin
                .strip_prefix("pg-bin")
                .and_then(|rest| rest.strip_suffix("-win"))
                .unwrap_or("")
                .to_string(),
            binary: format!("{}/{}/bin", "C:/Users/rolan/pgrustore/bins", bin),
        })
        .collect();
    
    let mut locals = find_binaries(&base_paths_arch);
    locals.extend(installed);
    locals
}

#[tauri::command]
pub fn add_ds(ds: DataSource) {
    let mut config = load_config();
    let new_ds = DataSource {
        name: ds.name,
        bin: ds.bin,
        host: ds.host,
        port: ds.port,
        user: ds.user,
        password: ds.password,
        is_active: false,
        is_ssh: false,
    };
    config.datasources.push(new_ds);
    save_config(&config);
}

#[tauri::command]
pub fn edit_ds(ds: DataSource) {
    let mut config = load_config();

    let data_source = config
        .datasources
        .iter_mut()
        .find(|d| d.name == ds.name)
        .expect("Datasource no encontrado");
    // 3. Modificar campos

    data_source.name = ds.name;
    data_source.bin = ds.bin;
    data_source.host = ds.host;
    data_source.port = ds.port;
    data_source.user = ds.user;
    data_source.password = ds.password;
    data_source.is_active = false;
    data_source.is_ssh = false;

    save_config(&config);
}

#[tauri::command]
pub fn delete_ds(ds_name: String) {
    let mut config = load_config();

    config.datasources.retain(|d| d.name != ds_name);

    save_config(&config);
}
#[tauri::command]
pub async fn fetch_bins() -> Result<Release, String> {
    let url = "https://api.github.com/repos/RolandoHidalgo/pg-Bins/releases/latest".to_string();
    let client = github_client();
    let req = client
        .get(&url)
        .header(USER_AGENT, "rolando-rust-client")
        .header(ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28");
    let mut release = req
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json::<Release>()
        .await
        .unwrap();
    let downloaded = get_downloaded_binaries();
    release.assets.retain(|a| a.name.contains("-win"));

    release.assets.iter_mut().for_each(|a| {
        let found = downloaded.iter().any(|entry_b| a.name.contains(entry_b));
        if found {
            a.installed = true;
        } else {
            a.installed = false;
        }
    });

    Ok(release)

    //download_with_progress("https://github.com/RolandoHidalgo/pg-Bins/releases/download/v2.0.3/pg-bin16.3-1-win.tar.gz","C:/Users/rolan/pgrustore/bins/pg-bin16.3-1-win.tar.gz").await.unwrap();
    //extract_tar_gz("C:/Users/rolan/pgrustore/bins/pg-bin16.3-1-win.tar.gz").expect("TODO: panic message");
}

#[tauri::command]
pub async fn download_bin(url: String, name: String, on_event: Channel<DownloadEvent>) {
    download_with_progress(url.as_str(), name.as_str(), &on_event)
        .await
        .unwrap();
    extract_tar_gz(format!("C:/Users/rolan/pgrustore/bins/{}", name).as_str())
        .expect("TODO: panic message");
    let path = Path::new("C:/Users/rolan/pgrustore/bins/").join(name.as_str());
    fs::remove_file(path).unwrap();
    on_event
        .send(DownloadEvent::Finished {
            msg: "completo".to_string(),
        }).unwrap();
}

#[tauri::command]
pub fn remove_bin(name: String, on_event: Channel<DownloadEvent>) {

    //format!("C:/Users/rolan/pgrustore/bins/{}", name).as_str()
    on_event
        .send(DownloadEvent::Started {
            msg: "iniciado".to_string(),
        }).unwrap();
    let path = Path::new("C:/Users/rolan/pgrustore/bins/").join(name.strip_suffix(".tar.gz").unwrap());
    fs::remove_dir_all(path).unwrap();
    on_event
        .send(DownloadEvent::Finished {
            msg: "completo".to_string(),
        }).unwrap();
}
