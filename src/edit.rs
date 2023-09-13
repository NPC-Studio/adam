use std::{collections::HashMap, path::PathBuf, process::ExitCode};

use colored::Colorize;
use yy_boss::{
    yy_typings::{
        self,
        object_yy::{EventType, EventTypeConvertErrors, Object, ObjectEvent},
        script::Script,
        CommonData, ViewPath, ViewPathLocation,
    },
    Resource, YypBoss,
};

use crate::{
    igor,
    input::{Add, Vfs},
};

pub fn handle_add_request(kind: Add) -> ExitCode {
    let Some(mut yyp_boss) = create_yyp_boss(YypBoss::without_resources) else {
        return ExitCode::FAILURE;
    };
    yyp_boss
        .quick_name()
        .expect("bad yyp entry -- couldn't add.");

    match kind {
        Add::Script { name, vfs } => {
            let Some(parent) = find_vfs_path(&yyp_boss, vfs) else {
                return ExitCode::FAILURE;
            };

            if let Err(e) = yyp_boss.add_resource(
                Script {
                    common_data: CommonData::new(name.clone()),
                    is_compatibility: false,
                    is_dn_d: false,
                    parent,
                },
                String::new(),
            ) {
                println!("{}: {}", console::style("error").bright().red(), e);
                return ExitCode::FAILURE;
            }
            if let Err(e) = yyp_boss.serialize() {
                println!(
                    "{}: couldn't serialize {}",
                    console::style("error").bright().red(),
                    e
                );
                return ExitCode::FAILURE;
            }

            println!(
                "{}: ./scripts/{SCRIPT_NAME}/{SCRIPT_NAME}.gml",
                console::style("success").green().bright(),
                SCRIPT_NAME = name
            );
        }
        Add::Object {
            name,
            events,
            vfs,
            parent,
            sprite,
        } => {
            let Some(vfs) = find_vfs_path(&yyp_boss, vfs) else {
                return ExitCode::FAILURE;
            };

            let sprite_id = if let Some(sprite) = sprite {
                if let Some(sprite) = yyp_boss
                    .yyp()
                    .resources
                    .iter()
                    .find(|v| v.id.name == sprite)
                {
                    Some(sprite.id.clone())
                } else {
                    println!(
                        "{}: no sprite named `{}` found",
                        console::style("error").bright().red(),
                        sprite
                    );
                    return ExitCode::FAILURE;
                }
            } else {
                None
            };

            let parent_object_id = if let Some(parent_object_id) = parent {
                if let Some(parent_object_id) = yyp_boss
                    .yyp()
                    .resources
                    .iter()
                    .find(|v| v.id.name == parent_object_id)
                {
                    Some(parent_object_id.id.clone())
                } else {
                    println!(
                        "{}: no object named `{}` found",
                        console::style("error").bright().red(),
                        parent_object_id
                    );
                    return ExitCode::FAILURE;
                }
            } else {
                None
            };

            // okay now we transform the event list...
            let event_result: Result<Vec<ObjectEvent>, EventTypeConvertErrors> = events
                .into_iter()
                .map(|event_name| {
                    let event_type = EventType::parse_filename_simple(&event_name)?;

                    Ok(ObjectEvent {
                        common_data: CommonData::default(),
                        event_type,
                        collision_object_id: None,
                        is_dn_d: false,
                    })
                })
                .collect();

            let event_list = match event_result {
                Ok(v) => v,
                Err(e) => {
                    println!(
                        "{}: failed to parse event_name {}",
                        console::style("error").bright().red(),
                        e
                    );

                    return ExitCode::FAILURE;
                }
            };

            let event_data: HashMap<_, _> = event_list
                .iter()
                .map(|v| (v.event_type, String::new()))
                .collect();

            if let Err(e) = yyp_boss.add_resource(
                Object {
                    common_data: CommonData::new(name.clone()),
                    parent: vfs,
                    sprite_id,
                    parent_object_id,
                    managed: true,
                    persistent: false,
                    event_list,
                    visible: true,
                    ..Default::default()
                },
                event_data.clone(),
            ) {
                println!("{}: {}", console::style("error").bright().red(), e);
                return ExitCode::FAILURE;
            }
            if let Err(e) = yyp_boss.serialize() {
                println!(
                    "{}: couldn't serialize {}",
                    console::style("error").bright().red(),
                    e
                );
                return ExitCode::FAILURE;
            }

            if event_data.is_empty() {
                println!(
                    "{}: ./objects/{OBJECT_NAME}/{OBJECT_NAME}",
                    console::style("success").green().bright(),
                    OBJECT_NAME = name
                );
            } else {
                for event in event_data.into_keys() {
                    let (ev_name, ev_num) = event.filename();
                    println!(
                        "{}: ./objects/{}/{}_{}.gml",
                        console::style("success").green().bright(),
                        name,
                        ev_name,
                        ev_num
                    );
                }
            }
        }
    }

    ExitCode::SUCCESS
}

