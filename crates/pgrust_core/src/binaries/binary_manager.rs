use flate2::read::GzDecoder;
use futures_util::StreamExt;
use reqwest::Client;
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::env::home_dir;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tar::Archive;

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
    let builder = Client::builder();
    let client = builder.build().expect("create client");
    // Nota: el header AUTHORIZATION se añade por request para poder alternar token/no-token.
    client
}
pub async fn download_with_progress<F>(
    url: &str,
    dest_path: &str,
    on_progress: &mut F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(u64,u64),
{
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

            on_progress(total_size,downloaded)
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
pub async fn download_bin_and_extract_bin<F>(
    url: &String,
    name: &String,
    mut on_progress: F,
) where
    F: FnMut(u64,u64),
{
    download_with_progress(url.as_str(), name.as_str(), &mut on_progress)
        .await
        .unwrap();
    let home = home_dir();
    let base_path = Path::new(home.unwrap().as_path())
        .join("pgrustore/bins/")
        .join(name);
    extract_tar_gz(base_path.to_str().unwrap()).expect("TODO: panic message");
    //let path = Path::new("C:/Users/rolan/pgrustore/bins/").join(name.as_str());
    fs::remove_file(base_path).unwrap();
    on_progress(0,0);
}
