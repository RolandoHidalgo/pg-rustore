use crate::config::{Config, DataSource, load_config};
use crate::utils::executors::DbError;
use crate::utils::{build_command, execute_streaming};
use serde::{Deserialize, Serialize};
use std::env::home_dir;

use std::path::{Path, PathBuf};
use time::OffsetDateTime;
use time::macros::format_description;
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RestoreOptions {
    pub db_name: String,
    pub backup: String,
    pub new_db_options: Option<NewDbOptions>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewDbOptions {
    pub encoding: String,
    pub template: String,
    pub collation: String,
    pub character_type: String,
    pub tablespace: String,
}
pub fn prueba_core() {
    println!("Hola desde app-core!");
}

fn get_formatted_date_time() -> String {
    let now = OffsetDateTime::now_local().unwrap();
    let format = format_description!("[year]_[month]_[day]_[hour][minute]");
    now.format(&format).unwrap()
}
pub fn backup_db<F>(ds: &DataSource, db_name: &String, schema_name: &String, mut on_line: F)
where
    F: FnMut(String),
{
    //`${dbName}${schemmaName}_${getFormattedDateTime()}.backup`
    let schema_suffix = if schema_name.is_empty() {
        String::new()
    } else {
        format!("_{}", schema_name)
    };

    let bin = format!("{}{}", &ds.bin, "/pg_dump.exe");

    let port = &ds.port.to_string();
    //let params = `--file ${path.normalize(backupPath)} --host ${host} --port ${port} --username ${user} --format=c --verbose${schammaParams} ${dbName}`
    // 🔹 Construimos los argumentos

    let base_path = Path::new(home_dir().unwrap().as_path()).join("pgrustore/backups");
    let (ext, format_arg) = (".backup", "--format=c");
    let backup_path = format!(
        "{}/{}{}_{}{}",
        base_path.display(),
        &db_name,
        schema_suffix,
        get_formatted_date_time(),
        ext
    );

    let mut args = vec![
        "--file",
        &backup_path,
        "--host",
        &ds.host,
        "--port",
        port,
        "--username",
        &ds.user,
        format_arg,
        "--verbose",
    ];
    if !schema_name.is_empty() {
        args.push("--schema");
        args.push(&schema_name);
    }

    // Finalmente el nombre de la base
    args.push(&db_name);

    let cmd = build_command(
        bin,
        &args,
        None,
        Some(vec![("PGPASSWORD".into(), ds.password.as_str().into())]),
    );

    println!("Started iniciado /n");
    execute_streaming(cmd, |line| on_line(line))
        .map_err(|e| DbError::ExecutionError(e.to_string()))
        .unwrap();

    println!("Finished finalizado /n");
}

pub fn list_db(name: &String) -> Result<Vec<String>, String> {
    let config: Config = load_config();

    let ds = config.datasources.iter().find(|d| &d.name == name).unwrap();
    // let tasker = LocalTasker::new(ds);

    // match tasker.list_db() {
    //     Ok(dbs) => Ok(dbs),
    //     Err(_) => Err("Error genérico al listar bases de datos".to_string()),
    // }

    Ok(list_db_base(ds))
}

fn list_db_base(ds: &DataSource) -> Vec<String> {
    let bin = format!("{}{}", ds.bin, "/psql.exe");
    let port = &ds.port.to_string();
    //const command = `"${binary}\\${commands.psql}" -U ${user} --host ${host} --port ${port} -c "\\l"`
    // 🔹 Construimos los argumentos
    let args = vec![
        "-U", &ds.user, "--host", &ds.host, "--port", port, "-c", "\\l",
    ];

    let mut cmd = build_command(
        bin,
        &args,
        None,
        Some(vec![("PGPASSWORD".into(), ds.password.clone())]),
    );
    let output = cmd
        .output()
        .map_err(|e| DbError::SpawnError(e.to_string()))
        .unwrap();

    if !output.status.success() {
        let _err_msg = String::from_utf8_lossy(&output.stderr).to_string();
        //return Err(DbError::ExecutionError(format!("psql falló: {}", err_msg)));
    }

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let mut dbs = Vec::new();

    for line in stdout_str.lines() {
        if line.contains('|')
            && !line.contains("Name")
            && !line.contains("Nombre")
            && !line.starts_with('-')
        {
            if let Some(first_col) = line.split('|').next() {
                let db_name = first_col.trim();
                if !db_name.is_empty() && db_name != "template0" && db_name != "template1" {
                    dbs.push(db_name.to_string());
                }
            }
        }
    }

    dbs
}
pub fn list_db_schemas(ds: &DataSource, db_name: &String) -> Vec<String> {
    let bin = format!("{}{}", &ds.bin, "/psql.exe");
    let port = &ds.port.to_string();
    //const command = `"${binary}\\${commands.psql}" -U ${username} --host ${host} -d ${dbName} --port ${port} -c "\\dn"`,
    // 🔹 Construimos los argumentos
    let args: Vec<&str> = vec![
        "-U", &ds.user, "--host", &ds.host, "-d", &db_name, "--port", port, "-c", "\\dn",
    ];

    let mut cmd = build_command(
        bin,
        &args,
        None,
        Some(vec![("PGPASSWORD".into(), ds.password.clone())]),
    );
    let output = cmd
        .output()
        .map_err(|e| DbError::SpawnError(e.to_string()))
        .unwrap();

    let stdout_str = String::from_utf8_lossy(&output.stdout);

    let mut schemas: Vec<String> = Vec::new();

    for line in stdout_str.lines() {
        // saltamos encabezados y separadores
        if line.contains('|')
            && !line.contains("Name")
            && !line.contains("Nombre")
            && !line.starts_with('-')
        {
            // primera columna antes del primer '|'
            if let Some(first_col) = line.split('|').next() {
                let db_name = first_col.trim();
                if !db_name.is_empty() && db_name != "template0" && db_name != "template1" {
                    schemas.push(db_name.to_string());
                }
            }
        }
    }

    schemas
}

pub fn restore<F>(ds: &DataSource, restore_options: &RestoreOptions, mut on_line: F)
where
    F: FnMut(String),
{
    if let Some(new_db_options) = &restore_options.new_db_options {
        println!(
            "crear db {} {} {}",
            new_db_options.encoding, new_db_options.template, restore_options.db_name
        );
        create_db(&ds, &restore_options, |line| on_line(line));
    }

    let bin = format!("{}{}", &ds.bin, "/pg_restore.exe");
    let path = PathBuf::from(&restore_options.backup);
    let port = &ds.port.to_string();
    let _is_backup = &restore_options.backup.ends_with(".backup");
    // 🔹 Construimos los argumentos
    let args = vec![
        "--host",
        &ds.host,
        "--port",
        port,
        "--username",
        &ds.user,
        "--role",
        "postgres",
        "-j",
        "3",
        "--dbname",
        &restore_options.db_name,
        "--verbose",
        path.to_str().unwrap(),
    ];
    // println!("{}",&restore_options.backup);
    // if *is_backup {
    //     println!("entro");
    //     args.push("-j");
    //     args.push("3");
    // }

    let cmd = build_command(
        bin,
        &args,
        None,
        Some(vec![("PGPASSWORD".into(), ds.password.as_str().into())]),
    );

    execute_streaming(cmd, |line| on_line(line))
        .map_err(|e| DbError::ExecutionError(e.to_string()))
        .unwrap();
}

fn create_db<F>(ds: &DataSource, restore_options: &RestoreOptions, mut on_line: F)
where
    F: FnMut(String),
{
    let bin = format!("{}{}", &ds.bin, "/createdb.exe");
    let port = &ds.port.to_string();
    //let params = `--file ${path.normalize(backupPath)} --host ${host} --port ${port} --username ${user} --format=c --verbose${schammaParams} ${dbName}`
    // 🔹 Construimos los argumentos
    let new_opts = restore_options.new_db_options.as_ref().unwrap();
    let args: Vec<&str> = vec![
        "--host",
        &ds.host,
        "--port",
        port,
        "--username",
        &ds.user,
        "--encoding",
        &new_opts.encoding,
        "--lc-ctype",
        &new_opts.character_type,
        "--tablespace",
        &new_opts.tablespace,
        "--lc-collate",
        &new_opts.collation,
        "--template",
        &new_opts.template,
        &restore_options.db_name,
    ];

    let cmd = build_command(
        bin,
        &args,
        None,
        Some(vec![("PGPASSWORD".into(), ds.password.as_str().into())]),
    );

    execute_streaming(cmd, |line| on_line(line))
        .map_err(|e| DbError::ExecutionError(e.to_string()))
        .unwrap();
}
