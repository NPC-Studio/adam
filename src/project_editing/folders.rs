use std::process::ExitCode;

use colored::Colorize;
use yy_boss::{Resource, YypBoss};
use yy_typings::{
    AnimationCurve, Extension, Font, Note, Object, Path, Room, Script, Sequence, Shader, Sound,
    Sprite, TileSet, Timeline, ViewPathLocation,
};

use crate::input::FolderRequest;

pub fn folder_request(vfs: FolderRequest) -> ExitCode {
    let Some(mut yyp_boss) = super::create_yyp_boss(|path_to_yyp| YypBoss::new(path_to_yyp, &[]))
    else {
        return ExitCode::FAILURE;
    };

    match vfs {
        FolderRequest::View { folder } => {
            let starter = folder
                .as_ref()
                .map(|v| format!("{}/", v).bright_blue())
                .unwrap_or_default();

            let root_folder = match folder {
                Some(root) => {
                    match yyp_boss
                        .vfs
                        .get_folder(&ViewPathLocation::new(format!("folders/{}.yy", root)))
                    {
                        Some(v) => v,
                        None => {
                            println!(
                                "{}: provided folder does not exist",
                                console::style("adam error").bright().red(),
                            );
                            return ExitCode::FAILURE;
                        }
                    }
                }
                None => yyp_boss.vfs.get_root_folder(),
            };

            for sub_folder in root_folder.folders.iter() {
                println!("{}{}", starter, &sub_folder.name.bright_blue());
            }

            for file in root_folder.files.inner().iter() {
                println!("{}{}", starter, &file.name);
            }
        }
        FolderRequest::Move {
            target,
            new_directory,
        } => {
            let resource_kind = match yyp_boss.vfs.get_resource_type(&target) {
                Some(v) => v,
                None => {
                    println!("{}: `{}` does not exist", "error".bright_red(), target);
                    return ExitCode::FAILURE;
                }
            };

            match yyp_boss.vfs.move_resource(
                &target,
                resource_kind,
                &ViewPathLocation(new_directory),
            ) {
                Ok(()) => {}
                Err(e) => {
                    println!("{}: could not move file: {}", "error".bright_red(), e);
                    return ExitCode::FAILURE;
                }
            }

            if let Err(e) = yyp_boss.serialize() {
                println!(
                    "{}: could not serialize {}",
                    console::style("error").bright().red(),
                    e
                );
                return ExitCode::FAILURE;
            }
        }
        FolderRequest::Where { asset_name } => {
            match yyp_boss.vfs.get_folder_by_fname(&asset_name) {
                Ok(v) => {
                    println!("{}", v.view_path_location());
                }
                Err(_) => {
                    println!("{}: `{}` does not exist", "error".bright_red(), asset_name);
                }
            }
        }
    }
    ExitCode::SUCCESS
}

pub fn remove(name: String) -> ExitCode {
    let Some(mut yyp_boss) = super::create_yyp_boss_with_data() else {
        return ExitCode::FAILURE;
    };

    let resource_kind = match yyp_boss.vfs.get_resource_type(&name) {
        Some(v) => v,
        None => {
            println!("{}: `{}` does not exist", "error".bright_red(), name);
            return ExitCode::FAILURE;
        }
    };

    let e = match resource_kind {
        Resource::Sprite => yyp_boss.remove_resource::<Sprite>(&name).err(),
        Resource::Script => yyp_boss.remove_resource::<Script>(&name).err(),
        Resource::Object => yyp_boss.remove_resource::<Object>(&name).err(),
        Resource::Note => yyp_boss.remove_resource::<Note>(&name).err(),
        Resource::Shader => yyp_boss.remove_resource::<Shader>(&name).err(),
        Resource::Sound => yyp_boss.remove_resource::<Sound>(&name).err(),
        Resource::Room => yyp_boss.remove_resource::<Room>(&name).err(),
        Resource::TileSet => yyp_boss.remove_resource::<TileSet>(&name).err(),
        Resource::AnimationCurve => yyp_boss.remove_resource::<AnimationCurve>(&name).err(),
        Resource::Extension => yyp_boss.remove_resource::<Extension>(&name).err(),
        Resource::Font => yyp_boss.remove_resource::<Font>(&name).err(),
        Resource::Path => yyp_boss.remove_resource::<Path>(&name).err(),
        Resource::Sequence => yyp_boss.remove_resource::<Sequence>(&name).err(),
        Resource::Timeline => yyp_boss.remove_resource::<Timeline>(&name).err(),
    };

    if let Some(e) = e {
        println!(
            "{}: failed to remove `{}`, {}",
            "error".bright_red(),
            name,
            e
        );

        return ExitCode::FAILURE;
    }

    if let Err(e) = yyp_boss.serialize() {
        println!("{}: could not serialize {}", "error".bright_red(), e);
        return ExitCode::FAILURE;
    }

    println!("{}: removed `{}`", "success".bright_green(), name,);
    ExitCode::SUCCESS
}

pub fn rename(original: String, new_name: String) -> ExitCode {
    let Some(mut yb) = super::create_yyp_boss_with_data() else {
        return ExitCode::FAILURE;
    };

    let resource_kind = match yb.vfs.get_resource_type(&original) {
        Some(v) => v,
        None => {
            println!("{}: `{}` does not exist", "error".bright_red(), original);
            return ExitCode::FAILURE;
        }
    };

    let new = new_name.clone();

    let e = match resource_kind {
        Resource::Sprite => yb.rename_resource::<Sprite>(&original, new).err(),
        Resource::Script => yb.rename_resource::<Script>(&original, new).err(),
        Resource::Object => yb.rename_resource::<Object>(&original, new).err(),
        Resource::Note => yb.rename_resource::<Note>(&original, new).err(),
        Resource::Shader => yb.rename_resource::<Shader>(&original, new).err(),
        Resource::Sound => yb.rename_resource::<Sound>(&original, new).err(),
        Resource::Room => yb.rename_resource::<Room>(&original, new).err(),
        Resource::TileSet => yb.rename_resource::<TileSet>(&original, new).err(),
        Resource::AnimationCurve => yb.rename_resource::<AnimationCurve>(&original, new).err(),
        Resource::Extension => yb.rename_resource::<Extension>(&original, new).err(),
        Resource::Font => yb.rename_resource::<Font>(&original, new).err(),
        Resource::Path => yb.rename_resource::<Path>(&original, new).err(),
        Resource::Sequence => yb.rename_resource::<Sequence>(&original, new).err(),
        Resource::Timeline => yb.rename_resource::<Timeline>(&original, new).err(),
    };

    if let Some(e) = e {
        println!(
            "{}: failed to rename `{}`, {}",
            "error".bright_red(),
            original,
            e
        );

        return ExitCode::FAILURE;
    }

    if let Err(e) = yb.serialize() {
        println!("{}: could not serialize {}", "error".bright_red(), e);
        return ExitCode::FAILURE;
    }

    if resource_kind == Resource::Script {
        println!(
            "{}: renamed `{}` to `./scripts/{}.gml`",
            "success".bright_green(),
            format!("./scripts/{}.gml", original).dimmed(),
            new_name,
        );
    } else {
        println!(
            "{}: renamed `{}` to `{}`",
            "success".bright_green(),
            original.dimmed(),
            new_name,
        );
    }

    ExitCode::SUCCESS
}
