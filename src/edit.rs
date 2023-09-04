use std::{collections::HashMap, process::ExitCode};

use yy_boss::{
    yy_typings::{
        self,
        object_yy::{EventType, EventTypeConvertErrors, Object, ObjectEvent},
        script::Script,
        CommonData, ViewPath, ViewPathLocation,
    },
    YypBoss,
};

use crate::{
    igor,
    input::{Add, Vfs},
};

pub fn handle_add_request(kind: Add) -> ExitCode {
    let Some(mut yyp_boss) = create_yyp_boss() else {
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
    let Some(yyp_boss) = create_yyp_boss() else {
        return ExitCode::FAILURE;
    };

    match vfs {
        Vfs::View { folder } => {
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
                println!("{}", console::style(&sub_folder.name).bright().blue());
            }

            for file in root_folder.files.inner().iter() {
                println!("{}", file.name);
            }
        }
    }
    ExitCode::SUCCESS
}

fn find_vfs_path(yyp_boss: &YypBoss, input: Option<String>) -> Option<ViewPath> {
    let parent = match input {
        Some(vfs) => {
            let path = ViewPathLocation(format!("folders/{}.yy", vfs));

            if yyp_boss.vfs.get_folder(&path).is_some() {
                yy_typings::ViewPath { name: vfs, path }
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

fn create_yyp_boss() -> Option<yy_boss::YypBoss> {
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

    match yy_boss::YypBoss::without_resources(
        application_data
            .current_directory
            .join(format!("{}.yyp", application_data.project_name)),
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
