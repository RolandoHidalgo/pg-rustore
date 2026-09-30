use console::{Color, Style};
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
                // backup();
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
pub fn print_banner() {


    let cyan_bold = Style::new().cyan().bold();
    let blue_bold = Style::new().blue().bold();
    let blue = Style::new().blue();
    let blue_dim = Style::new().blue().dim();

    let lines = [
        " ██████╗ ██████╗ ███████╗██████╗  █████╗ ████████╗ ██████╗ ██████╗ ",
        " ██╔══██╗██╔══██╗██╔════╝██╔══██╗██╔══██╗╚══██╔══╝██╔═══██╗██╔══██╗",
        " ██████╔╝██████╔╝█████╗  ██████╔╝███████║   ██║   ██║   ██║██████╔╝",
        " ██╔═══╝ ██╔══██╗██╔══╝  ██╔══██╗██╔══██║   ██║   ██║   ██║██══██╗",
        " ██║     ██║  ██║███████╗██║  ██║██║  ██║   ██║   ╚██████╔╝██║  ██║",
        " ╚═╝     ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ═╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝",
    ];

    for (i, line) in lines.iter().enumerate() {
        let style = if i % 2 == 0 { &cyan_bold } else { &blue_bold };
        println!("{}", style.apply_to(line));
    }

    println!();
    println!(
        "  {} {} {}",
        blue.apply_to("🦀"),
        cyan_bold.apply_to("PGRUSTORE"),
        blue_dim.apply_to("— PostgreSQL Wrapper")
    );
    println!();
}

pub fn print_banner2() {
    let cyan_bold = Style::new().cyan().bold();
    let blue = Style::new().blue();
    let blue_dim = Style::new().blue().dim();

    let banner = r#"
 ██████╗ ██████╗ ███████╗██████╗  █████╗ ████████╗ ██████╗ ██████╗
 ██╔══██╗██╔══██╗██╔════╝██╔══██╗██╔══██╗╚══██╔══╝██═══██╗██╔══██╗
 ██████╔╝██████╔╝█████╗  ██████╔╝███████║   ██║   ██║   ██║██████╔╝
 ██╔═══╝ ██╔══██╗██╔══╝  ██╔══██╗██╔══██║   ██║   ██║   ██║██╔══██╗
 ██║     ██║  ██║███████╗██║  ██║██║  ██║   ██║   ╚██████╔╝██║  ██║
 ╚═╝     ╚═╝  ╚═╝╚══════╝═╝  ╚═╝╚═╝  ╚═╝   ╚═╝    ═════╝ ╚═╝  ╚═╝
"#;

    println!("{}", cyan_bold.apply_to(banner));

    println!(
        "  {} {} {}",
        blue.apply_to(""),
        cyan_bold.apply_to("PGRUSTORE"),
        blue_dim.apply_to("— PostgreSQL Wrapper")
    );
    println!();}



pub fn print_banner3() {
    // Color256(208) es un naranja vibrante estilo Rust
    let rust_orange = Style::new().fg(Color::Color256(208)).bold();
    let rust_orange_dim = Style::new().fg(Color::Color256(208)).dim();

    let banner = r#"
██████╗  ██████╗ ██████╗ ██╗   ██╗███████╗████████╗ ██████╗ ██████╗ ███████╗
██╔══██╗██╔════╝ ██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗██╔════╝
██████╔╝██║  ███╗██████╔╝██║   ██║███████╗   ██║   ██║   ██║██████╔╝█████╗
██╔═══╝ ██║   ██║██╔══██╗██║   ██║╚════██║   ██║   ██║   ██║██╔══██╗██╔══╝
██║     ╚██████╔╝██║  ██║╚██████╔╝███████║   ██║   ╚██████╔╝██║  ██║███████╗
╚═╝      ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚══════╝
"#;

    println!("{}", rust_orange.apply_to(banner));

    println!(
        "  {} {} {}",
        rust_orange.apply_to("🦀"),
        rust_orange.apply_to("PGRUSTORE"),
        rust_orange_dim.apply_to("— PostgreSQL Wrapper")
    );
    println!();
}