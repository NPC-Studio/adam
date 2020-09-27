use filetime::FileTime;
use indicatif::ProgressBar;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, PartialEq, Eq, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Manifest {
    config: String,
    files: std::collections::HashMap<String, i64>,
}

impl Manifest {
    pub fn new(config: String) -> Self {
        Manifest {
            config,
            files: Default::default(),
        }
    }
}

pub fn check_manifest(
    config: String,
    yyp_dir: &Path,
    manifest_dir: &Path,
    target_directory: &Path,
) -> bool {
    let manifest_path = manifest_dir.join(".manifest.toml");

    let old_manifest: Manifest = std::fs::read_to_string(&manifest_path)
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default();

    let mut new_manifest: Manifest = Manifest::new(config);

    let progress_bar = ProgressBar::new(3000);
    progress_bar.set_draw_target(indicatif::ProgressDrawTarget::stdout());
    progress_bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("Building cache manifest {spinner:.blue} [{bar:40.cyan/blue}]")
            .progress_chars("#> "),
    );

    for entry in WalkDir::new(yyp_dir)
        .into_iter()
        .filter_entry(|e| is_hidden(e) == false && e.path() != target_directory)
        .filter_map(|e| e.ok())
    {
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_file() {
                let last_accesstime = FileTime::from_last_modification_time(&metadata).seconds();

                let pathname = entry.path().to_string_lossy().to_string();
                new_manifest.files.insert(pathname.clone(), last_accesstime);
            }
        }

        progress_bar.inc(1);
    }

    progress_bar.finish_and_clear();

    if new_manifest != old_manifest {
        if let Err(e) = std::fs::write(
            &manifest_path,
            toml::to_string_pretty(&new_manifest).unwrap(),
        ) {
            println!(
                "{}: couldn't save manifest, {}",
                console::style("warn").yellow(),
                e
            )
        }
        false
    } else {
        true
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

pub fn invalidate_manifest(manifest_dir: &Path) {
    let manifest_path = manifest_dir.join(".manifest.toml");
    match std::fs::remove_file(manifest_path) {
        Ok(()) => {}
        Err(e) => println!(
            "{}: we couldn't clear the manifest because {}. please run `adam clean`.",
            console::style("adam error").red().bright(),
            console::style(e).white()
        ),
    }
}
