mod script;
use std::path::PathBuf;

pub use script::add_script;

mod object;
pub use object::{add_object, edit_manifest};

mod vfs;
use colored::Colorize;
pub use vfs::{remove, rename, vfs_request};
use yy_boss::YypBoss;
use yy_typings::{ViewPath, ViewPathLocation};

use crate::igor;

fn maybe_find_vfs_path(yyp_boss: &YypBoss, input: Option<String>) -> Option<ViewPath> {
    match input {
        Some(vfs) => find_vfs_path(yyp_boss, &vfs),
        None => Some(yyp_boss.project_metadata().root_file),
    }
}

fn find_vfs_path(yyp_boss: &YypBoss, vfs: &str) -> Option<ViewPath> {
    let path = ViewPathLocation(format!("folders/{}.yy", vfs));

    if yyp_boss.vfs.get_folder(&path).is_some() {
        // this is just a fancy way of getting the folder name basically.
        let utf8_path = camino::Utf8Path::new(path.0.as_str());
        let name = utf8_path.file_stem().unwrap().to_owned();

        Some(yy_typings::ViewPath { name, path })
    } else {
        println!(
            "{}: folder `{}` does not exist",
            "adam error".bright_red(),
            path.to_string().bold(),
        );
        None
    }
}

fn create_yyp_boss(
    mut make_yyp_boss: impl FnMut(PathBuf) -> Result<YypBoss, yy_boss::StartupError>,
) -> Option<yy_boss::YypBoss> {
    let application_data = match igor::ApplicationData::new() {
        Ok(v) => v,
        Err(e) => {
            println!(
                "{}: {}",
                console::style("adam error").bright().red(),
                console::style(e).bold()
            );

            return None;
        }
    };

    let output = make_yyp_boss(
        application_data
            .current_directory
            .join(format!("{}.yyp", application_data.project_name)),
    );

    match output {
        Ok(v) => Some(v),
        Err(e) => {
            println!(
                "{}: couldn't read yyp {}",
                console::style("adam error").bright().red(),
                console::style(e).bold()
            );
            None
        }
    }
}

fn create_yyp_boss_with_data() -> Option<yy_boss::YypBoss> {
    let application_data = match igor::ApplicationData::new() {
        Ok(v) => v,
        Err(e) => {
            println!(
                "{}: {}",
                console::style("adam error").bright().red(),
                console::style(e).bold()
            );

            return None;
        }
    };

    match yy_boss::YypBoss::new(
        application_data
            .current_directory
            .join(format!("{}.yyp", application_data.project_name)),
        &[],
    ) {
        Ok(v) => Some(v),
        Err(e) => {
            println!(
                "{}: couldn't read yyp {}",
                console::style("adam error").bright().red(),
                console::style(e).bold()
            );
            None
        }
    }
}
