use console::Style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{FuzzySelect, Select};
use pgrust_core::config::{Config, DataSourceConfig, load_config};
use pgrust_core::tasks::list_db;
use crate::backups::backup;

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
                    if ds.name == ds_config.active_ds {
                        selected = "[X]";
                    }

                    println!(
                        "{}",
                        cyan.apply_to(format!("{}@{} {}", ds.name, ds.host, selected))
                    );
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

pub fn select_ds() {
    println!("\n");
    //println!("Listado de los ds encontrados:");
    let config: Config = load_config();
    let ds_config = DataSourceConfig {
        datasources: config.datasources,
        active_ds: config.general.active_ds,
    };
    //let cyan = Style::new().blue().bold();
    // for ds in &ds_config.datasources {
    //     let mut selected = "";
    //     if ds.name == ds_config.active_ds{
    //         selected = "[X]";
    //     }
    //
    //     println!("{}",cyan.apply_to(format!("{}@{} {}", ds.name, ds.host,selected)));
    // }
    let selections: Vec<String> = ds_config
        .datasources
        .iter()
        .map(|ds| format!("{}@{}", ds.name, ds.host))
        .collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Seleccione un ds.")
        // .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
    println!("{}", selection);
    let dbs = list_db(&ds_config.datasources[selection].name).expect("TODO: panic message");
    // for db in &dbs {
    //     println!("{}", db);
    // }
    let selected_db = show_db_selection(&dbs);
    backup(&ds_config.datasources[selection],&selected_db);
}
pub fn show_db_selection(dbs: &Vec<String>) -> String {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Seleccione una db")
        .default(0)
        .items(&dbs[..])
        .interact()
        .unwrap();
    dbs[selection].to_string()
}
