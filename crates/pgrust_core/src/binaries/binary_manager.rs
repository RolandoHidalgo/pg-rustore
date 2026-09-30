use std::fs;
use std::path::Path;
use serde::Serialize;

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
