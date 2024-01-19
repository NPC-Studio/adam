use std::process::ExitCode;

use camino::{Utf8Path, Utf8PathBuf};
use colored::Colorize;
use yy_boss::{Resource, YypBoss};

pub fn reserialize() -> ExitCode {
    let Some(mut yyp_boss) = super::create_yyp_boss(|p| {
        YypBoss::new(
            p,
            &[
                Resource::Sprite,
                Resource::Script,
                Resource::Object,
                Resource::Note,
                Resource::Shader,
                Resource::Sound,
                Resource::Room,
                Resource::TileSet,
                Resource::AnimationCurve,
                Resource::Extension,
                Resource::Font,
                Resource::Path,
                Resource::Sequence,
                Resource::Timeline,
            ],
        )
    }) else {
        return ExitCode::FAILURE;
    };

    // more or less the only things we reserialize well
    let root_directory =
        Utf8PathBuf::from_path_buf(yyp_boss.directory_manager.root_directory().to_owned()).unwrap();
    read_and_mark(&mut yyp_boss.sprites, &root_directory);
    read_and_mark(&mut yyp_boss.scripts, &root_directory);
    read_and_mark(&mut yyp_boss.objects, &root_directory);

    if let Err(e) = yyp_boss.serialize() {
        println!(
            "{}: couldn't serialize yyp_boss because {}",
            "error".bright_red(),
            e
        );

        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn read_and_mark<T: yy_boss::YyResource>(
    holder: &mut yy_boss::YyResourceHandler<T>,
    root_directory: &Utf8Path,
) -> ExitCode {
    let names: Vec<String> = holder.resources().keys().cloned().collect();

    for sprite in names {
        if let Err(e) = holder.mark_for_serialization(&sprite) {
            println!(
                "{}: couldn't mark `{}` for serialization because {}",
                "error".bright_red(),
                sprite,
                e
            );
            return ExitCode::FAILURE;
        }
    }
    if std::fs::remove_dir_all(root_directory.join(T::RESOURCE.subpath_name())).is_err() {
        println!(
            "{}: couldn't remove {} folder",
            "error".bright_red(),
            T::RESOURCE.subpath_name()
        );

        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
