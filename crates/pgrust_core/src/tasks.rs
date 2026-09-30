use crate::utils::executors::DbError;
use crate::utils::{build_command, execute_streaming};
use std::env::home_dir;
use std::path::Path;
use time::macros::format_description;
use time::OffsetDateTime;


pub fn prueba_core() {
    println!("Hola desde app-core!");
}

fn get_formatted_date_time() -> String {
    let now = OffsetDateTime::now_local().unwrap();
    let format = format_description!("[year]_[month]_[day]_[hour][minute]");
    now.format(&format).unwrap()
}
pub fn backup_db() {
    //`${dbName}${schemmaName}_${getFormattedDateTime()}.backup`
    let schema_suffix = String::new();

    let bin = format!(
        "{}{}",
        "C:/Users/rolan/pgrustore/bins/pg-bin16.3-1-win/bin", "/pg_dump.exe"
    );

    let port = "5432";
    //let params = `--file ${path.normalize(backupPath)} --host ${host} --port ${port} --username ${user} --format=c --verbose${schammaParams} ${dbName}`
    // 🔹 Construimos los argumentos

    let base_path = Path::new(home_dir().unwrap().as_path()).join("pgrustore/backups");
    let (ext, format_arg) = (".backup", "--format=c");
    let backup_path = format!(
        "{}/{}{}_{}{}",
        base_path.display(),
        "sigip-julio-26",
        schema_suffix,
        get_formatted_date_time(),
        ext
    );

    let mut args = vec![
        "--file",
        &backup_path,
        "--host",
        "127.0.0.1",
        "--port",
        port,
        "--username",
        "postgres",
        format_arg,
        "--verbose",
    ];

    // Finalmente el nombre de la base
    args.push("sigip-julio-26");

    let cmd = build_command(
        bin,
        &args,
        None,
        Some(vec![("PGPASSWORD".into(), "nvideacerr".into())]),
    );

    println!("Started iniciado /n");
    execute_streaming(cmd, |line| {
        println!("Progress: {}", line);
    })
    .map_err(|e| DbError::ExecutionError(e.to_string())).unwrap();

    println!("Finished finalizado /n");
}
