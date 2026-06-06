use crate::commands::DownloadEvent;
use crate::config::{ DataSource};
use serde::{Deserialize, Serialize};

use std::io::{BufRead, BufReader};
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tauri::ipc::Channel;
use time::macros::format_description;
use time::OffsetDateTime;
const CREATE_NO_WINDOW: u32 = 0x08000000;
#[derive(Clone, Serialize, Deserialize)]
pub struct RestoreOptions {
    pub db_name: String,
    pub backup: String,
    pub new_db_options: Option<NewDbOptions>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NewDbOptions {
    pub encoding: String,
    pub template: String,
    pub collation: String,
    pub character_type: String,
    pub tablespace: String,
}
fn spawn_builder<P: AsRef<Path>>(
    bin: P,
    args: Vec<&str>,
    cwd: Option<P>,
    envs: Option<Vec<(String, String)>>,
) -> Command {
    let mut cmd = Command::new(bin.as_ref());
    cmd.args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(CREATE_NO_WINDOW);
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    if let Some(env_list) = envs {
        for (k, v) in env_list {
            cmd.env(k, v);
        }
    }
    cmd
}

#[derive(Debug, Serialize)]
pub struct BackupInfo {
    pub fecha: String,
    pub entries: u32,
    pub db_version: String,
    pub pg_dump_version: String,
    pub db_name: String,
}

fn get_value_after_double_points(line: &str, split_pattern: &str) -> String {
    line.split(split_pattern)
        .nth(1)
        .unwrap_or("")
        .trim()
        .split('\r')
        .next()
        .unwrap_or("")
        .to_string()
}

pub trait Tasker {
    fn backup(&self, db_name: String, schema_name: String, on_event: &Channel<DownloadEvent>);

    fn restore(&self, restore_options: RestoreOptions, on_event: &Channel<DownloadEvent>);
    fn create_db(&self, restore_options: &RestoreOptions, on_event: &Channel<DownloadEvent>);
    fn list_db(&self) -> Result<Vec<String>, ()>;
    fn list_db_schemas(&self, db_name: String) -> Vec<String>;
    fn parse_backup(&self, path: String) -> Result<BackupInfo, ()>;
}

pub struct LocalTasker<'a> {
    pub data_source: &'a DataSource,
}

// fn run_command<P: AsRef<Path>>(
//     bin: P,
//     args: Vec<&str>,
//     cwd: Option<P>,
//     envs: Option<Vec<(String, String)>>,
// ) -> Cow<str> {
//     let out = spawn_builder(bin, args, cwd, envs).output().unwrap();
//      String::from_utf8_lossy(out.clone().stdout)
// }

impl<'a> LocalTasker<'a> {
    pub fn new(data_source: &'a DataSource) -> Self {
        Self { data_source }
    }
}

impl<'a> Tasker for LocalTasker<'a> {
    fn backup(&self, db_name: String, schema_name: String, on_event: &Channel<DownloadEvent>) {
        //`${dbName}${schemmaName}_${getFormattedDateTime()}.backup`
        let schema_suffix = if schema_name.is_empty() {
            String::new()
        } else {
            format!("_{}", schema_name)
        };
        let backup_path = format!(
            "C:/Users/rolan/pgrustore/backups/{}{}_{}.backup",
            db_name,
            schema_suffix,
            get_formatted_date_time()
        );
        let bin = format!("{}{}", self.data_source.bin, "/pg_dump.exe");

        let port = &self.data_source.port.to_string();
        //let params = `--file ${path.normalize(backupPath)} --host ${host} --port ${port} --username ${user} --format=c --verbose${schammaParams} ${dbName}`
        // 🔹 Construimos los argumentos
        let mut args = vec![
            "--file",
            &backup_path,
            "--host",
            &self.data_source.host,
            "--port",
            port,
            "--username",
            &self.data_source.user,
            "--format=c",
            "--verbose",
        ];

        if !schema_name.is_empty() {
            args.push("--schema");
            args.push(&schema_name);
        }
        // Finalmente el nombre de la base
        args.push(&db_name);

        let mut child = spawn_builder(
            bin,
            args,
            None,
            Some(vec![(
                "PGPASSWORD".into(),
                self.data_source.password.as_str().into(),
            )]),
        )
        .spawn()
        .unwrap();

        on_event
            .send(DownloadEvent::Started {
                msg: "iniciado".to_string(),
            })
            .unwrap();

        // Obtenemos el pipe de salida
        let stdout = child.stderr.take().expect("no se pudo capturar stdout");
        // Lo envolvemos en un BufReader para leer línea por línea
        let mut reader = BufReader::new(stdout);

        let mut buf = Vec::new();
        while let Ok(n) = reader.read_until(b'\n', &mut buf) {
            if n == 0 {
                break;
            }

            on_event
                .send(DownloadEvent::Progress {
                    msg: String::from_utf8_lossy(&buf).to_string(),
                })
                .unwrap();
            buf.clear();
        }
        // Esperamos a que termine el proceso
       // let status = child.wait().expect("error al esperar");

        on_event
            .send(DownloadEvent::Finished {
                msg: "finalizado".to_string(),
            })
            .unwrap();
    }

