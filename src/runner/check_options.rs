use std::process::{Command, Output};

use camino::Utf8PathBuf;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct CheckOptions {
    pub path_to_run: Utf8PathBuf,
    pub directory_to_use: Option<Utf8PathBuf>,
}

/// Run the check option
#[cfg(not(target_os = "windows"))]
pub fn run_check(check_options: CheckOptions) -> Result<(), Output> {
    let current_dir = std::env::current_dir().unwrap();
    let path = current_dir.join(&check_options.path_to_run);
    let mut cmd = Command::new(path);

    if let Some(d2u) = check_options.directory_to_use {
        let dir_to_use = current_dir.join(d2u);
        cmd.current_dir(dir_to_use);
    }

    let output = cmd.output().expect("Failed to execute command");

    if output.status.success() {
        Ok(())
    } else {
        Err(output)
    }
}

/// Run the check option
#[cfg(target_os = "windows")]
pub fn run_check(check_options: CheckOptions) -> Result<(), Output> {
    let current_dir = Utf8PathBuf::from_path_buf(std::env::current_dir().unwrap()).unwrap();

    let mut cmd = Command::new("powershell");
    cmd.arg("-ExecutionPolicy")
        .arg("RemoteSigned")
        .arg("-File")
        .arg(current_dir.join(&check_options.path_to_run));

    if let Some(d2u) = check_options.directory_to_use {
        let dir_to_use = current_dir.join(&d2u);
        cmd.current_dir(dir_to_use);
    }

    let output = cmd.output().expect("Failed to execute command");

    if output.status.success() {
        Ok(())
    } else {
        Err(output)
    }
}
