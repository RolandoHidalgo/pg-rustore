use crate::backups::backup;
use crate::backups::backup_manager::restore_db;
use console::Style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, FuzzySelect, Input, Select};
use pgrust_core::config::{Config, DataSource, DataSourceConfig, load_config};
use pgrust_core::tasks::{NewDbOptions, RestoreOptions, list_db, list_db_schemas};
use std::path::Path;

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
pub fn show_backup_flow() {
    let ds = select_ds();
    let dbs = list_db(&ds.name).expect("TODO: panic message");
    // for db in &dbs {
    //     println!("{}", db);
    // }
    let selected_db = show_db_selection(&dbs);
    let schema = get_schema_name_menu(&ds, &selected_db);
    backup(&ds, &selected_db, &schema);
}
pub fn select_ds() -> DataSource {
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
    ds_config
        .datasources
        .into_iter()
        .nth(selection)
        .expect("Error de seleccion de ds.")
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

pub fn get_schemas(ds: &DataSource, db_name: &String) -> Vec<String> {
    list_db_schemas(ds, db_name)
}

pub fn get_schema_name_menu(ds: &DataSource, db_name: &String) -> String {
    let mut schema_name = "".to_string();
    let selections = &["Toda la db", "seleccionar esquema"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Optionally pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    if selection == 1 {
        let schemmas = get_schemas(ds, db_name);

        let s_selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Seleccione un schema")
            .default(0)
            .items(&schemmas[..])
            .interact()
            .unwrap();
        schema_name = format!("{}", &schemmas[s_selection]);
    }

    schema_name.to_string()
}

pub fn new_db_menu() -> RestoreOptions {
    let templates = &["template0"];
    let collations = &["C"];
    let c_type = &["C"];
    let tablesapces = &["pg_default"];
    let encodings = &["LATIN1"];

    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your name")
        .interact_text()
        .unwrap();
    let selection_template = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Seleccione un template")
        .default(0)
        .items(&templates[..])
        .interact()
        .unwrap();

    let selection_collations = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Seleccione un collations")
        .default(0)
        .items(&collations[..])
        .interact()
        .unwrap();

    let selection_c_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Seleccione un C. type")
        .default(0)
        .items(&c_type[..])
        .interact()
        .unwrap();

    let selection_ts = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Seleccione un Table_space")
        .default(0)
        .items(&tablesapces[..])
        .interact()
        .unwrap();

    let selection_encodings = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Seleccione un Table_space")
        .default(0)
        .items(&encodings[..])
        .interact()
        .unwrap();
    let backup_path = get_backup_path();

    RestoreOptions {
        db_name: input,
        backup: backup_path,
        new_db_options: Some(NewDbOptions {
            template: templates[selection_template].to_string(),
            collation: collations[selection_collations].to_string(),
            character_type: c_type[selection_c_type].to_string(),
            tablespace: tablesapces[selection_ts].to_string(),
            encoding: encodings[selection_encodings].to_string(),
        }),
    }
}

/// Solicita al usuario la ruta absoluta de un archivo .backup y valida que exista.
fn get_backup_path() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Ingrese la ruta absoluta del archivo .backup")
        .validate_with(|input: &String| {
            let path = Path::new(input);

            if !path.is_absolute() {
                return Err(
                    "La ruta debe ser absoluta. Ejemplo: C:/backups/mi_db.backup".to_string(),
                );
            }
            if !path.exists() {
                return Err(format!("El archivo '{}' no existe.", input));
            }
            if !path.is_file() {
                return Err(format!("'{}' no es un archivo válido.", input));
            }
            Ok(())
        })
        .interact_text()
        .unwrap()
}

pub fn show_restore_flow() {
    let ds = select_ds();
    let options = get_restore_options(&ds);
    restore_db(&ds, &options);
}

pub fn get_restore_options(ds: &DataSource) -> RestoreOptions {
    let new_db = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("¿Crear una nueva base de datos?")
        .interact()
        .unwrap();

    if new_db {
        new_db_menu()
    } else {
        let dbs = list_db(&ds.name).expect("Error al listar bases de datos");
        let db_name = show_db_selection(&dbs);
        let backup = get_backup_path();

        RestoreOptions {
            db_name,
            backup,
            new_db_options: None,
        }
    }
}
