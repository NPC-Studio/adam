use crossbeam::channel;
use filetime::FileTime;
use indicatif::ProgressBar;
use std::{
    collections::HashSet,
    hash::Hasher,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Debug, PartialEq, Eq, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Manifest {
    config: String,
    fingerprint: HashSet<u64>,
}

impl Manifest {
    pub fn new(config: String) -> Self {
        Manifest {
            config,
            fingerprint: HashSet::new(),
        }
    }
}

pub fn check_manifest(
    config: String,
    yyp_dir: &Path,
    manifest_dir: &Path,
    target_directory: &Path,
) -> bool {
    let manifest_path = manifest_dir.join(".manifest.json");

    let old_manifest: Manifest = std::fs::read_to_string(&manifest_path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();

    let mut new_manifest: Manifest = Manifest::new(config);
    let progress_bar = ProgressBar::new(1000);
    progress_bar.set_draw_target(indicatif::ProgressDrawTarget::stdout());
    progress_bar.set_style(
        indicatif::ProgressStyle::default_bar().template("Building cache manifest {spinner:.blue}"),
    );
    progress_bar.enable_steady_tick(1);

    let (s, r) = channel::unbounded();
    let mut threads_made = 0;

    // iterate over EACH file in the directory, giving us SOME parallelism...
    for entry in std::fs::read_dir(yyp_dir)
        .unwrap()
        .filter_map(|v| v.ok())
        .filter(is_not_hidden)
    {
        let path = entry.path();
        if path == target_directory {
            continue;
        }
        threads_made += 1;
        quick_thread(path, s.clone());
    }

    // sleep for 3 ms so we don't contend with ourselves too fast...
    std::thread::sleep(std::time::Duration::new(0, 3_000_000));

    // each thread is done...
    while let Ok(new_value) = r.recv() {
        new_manifest.fingerprint.insert(new_value);
        threads_made -= 1;
        if threads_made == 0 {
            break;
        }
    }

    progress_bar.finish_and_clear();

    if new_manifest != old_manifest {
        if let Err(e) = std::fs::write(
            &manifest_path,
            serde_json::to_string(&new_manifest).unwrap(),
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

fn is_not_hidden(entry: &std::fs::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(true)
        == false
}

pub fn invalidate_manifest(manifest_dir: &Path) {
    let manifest_path = manifest_dir.join(".manifest.json");
    match std::fs::remove_file(manifest_path) {
        Ok(()) => {}
        Err(e) => println!(
            "{}: we couldn't clear the manifest because {}. please run `adam clean`.",
            console::style("adam error").red().bright(),
            console::style(e).white()
        ),
    }
}

fn quick_thread(path: PathBuf, handle: channel::Sender<u64>) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let mut hasher = rustc_hash::FxHasher::default();

        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    let last_accesstime =
                        FileTime::from_last_modification_time(&metadata).seconds();

                    hasher.write_i64(last_accesstime);
                }
            }
        }

        handle.send(hasher.finish()).unwrap();
    })
}
