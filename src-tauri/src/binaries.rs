use crate::commands::DownloadEvent;
use crate::config::BinaryInfo;
use flate2::read::GzDecoder;
use futures_util::StreamExt;
use reqwest::{header::USER_AGENT, Client};
use serde::{Deserialize, Serialize};
use std::env::home_dir;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tar::Archive;
use tauri::ipc::Channel;

#[derive(Debug, Deserialize, Serialize)]
pub struct Release {
    tag_name: String,
    name: String,
    pub assets: Vec<Asset>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
    pub content_type: String,
    #[serde(default)]
    pub installed: bool,
}

pub fn github_client() -> Client {
    let mut builder = Client::builder();
    let client = builder.build().expect("create client");
    // Nota: el header AUTHORIZATION se añade por request para poder alternar token/no-token.
    client
}
pub async fn download_with_progress(
    url: &str,
    dest_path: &str,
    on_event: &Channel<DownloadEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    let home = home_dir();

    let client = Client::new();

    let resp = client
        .get(url)
        .header(USER_AGENT, "rolando-rust-client")
        .send()
        .await?
        .error_for_status()?;

    let total_size = resp
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    on_event.send(DownloadEvent::Started {
        msg: "iniciado".to_string(),
    })?;

    let mut file = File::create(
        Path::new(home.unwrap().as_path())
            .join("pgrustore/bins/")
            .join(dest_path),
    )?;
    let mut downloaded: u64 = 0;

    let mut stream = resp.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;

        if total_size > 0 {
            let percent = (downloaded as f64 / total_size as f64) * 100.0;

            on_event.send(DownloadEvent::Progress {
                msg: percent.to_string(),
            })?;
        } else {
        }
    }

    Ok(())
}

pub fn extract_tar_gz(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Abrir el archivo .tar.gz
    let tar_gz = File::open(file_path)?;
    let decompressor = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(decompressor);
    // 2. Obtener nombre base (sin .tar.gz)
    let path = Path::new(file_path);
    let file_name = path.file_name().unwrap().to_string_lossy();
    let base_name = file_name.trim_end_matches(".tar.gz");
    // 3. Carpeta destino = misma carpeta donde está el tar
    let parent_dir = path.parent().unwrap_or(Path::new("."));
    let dest_dir: PathBuf = parent_dir.join(base_name);
    std::fs::create_dir_all(&dest_dir)?;
    // 4. Extraer dentro de esa carpeta
    archive.unpack(&dest_dir)?;

    Ok(())
}
pub fn get_downloaded_binaries() -> Vec<String> {
    // Detectar arquitectura
    let home = home_dir();
    let base_path = Path::new(home.unwrap().as_path()).join("pgrustore/bins/");

    let mut results = Vec::new();
    if let Ok(entries) = fs::read_dir(base_path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    results.push(entry.file_name().into_string().unwrap());
                }
            }
        }
    }
    results
}

pub fn remove_dir(name: String) {
    let downloaded_binaries = get_downloaded_binaries();
    if let Some(found) = downloaded_binaries
        .iter()
        .find(|entry_b| name.contains(entry_b.as_str()))
    {
        let home = home_dir();
        let base_path = Path::new(home.unwrap().as_path()).join("pgrustore/bins/");

        let path = base_path.join(found);
        fs::remove_dir_all(path).unwrap();
    } else {
        println!("No se encontró ese elemento");
    }
}



