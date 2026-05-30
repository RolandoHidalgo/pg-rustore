use std::env::home_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub datasources: Vec<DataSource>,
    pub general: General,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct General {
    pub active_ds: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataSource {
    pub name: String,
    pub bin: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub is_active: bool,
    pub is_ssh: bool,
}

#[derive(Clone, Serialize)]

pub struct DataSourceConfig {
    pub active_ds: String,
    pub datasources: Vec<DataSource>,
}

pub fn load_config() -> Config {
    let home = home_dir();

    let base_path = Path::new(home.unwrap().as_path()).join("pgrustore/config.toml");
    
    //let path = Path::new("C:/Users/rolan/pgrustore/config.toml");
    let content = fs::read_to_string(base_path).expect("No se pudo leer el archivo");
    toml::from_str(&content).expect("Error al parsear TOML")
}

pub fn save_config(cfg: &Config) {
    let home = home_dir();

    let base_path = Path::new(home.unwrap().as_path()).join("pgrustore/config.toml"); 
    
    //let path = Path::new("C:/Users/rolan/pgrustore/config.toml");
    let new_content = toml::to_string_pretty(&cfg).expect("Error al serializar TOML");
    fs::write(base_path, new_content).expect("NO se pudo guarda la config");
}
#[derive(Clone, Serialize, Debug)]
pub struct BinaryInfo {
    pub arq: String,
    pub version: String,
    pub binary: String,
}
fn get_versions(base_path: &Path) -> Vec<BinaryInfo> {
    // Detectar arquitectura
    let arq = if base_path.to_string_lossy().contains("(x86)") {
        "x86".to_string()
    } else {
        "64".to_string()
    };
    let mut results = Vec::new();
    if let Ok(entries) = fs::read_dir(base_path) {
        for entry in entries.flatten() {
            let version = entry.file_name().to_string_lossy().to_string();
            let path_to_restore = base_path.join(&version).join("bin").join("pg_restore.exe");
            if path_to_restore.exists() {
                if let Some(parent) = path_to_restore.parent() {
                    results.push(BinaryInfo {
                        arq: arq.clone(),
                        version,
                        binary: parent.to_string_lossy().to_string(),
                    });
                }
            }
        }
    }
    results
}
pub fn find_binaries(base_paths_arch: &[&Path]) -> Vec<BinaryInfo> {
    base_paths_arch
        .iter()
        .flat_map(|base| {
            if base.exists() {
                get_versions(base)
            } else {
                Vec::new()
            }
        })
        .collect()
}
