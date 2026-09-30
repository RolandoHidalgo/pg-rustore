use crate::backups::backup;
use crate::data_sources::ds_ui::ds_main_menu;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

pub fn handle_main_menu() {
    let selections = &[
        "Gestonar DS",
        "Edit ds",
        "Backup",
        "A Pile of sweet, sweet mustard",
        "salir",
    ];
    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your flavor")
            // .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();
        match selection {
            0 => {
                ds_main_menu();
            }
            1 => {
                println!("nevermind then {}:(", 1)
            }
            2 => {
                println!("Creando backup /n {}:(", 2);
                println!();
                backup();
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
