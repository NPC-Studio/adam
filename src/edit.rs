use std::{collections::HashMap, path::PathBuf, process::ExitCode};

use colored::Colorize;
use yy_boss::{
    yy_typings::{
        self,
        object_yy::{EventType, EventTypeConvertErrors, Object},
        script::Script,
        utils::TrailingCommaUtility,
        CommonData, ViewPath, ViewPathLocation,
    },
    Resource, YypBoss,
};

use crate::{
    igor,
    input::{ObjectEditRequest, ScriptEditRequest, Vfs},
};

pub fn handle_script(script: ScriptEditRequest) -> ExitCode {
    let Some(mut yyp_boss) = create_yyp_boss(YypBoss::without_resources) else {
        return ExitCode::FAILURE;
    };
    yyp_boss
        .quick_name()
        .expect("bad yyp entry -- couldn't add.");

    let Some(parent) = maybe_find_vfs_path(&yyp_boss, script.vfs) else {
        return ExitCode::FAILURE;
    };

    if let Err(e) = yyp_boss.add_resource(
        Script {
            common_data: CommonData::new(script.name.clone()),
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
            "{}: could not serialize {}",
            console::style("error").bright().red(),
            e
        );
        return ExitCode::FAILURE;
    }

    println!(
        "{}: ./scripts/{SCRIPT_NAME}/{SCRIPT_NAME}.gml",
        console::style("success").green().bright(),
        SCRIPT_NAME = script.name
    );

    ExitCode::SUCCESS
}

pub fn handle_object(object: ObjectEditRequest) -> ExitCode {
    let Some(mut yyp_boss) = create_yyp_boss(|p| YypBoss::new(p, &[Resource::Object])) else {
        return ExitCode::FAILURE;
    };

    let ObjectEditRequest {
        name,
        events,
        parent,
        sprite,
        vfs,
        visible,
        tags,
    } = object;

    yyp_boss
        .quick_name()
        .expect("bad yyp entry -- couldn't add.");

    let vfs = vfs.map(|v| match find_vfs_path(&yyp_boss, v) {
        Some(v) => v,
        None => {
            // we're OUTTA here!!
            std::process::exit(1);
        }
    });

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
    let event_result: Result<Vec<EventType>, EventTypeConvertErrors> = events
        .into_iter()
        .map(|event_name| EventType::parse_filename_heuristic(&event_name))
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

    let is_new = yyp_boss.objects.get(&name).is_none();

    if is_new {
        if let Err(e) = yyp_boss.add_resource(
            Object {
                common_data: CommonData::new(name.clone()),
                managed: true,
                persistent: false,
                visible: true,
                parent: yyp_boss.project_metadata().root_file.clone(),
                ..Default::default()
            },
            HashMap::new(),
        ) {
            println!("{}: {}", console::style("error").bright().red(), e);
            return ExitCode::FAILURE;
        }
    } else {
        yyp_boss
            .objects
            .load_resource_associated_data(
                &name,
                yyp_boss.directory_manager.root_directory(),
                &TrailingCommaUtility::new(),
            )
            .unwrap();
    }

    let obj_data = unsafe { yyp_boss.objects.get_mut(&name).unwrap() };

    if let Some(vfs) = &vfs {
        obj_data.yy_resource.parent = vfs.clone();
    }

    if let Some(sprite_id) = &sprite_id {
        obj_data.yy_resource.sprite_id = Some(sprite_id.clone());
    }
    if let Some(parent_object_id) = &parent_object_id {
        obj_data.yy_resource.parent_object_id = Some(parent_object_id.clone());
    }
    if let Some(vis) = visible {
        obj_data.yy_resource.visible = vis;
    }
    if let Some(tags) = &tags {
        obj_data.yy_resource.tags = tags.clone();
    }

    for event in event_list.iter().copied() {
        yyp_boss.objects.add_event(&name, event);
    }

    // and finally mark it for serialization
    yyp_boss.objects.mark_for_serialization(&name).unwrap();

    if let Err(e) = yyp_boss.serialize() {
        println!(
            "{}: could not serialize {}",
            console::style("error").bright().red(),
            e
        );
        return ExitCode::FAILURE;
    }

    println!(
        "{}: {}",
        if is_new { "created" } else { "edited" }.bright_green(),
        name
    );

    if let Some(vfs) = &vfs {
        println!("vfs_parent: `{}`", vfs.name.bold());
    }

    if let Some(sprite_id) = &sprite_id {
        println!("sprite: `{}`", sprite_id.name.bold());
    }
    if let Some(parent_object_id) = &parent_object_id {
        println!("parent: `{}`", parent_object_id.name.bold());
    }
    if let Some(vis) = visible {
        println!("visible: {}", vis.to_string().bold());
    }

    for event in event_list {
        let (ev_name, ev_num) = event.filename();
        println!(
            "{}: ./objects/{}/{}_{}.gml",
            "created".bright_green(),
            name,
            ev_name,
            ev_num
        );
    }

    ExitCode::SUCCESS
}

pub fn handle_vfs_request(vfs: Vfs) -> ExitCode {
    let Some(mut yyp_boss) = create_yyp_boss(|path_to_yyp| YypBoss::new(path_to_yyp, &[])) else {
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
        Vfs::Move {
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
        Vfs::Where { asset_name } => match yyp_boss.vfs.get_folder_by_fname(&asset_name) {
            Ok(v) => {
                println!("{}", v.view_path_location());
            }
            Err(_) => {
                println!("{}: `{}` does not exist", "error".bright_red(), asset_name);
            }
        },
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
        println!("{}: could not serialize {}", "error".bright_red(), e);
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
        println!("{}: could not serialize {}", "error".bright_red(), e);
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

fn maybe_find_vfs_path(yyp_boss: &YypBoss, input: Option<String>) -> Option<ViewPath> {
    match input {
        Some(vfs) => find_vfs_path(yyp_boss, vfs),
        None => Some(yyp_boss.project_metadata().root_file),
    }
}

fn find_vfs_path(yyp_boss: &YypBoss, vfs: String) -> Option<ViewPath> {
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
