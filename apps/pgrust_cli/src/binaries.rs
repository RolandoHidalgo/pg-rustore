use console::Style;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use indicatif::{ProgressBar, ProgressStyle};
use pgrust_core::binaries::binary_manager::{Asset, Release, download_bin_and_extract_bin};
use pgrust_core::binaries::fetch_bins;
use std::time::Duration;

pub async fn show_binaries() {
    let spinner = ProgressBar::new_spinner();

    // 2. ¡CRUCIAL! Habilitar la animación automática cada 100ms.
    // Sin esto, el spinner se congelará durante el .await
    spinner.enable_steady_tick(Duration::from_millis(100));

    // 3. Configurar el estilo visual (usando caracteres Unicode de braille, se ven muy profesionales)
    let style = ProgressStyle::with_template("{spinner:.cyan.bold} {msg}")
        .unwrap()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]);

    spinner.set_style(style);
    spinner.set_message("Consultando última versión en GitHub...");

    let release: Release = fetch_bins().await.expect("ad");
    let binaries: Vec<String> = release
        .assets
        .iter()
        .map(|x| {
            let installed = match x.installed {
                true => "[X]",
                false => "[]",
            };
            format!("{}@{} installed{}", x.name, x.size, installed)
        })
        .collect();
    spinner.finish_and_clear();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select binary")
        // .default(0)
        .items(&binaries[..])
        .interact()
        .unwrap();
    let bin = &release.assets[selection];
    download_binary(&bin).await;
}
pub async fn download_binary(asset: &Asset) {
    let pb = ProgressBar::new(0);
    // Estilo para los corchetes
    let cyan = Style::new().cyan().bold();

    // Template original con corchetes normales
    let template = "({percent:.blue}%) [{bar:30.cyan/blue}] {bytes:>8}/{total_bytes:<8} {binary_bytes_per_sec:>10} ETA {eta:>4}";

    // Reemplazamos los corchetes por versiones coloreadas
    let template = template
        .replace("[", &cyan.apply_to("[").to_string())
        .replace("]", &cyan.apply_to("]").to_string())
        .replace("(", &cyan.apply_to("(").to_string())
        .replace("%", &cyan.apply_to("%").to_string())
        .replace(")", &cyan.apply_to(")").to_string());
    pb.set_style(
        ProgressStyle::with_template(&template)
            .unwrap()
            .progress_chars("=> "),
    );
    download_bin_and_extract_bin(
        &asset.browser_download_url,
        &asset.name,
        |total_size, downloaded| {
            if total_size > 0 && pb.length().unwrap_or(0) == 0 {
                // La primera vez que recibimos el tamaño total, configuramos la barra
                pb.set_length(total_size);
            }

            if downloaded == 0 && total_size == 0 {
                // Señal de que terminó (según nuestra lógica en download_bin_and_extract_bin)
                pb.finish_with_message("✅ Extracción completada");
            } else {
                // Actualizamos la posición actual. Indicatif calcula la velocidad y ETA automáticamente.
                pb.set_position(downloaded);
            }
        },
    )
    .await
}
