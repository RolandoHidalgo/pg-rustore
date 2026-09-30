use console::Style;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use pgrust_core::config::{Config, DataSourceConfig, load_config};

pub fn ds_main_menu() {
    let selections = &[
        "Add ds",
        "List ds",
        "Delete ds",
        "A Pile of sweet, sweet mustard",
        "<- Volver",
    ];
    loop {
        println!();
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Seleccione una opcion para los ds.")
            // .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();
        match selection {
            0 => {
                println!("nevermind then {}:(", 0)
            }
            1 => {
                println!("\n");
                println!("Listado de los ds encontrados:");
                let config: Config = load_config();
                let ds_config = DataSourceConfig {
                    datasources: config.datasources,
                    active_ds: config.general.active_ds,
                };
                let cyan = Style::new().blue().bold();
                for ds in &ds_config.datasources {
                    let mut selected = "";
                    if ds.name == ds_config.active_ds{
                        selected = "[X]";
                    }

                    println!("{}",cyan.apply_to(format!("{}@{} {}", ds.name, ds.host,selected)));
                }
            }
            2 => {
                println!("nevermind then {}:(", 2)
            }
            3 => {
                println!("nevermind then {}:(", 3)
            }
            4 => {
                println!("nevermind then {}:(", 4);
                break;
            }
            _ => {
                println!("nevermind then {}:(", 44);
            }
        }
    }
}