pub fn handle_vfs_request(vfs: Vfs) -> ExitCode {
    let Some(yyp_boss) = create_yyp_boss(|path_to_yyp| YypBoss::new(path_to_yyp, &[])) else {
        return ExitCode::FAILURE;
    };

    match vfs {
        Vfs::View { folder } => {
            let starter = folder
                .as_ref()
                .map(|v| format!("{}/", v).bright_blue())
                .unwrap_or_default();

            let root_folder = match folder {
                Some(root) => {
                    match yyp_boss
                        .vfs
                        .get_folder(&yy_typings::ViewPathLocation::new(format!(
                            "folders/{}.yy",
                            root
                        ))) {
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
    }
    ExitCode::SUCCESS
}

pub fn handle_remove_request(name: String) -> ExitCode {
    let Some(mut yyp_boss) = create_yyp_boss_with_data() else {
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
        Resource::Sprite => yyp_boss
            .remove_resource::<yy_typings::sprite_yy::Sprite>(&name)
            .err(),
        Resource::Script => yyp_boss.remove_resource::<Script>(&name).err(),
        Resource::Object => yyp_boss.remove_resource::<Object>(&name).err(),
        Resource::Note => yyp_boss.remove_resource::<yy_typings::Note>(&name).err(),
        Resource::Shader => yyp_boss
            .remove_resource::<yy_typings::shader::Shader>(&name)
            .err(),
        Resource::Sound => yyp_boss
            .remove_resource::<yy_typings::sound::Sound>(&name)
            .err(),
        Resource::Room => yyp_boss.remove_resource::<yy_typings::Room>(&name).err(),
        Resource::TileSet => yyp_boss.remove_resource::<yy_typings::TileSet>(&name).err(),
        Resource::AnimationCurve => yyp_boss
            .remove_resource::<yy_typings::AnimationCurve>(&name)
            .err(),
        Resource::Extension => yyp_boss
            .remove_resource::<yy_typings::Extension>(&name)
            .err(),
        Resource::Font => yyp_boss.remove_resource::<yy_typings::Font>(&name).err(),
        Resource::Path => yyp_boss.remove_resource::<yy_typings::Path>(&name).err(),
        Resource::Sequence => yyp_boss
            .remove_resource::<yy_typings::Sequence>(&name)
            .err(),
        Resource::Timeline => yyp_boss
            .remove_resource::<yy_typings::Timeline>(&name)
            .err(),
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
        println!("{}: couldn't serialize {}", "error".bright_red(), e);
        return ExitCode::FAILURE;
    }

    println!("{}: removed `{}`", "success".bright_green(), name,);
    ExitCode::SUCCESS
}

pub fn handle_rename_request(original_name: String, new_name: String) -> ExitCode {
    let Some(mut yyp_boss) = create_yyp_boss_with_data() else {
        return ExitCode::FAILURE;
    };

    let resource_kind = match yyp_boss.vfs.get_resource_type(&original_name) {
        Some(v) => v,
        None => {
            println!(
                "{}: `{}` does not exist",
                "error".bright_red(),
                original_name
            );
            return ExitCode::FAILURE;
        }
    };

    let to_boss = new_name.clone();

    let e = match resource_kind {
        Resource::Sprite => yyp_boss
            .rename_resource::<yy_typings::sprite_yy::Sprite>(&original_name, to_boss)
            .err(),
        Resource::Script => yyp_boss
            .rename_resource::<Script>(&original_name, to_boss)
            .err(),
        Resource::Object => yyp_boss
            .rename_resource::<Object>(&original_name, to_boss)
            .err(),
        Resource::Note => yyp_boss
            .rename_resource::<yy_typings::Note>(&original_name, to_boss)
            .err(),
        Resource::Shader => yyp_boss
            .rename_resource::<yy_typings::shader::Shader>(&original_name, to_boss)
            .err(),
        Resource::Sound => yyp_boss
            .rename_resource::<yy_typings::sound::Sound>(&original_name, to_boss)
            .err(),
        Resource::Room => yyp_boss
            .rename_resource::<yy_typings::Room>(&original_name, to_boss)
            .err(),
        Resource::TileSet => yyp_boss
            .rename_resource::<yy_typings::TileSet>(&original_name, to_boss)
            .err(),
        Resource::AnimationCurve => yyp_boss
            .rename_resource::<yy_typings::AnimationCurve>(&original_name, to_boss)
            .err(),
        Resource::Extension => yyp_boss
            .rename_resource::<yy_typings::Extension>(&original_name, to_boss)
            .err(),
        Resource::Font => yyp_boss
            .rename_resource::<yy_typings::Font>(&original_name, to_boss)
            .err(),
        Resource::Path => yyp_boss
            .rename_resource::<yy_typings::Path>(&original_name, to_boss)
            .err(),
        Resource::Sequence => yyp_boss
            .rename_resource::<yy_typings::Sequence>(&original_name, to_boss)
            .err(),
        Resource::Timeline => yyp_boss
            .rename_resource::<yy_typings::Timeline>(&original_name, to_boss)
            .err(),
    };

    if let Some(e) = e {
        println!(
            "{}: failed to rename `{}`, {}",
            "error".bright_red(),
            original_name,
            e
        );

        return ExitCode::FAILURE;
    }

    if let Err(e) = yyp_boss.serialize() {
        println!("{}: couldn't serialize {}", "error".bright_red(), e);
        return ExitCode::FAILURE;
    }

    if resource_kind == Resource::Script {
        println!(
            "{}: renamed `{}` to `./scripts/{}.gml`",
            "success".bright_green(),
            format!("./scripts/{}.gml", original_name).dimmed(),
            new_name,
        );
    } else {
        println!(
            "{}: renamed `{}` to `{}`",
            "success".bright_green(),
            original_name.dimmed(),
            new_name,
        );
    }

    ExitCode::SUCCESS
}

fn find_vfs_path(yyp_boss: &YypBoss, input: Option<String>) -> Option<ViewPath> {
    let parent = match input {
        Some(vfs) => {
            let path = ViewPathLocation(format!("folders/{}.yy", vfs));

            if yyp_boss.vfs.get_folder(&path).is_some() {
                // this is just a fancy way of getting the folder name basically.
                let utf8_path = camino::Utf8Path::new(path.0.as_str());
                let name = utf8_path.file_stem().unwrap().to_owned();

                yy_typings::ViewPath { name, path }
            } else {
                println!(
                    "{}: provided folder does not exist",
                    console::style("adam error").bright().red(),
                );
                return None;
            }
        }
        None => yyp_boss.project_metadata().root_file,
    };

    Some(parent)
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
