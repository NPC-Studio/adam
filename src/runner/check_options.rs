use std::process::Command;

use camino::Utf8PathBuf;

#[cfg(target_os = "windows")]
fn harness_check(path_to_run: Utf8PathBuf) -> Command {
    let current_dir = Utf8PathBuf::from_path_buf(std::env::current_dir().unwrap()).unwrap();

    let mut cmd = Command::new("powershell");
    cmd.arg("-ExecutionPolicy")
        .arg("RemoteSigned")
        .arg("-File")
        .arg(current_dir.join(&path_to_run));

    cmd
}

#[cfg(not(target_os = "windows"))]
fn harness_check(path_to_run: Utf8PathBuf) -> Command {
    let current_dir = std::env::current_dir().unwrap();
    let path = current_dir.join(path_to_run);
    Command::new(path)
}

/// Run the check option
pub fn run_check(path_to_run: Utf8PathBuf) -> Result<(), ()> {
    let mut cmd = harness_check(path_to_run);
    let output = cmd.output().expect("Failed to execute command");

    if let Ok(value) = String::from_utf8(output.stderr) {
        if !value.is_empty() {
            print!("{value}");
        }
    }
    if let Ok(value) = String::from_utf8(output.stdout) {
        if !value.is_empty() {
            print!("{value}");
        }
    }

    if output.status.success() {
        Ok(())
    } else {
        println!(
            "{}: check FAILED with {}",
            console::style("adam error").bright().red(),
            output.status
        );

        Err(())
    }
}