    fn restore(&self, restore_options: RestoreOptions, on_event: &Channel<DownloadEvent>) {
        on_event
            .send(DownloadEvent::Started {
                msg: "iniciado".to_string(),
            })
            .unwrap();
        if let Some(new_db_options) = &restore_options.new_db_options {
            println!(
                "crear db {} {} {}",
                new_db_options.encoding, new_db_options.template, restore_options.db_name
            );
            self.create_db(&restore_options, on_event);
        }

        let bin = format!("{}{}", self.data_source.bin, "/pg_restore.exe");
        let path = PathBuf::from(&restore_options.backup);
        let port = &self.data_source.port.to_string();

        // 🔹 Construimos los argumentos
        let args = vec![
            "--host",
            &self.data_source.host,
            "--port",
            port,
            "--username",
            &self.data_source.user,
            "--role",
            "postgres",
            "--dbname",
            &restore_options.db_name,
            "--verbose",
            path.to_str().unwrap(),
        ];

        let mut child = spawn_builder(
            bin,
            args,
            None,
            Some(vec![(
                "PGPASSWORD".into(),
                self.data_source.password.as_str().into(),
            )]),
        )
        .spawn()
        .unwrap();

        // Obtenemos el pipe de salida
        let stdout = child.stderr.take().expect("no se pudo capturar stdout");
        // Lo envolvemos en un BufReader para leer línea por línea
        let mut reader = BufReader::new(stdout);

        let mut buf = Vec::new();
        while let Ok(n) = reader.read_until(b'\n', &mut buf) {
            if n == 0 {
                break;
            }

            on_event
                .send(DownloadEvent::Progress {
                    msg: String::from_utf8_lossy(&buf).to_string(),
                })
                .unwrap();
            buf.clear();
        }
        // Esperamos a que termine el proceso
        //let status = child.wait().expect("error al esperar");

        on_event
            .send(DownloadEvent::Finished {
                msg: "finalizado".to_string(),
            })
            .unwrap();
    }
    fn create_db(&self, restore_options: &RestoreOptions, on_event: &Channel<DownloadEvent>) {
        let bin = format!("{}{}", self.data_source.bin, "/createdb.exe");
        let port = &self.data_source.port.to_string();
        //let params = `--file ${path.normalize(backupPath)} --host ${host} --port ${port} --username ${user} --format=c --verbose${schammaParams} ${dbName}`
        // 🔹 Construimos los argumentos
        let new_opts = restore_options.new_db_options.as_ref().unwrap();
        let args: Vec<&str> = vec![
            "--host",
            &self.data_source.host,
            "--port",
            port,
            "--username",
            &self.data_source.user,
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

        let mut child = spawn_builder(
            bin,
            args,
            None,
            Some(vec![(
                "PGPASSWORD".into(),
                self.data_source.password.as_str().into(),
            )]),
        )
        .spawn()
        .unwrap();

        // Obtenemos el pipe de salida
        let stdout = child.stderr.take().expect("no se pudo capturar stdout");
        // Lo envolvemos en un BufReader para leer línea por línea
        let mut reader = BufReader::new(stdout);

        let mut buf = Vec::new();
        while let Ok(n) = reader.read_until(b'\n', &mut buf) {
            if n == 0 {
                break;
            }

            on_event
                .send(DownloadEvent::Progress {
                    msg: String::from_utf8_lossy(&buf).to_string(),
                })
                .unwrap();
            buf.clear();
        }
        // Esperamos a que termine el proceso
       // let status = child.wait().expect("error al esperar");
    }

    fn list_db(&self) -> Result<Vec<String>, ()> {
        let bin = format!("{}{}", self.data_source.bin, "/psql.exe");
        let port = &self.data_source.port.to_string();
        //const command = `"${binary}\\${commands.psql}" -U ${user} --host ${host} --port ${port} -c "\\l"`
        // 🔹 Construimos los argumentos
        let args = vec![
            "-U",
            &self.data_source.user,
            "--host",
            &self.data_source.host,
            "--port",
            port,
            "-c",
            "\\l",
        ];


        match spawn_builder(
            bin,
            args,
            None,
            Some(vec![(
                "PGPASSWORD".into(),
                self.data_source.password.as_str().into(),
            )]),
        )
        .output()
        {
            Ok(out) => {
                if !out.status.success() || !out.stderr.is_empty() {
                    return Err(());
                }
                // error genérico

                let stdout_str = String::from_utf8_lossy(&out.stdout);
                let mut dbs: Vec<String> = Vec::new();

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
                            if !db_name.is_empty()
                                && db_name != "template0"
                                && db_name != "template1"
                            {
                                dbs.push(db_name.to_string());
                            }
                        }
                    }
                }

                Ok(dbs)
            }
            Err(_) => return Err(()),
        }
    }

    fn list_db_schemas(&self, db_name: String) -> Vec<String> {
        let bin = format!("{}{}", self.data_source.bin, "/psql.exe");
        let port = &self.data_source.port.to_string();
        //const command = `"${binary}\\${commands.psql}" -U ${username} --host ${host} -d ${dbName} --port ${port} -c "\\dn"`,
        // 🔹 Construimos los argumentos
        let args: Vec<&str> = vec![
            "-U",
            &self.data_source.user,
            "--host",
            &self.data_source.host,
            "-d",
            &db_name,
            "--port",
            port,
            "-c",
            "\\dn",
        ];
        let out = spawn_builder(
            bin,
            args,
            None,
            Some(vec![(
                "PGPASSWORD".into(),
                self.data_source.password.as_str().into(),
            )]),
        )
        .output()
        .unwrap();
        let stdout_str = String::from_utf8_lossy(&out.stdout);

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

    fn parse_backup(&self, path: String) -> Result<BackupInfo, ()> {
        let bin = format!("{}{}", self.data_source.bin, "/pg_restore.exe");

        let args: Vec<&str> = vec!["-l", &path];

        let out = spawn_builder(
            bin,
            args,
            None,
            Some(vec![(
                "PGPASSWORD".into(),
                self.data_source.password.as_str().into(),
            )]),
        )
        .output()
        .unwrap();

        let stdout = String::from_utf8_lossy(&out.stdout);

        let info_lines: Vec<&str> = stdout
            .lines()
            .filter(|line| line.trim().starts_with(';'))
            .collect();

        let mut info = BackupInfo {
            fecha: String::new(),
            entries: 0,
            db_version: String::new(),
            pg_dump_version: String::new(),
            db_name: String::new(),
        };

        for line in info_lines {
            if line.contains("pg_dump") {
                info.pg_dump_version = get_value_after_double_points(line, ":");
            } else if line.contains("database") {
                info.db_version = get_value_after_double_points(line, ":");
            } else if line.contains("Entries") && info.entries == 0 {
                info.entries = get_value_after_double_points(line, ":")
                    .parse::<u32>()
                    .unwrap_or(0);
            } else if line.contains("created at") {
                if let Some(rest) = line.split("created at").nth(1) {
                    info.fecha = rest.split('\r').next().unwrap_or("").trim().to_string();
                }
            } else if line.contains("dbname") {
                info.db_name = get_value_after_double_points(line, ":");
            }
        }

        Ok(info)
    }
}
fn get_formatted_date_time() -> String {
    let now = OffsetDateTime::now_local().unwrap();
    let format = format_description!("[year]_[month]_[day]_[hour][minute]");
    now.format(&format).unwrap()
}
// pub fn local_tasker_builder(ds_name: &str) -> LocalTasker {
//     let config: Config = load_config();
//     let ds = config
//         .datasources
//         .iter()
//         .find(|d| d.name == "local")
//         .unwrap();
//     LocalTasker::new(ds)
// }
