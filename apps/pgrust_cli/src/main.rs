use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;
use console::Style;
use pgrust_core::tasks::prueba_core;
fn main() {
    //println!("Hello, world!");
    prueba_core();
    let total_size = 100 * 1024 * 1024; // 100 MB

    let pb = ProgressBar::new(total_size);
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
        ProgressStyle::with_template(
            &template,
        )
        .unwrap()
        .progress_chars("=> "),
    );

    let chunk = 512 * 1024; // 512 KB

    while pb.position() < total_size {
        pb.inc(chunk);

        thread::sleep(Duration::from_millis(100));
    }

    pb.finish_with_message("Completado");
}
