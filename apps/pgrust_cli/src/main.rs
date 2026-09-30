use crate::menu::{handle_main_menu, print_banner3};
use crate::tasks::{inputs, selects};
use dialoguer::{FuzzySelect, theme::ColorfulTheme};
use pgrust_core::config::ensure_config_exist;
use crate::data_sources::ds_ui::select_ds;

mod backups;
mod data_sources;
mod menu;
mod tasks;
// use indicatif::{ProgressBar, ProgressStyle};
// use std::thread;
// use std::time::Duration;
// use console::Style;
// use dialoguer::Confirm;
// use dialoguer::theme::ColorfulTheme;
// use pgrust_core::tasks::prueba_core;

fn main() {
    // inputs();
    print_banner3();
    ensure_config_exist().expect("TODO: panic message");
   // handle_main_menu();
    select_ds();

    // let selections = &[
    //     "Ice Cream",
    //     "Vanilla Cupcake",
    //     "Chocolate Muffin",
    //     "A Pile of sweet, sweet mustard",
    //     "Carrots",
    //     "Peas",
    //     "Pistacio",
    //     "Mustard",
    //     "Cream",
    //     "Banana",
    //     "Chocolate",
    //     "Flakes",
    //     "Corn",
    //     "Cake",
    //     "Tarte",
    //     "Cheddar",
    //     "Vanilla",
    //     "Hazelnut",
    //     "Flour",
    //     "Sugar",
    //     "Salt",
    //     "Potato",
    //     "French Fries",
    //     "Pizza",
    //     "Mousse au chocolat",
    //     "Brown sugar",
    //     "Blueberry",
    //     "Burger",
    // ];
    //
    // let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Pick your flavor")
    //     .default(0)
    //     .items(&selections[..])
    //     .interact()
    //     .unwrap();
    //
    // println!("Enjoy your {}!", selections[selection]);
    //println!("Hello, world!");
    // prueba_core();
    // let total_size = 100 * 1024 * 1024; // 100 MB
    //
    // let pb = ProgressBar::new(total_size);
    // // Estilo para los corchetes
    // let cyan = Style::new().cyan().bold();
    //
    // // Template original con corchetes normales
    // let template = "({percent:.blue}%) [{bar:30.cyan/blue}] {bytes:>8}/{total_bytes:<8} {binary_bytes_per_sec:>10} ETA {eta:>4}";
    //
    // // Reemplazamos los corchetes por versiones coloreadas
    // let template = template
    //     .replace("[", &cyan.apply_to("[").to_string())
    //     .replace("]", &cyan.apply_to("]").to_string())
    //     .replace("(", &cyan.apply_to("(").to_string())
    //     .replace("%", &cyan.apply_to("%").to_string())
    //     .replace(")", &cyan.apply_to(")").to_string());
    // pb.set_style(
    //     ProgressStyle::with_template(
    //         &template,
    //     )
    //     .unwrap()
    //     .progress_chars("=> "),
    // );
    //
    // let chunk = 512 * 1024; // 512 KB
    //
    // while pb.position() < total_size {
    //     pb.inc(chunk);
    //
    //     thread::sleep(Duration::from_millis(100));
    // }
    //
    // pb.finish_with_message("Completado");
}
