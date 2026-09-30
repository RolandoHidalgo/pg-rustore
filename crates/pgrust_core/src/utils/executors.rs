use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio, ExitStatus};

// 🔹 1. Constructor del comando (sin ejecutarlo aún)
pub fn build_command<P: AsRef<Path>>(
    bin: P,
    args: &[&str], // Usar slice es más eficiente que Vec
    cwd: Option<P>,
    envs: Option<Vec<(String, String)>>,
) -> Command {
    let mut cmd = Command::new(bin.as_ref());

    cmd.args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // CREATE_NO_WINDOW es solo de Windows. Esto evita errores de compilación en otros OS.
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

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

// 🔹 2. Ejecución en modo STREAMING (Caso 1: Progreso línea por línea)
pub fn execute_streaming<F>(
    mut cmd: Command,
    mut on_line: F,
) -> Result<ExitStatus, Box<dyn std::error::Error>>
where
    F: FnMut(String),
{
    let mut child = cmd.spawn().map_err(|e| format!("Error al iniciar proceso: {}", e))?;

    // Las herramientas de Postgres (pg_restore, createdb) envían el progreso (--verbose) 
    // por STDERR, no por STDOUT. STDOUT es para los datos reales.
    let stderr = child.stderr.take().expect("stderr no fue configurado como piped");
    let mut reader = BufReader::new(stderr);
    let mut buf = Vec::new();

    while let Ok(n) = reader.read_until(b'\n', &mut buf) {
        if n == 0 { break; }

        let line = String::from_utf8_lossy(&buf).to_string();
        on_line(line); // Notificamos al closure
        buf.clear();
    }

    let status = child.wait().expect("Error al esperar el proceso");

    if !status.success() {
        return Err(format!("El proceso falló con código: {:?}", status.code()).into());
    }

    Ok(status)
}

// 🔹 (Opcional) Definir un error propio para tu crate es mejor que usar `()`
#[derive(Debug)]
pub enum DbError {
    SpawnError(String),
    ExecutionError(String),
    ParseError(String),
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::SpawnError(e) => write!(f, "Error al iniciar: {}", e),
            DbError::ExecutionError(e) => write!(f, "Error de ejecución: {}", e),
            DbError::ParseError(e) => write!(f, "Error de parseo: {}", e),
        }
    }
}
impl std::error::Error for DbError {}