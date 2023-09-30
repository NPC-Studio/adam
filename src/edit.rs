use std::{collections::HashMap, path::PathBuf, process::ExitCode};

use camino::Utf8Path;
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

pub fn handle_object(request: ObjectEditRequest) -> ExitCode {
    let Some(mut yyp_boss) = create_yyp_boss(|p| YypBoss::new(p, &[Resource::Object])) else {
        return ExitCode::FAILURE;
    };

    yyp_boss
        .quick_name()
        .expect("bad yyp entry -- couldn't add.");

    let vfs = request.folder.map(|v| match find_vfs_path(&yyp_boss, v) {
        Some(v) => v,
        None => {
            // we're OUTTA here!!
            std::process::exit(1);
        }
    });

    let sprite_id = if let Some(sprite) = request.sprite {
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

    let parent_object_id = if let Some(parent_object_id) = request.parent {
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
    let event_result: Result<Vec<EventType>, EventTypeConvertErrors> = request
        .events
        .into_iter()
        .map(|event_name| EventType::from_human_readable(&event_name))
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

    let is_new = yyp_boss.objects.get(&request.name).is_none();

    if is_new {
        if let Err(e) = yyp_boss.add_resource(
            Object {
                common_data: CommonData::new(request.name.clone()),
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
                &request.name,
                yyp_boss.directory_manager.root_directory(),
                &TrailingCommaUtility::new(),
            )
            .unwrap();
    }

    let obj_data = unsafe { yyp_boss.objects.get_mut(&request.name).unwrap() };

    if let Some(vfs) = &vfs {
        obj_data.yy_resource.parent = vfs.clone();
    }

    if let Some(sprite_id) = &sprite_id {
        obj_data.yy_resource.sprite_id = Some(sprite_id.clone());
    }
    if let Some(parent_object_id) = &parent_object_id {
        obj_data.yy_resource.parent_object_id = Some(parent_object_id.clone());
    }
    if let Some(vis) = request.visible {
        obj_data.yy_resource.visible = vis;
    }
    if let Some(tags) = &request.tags {
        obj_data.yy_resource.tags = tags.clone();
    }

    for event in event_list.iter().copied() {
        yyp_boss.objects.add_event(&request.name, event);
    }

    // and finally mark it for serialization
    yyp_boss
        .objects
        .mark_for_serialization(&request.name)
        .unwrap();

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
        request.name
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
    if let Some(vis) = request.visible {
        println!("visible: {}", vis.to_string().bold());
    }

    for event in event_list {
        let (ev_name, ev_num) = event.filename();
        println!(
            "{}: ./objects/{}/{}_{}.gml",
            "created".bright_green(),
            request.name,
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

pub fn edit(name: String, view: bool, target_folder: &Utf8Path) -> ExitCode {
    let Some(mut yyp_boss) = create_yyp_boss(|p| YypBoss::new(p, &[Resource::Object])) else {
        return ExitCode::FAILURE;
    };

    yyp_boss
        .quick_name()
        .expect("bad yyp entry -- couldn't add.");

    if let Some(resource_type) = yyp_boss.vfs.get_resource_type(&name) {
        if resource_type != Resource::Object {
            println!(
                "{}: `{}` is not an Object. only objects are currently supported.",
                "error".bright_red(),
                name.bold()
            );
            return ExitCode::FAILURE;
        }
    }

    if view {
        let Some(object_data) = yyp_boss.objects.get(&name) else {
            println!(
                "{}: cannot view `{}`: object not found.",
                "error".bright_red(),
                name.bold()
            );

            return ExitCode::FAILURE;
        };
        let configuration = gm_object_to_object_configuration(&object_data.yy_resource);

        let txt = manifest_maker(
            configuration,
            (include_str!("../assets/object_view_manifest.toml"))
                .parse()
                .unwrap(),
        );

        if target_folder.exists() == false && std::fs::create_dir_all(target_folder).is_err() {
            println!("{}: couldn't make target folder", "error".bright_red());
        }

        let path = target_folder.join("object_manifest.toml");
        std::fs::write(&path, txt).unwrap();

        let o = std::process::Command::new("code").arg(path).output();

        if o.is_err() {
            println!("{}: couldn't spawn child process", "error".bright_red());
        }

        return ExitCode::SUCCESS;
    }

    todo!()
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

fn gm_object_to_object_configuration(
    gm_object: &yy_typings::object_yy::Object,
) -> ObjectEditRequest {
    ObjectEditRequest {
        name: gm_object.common_data.name.clone(),
        events: gm_object
            .event_list
            .iter()
            .map(|v| v.event_type.to_human_readable())
            .collect(),
        parent: gm_object.parent_object_id.as_ref().map(|v| v.name.clone()),
        sprite: gm_object.sprite_id.as_ref().map(|v| v.name.clone()),
        folder: if gm_object.parent.path.0.starts_with("folders/") {
            Some(gm_object.parent.name.clone())
        } else {
            None
        },
        visible: Some(gm_object.visible),
        tags: Some(gm_object.tags.clone()),
    }
}

fn manifest_maker(edit_request: ObjectEditRequest, mut base_doc: toml_edit::Document) -> String {
    let d = toml::Value::try_from(edit_request.clone()).unwrap();
    let d_map = d.as_table().unwrap();

    for (k, v) in d_map {
        if k == "events" {
            continue;
        }
        let v = toml_value_to_toml_edit_value(v.clone());
        base_doc[k] = toml_edit::value(v);
    }

    // handling the events is easier with just replace tools
    let mut base_doc = base_doc.to_string();

    let events = d_map.get("events").unwrap();
    let events: Vec<&str> = events
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();

    for event in events {
        base_doc = base_doc.replace(&format!("# \"{}\"", event), &format!("\"{}\"", event));
    }

    base_doc
}

fn toml_value_to_toml_edit_value(value: toml::Value) -> toml_edit::Value {
    match value {
        toml::Value::String(v) => toml_edit::Value::String(toml_edit::Formatted::new(v)),
        toml::Value::Integer(v) => toml_edit::Value::Integer(toml_edit::Formatted::new(v)),
        toml::Value::Float(v) => toml_edit::Value::Float(toml_edit::Formatted::new(v)),
        toml::Value::Boolean(v) => toml_edit::Value::Boolean(toml_edit::Formatted::new(v)),
        toml::Value::Datetime(v) => toml_edit::Value::Datetime(toml_edit::Formatted::new(v)),
        toml::Value::Array(v) => {
            let inner = v.into_iter().map(toml_value_to_toml_edit_value).collect();

            toml_edit::Value::Array(inner)
        }
        toml::Value::Table(v) => toml_edit::Value::InlineTable(toml_edit::InlineTable::from_iter(
            v.into_iter()
                .map(|(k, v)| (k, toml_value_to_toml_edit_value(v))),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest() {
        let txt = include_str!("../assets/object_view_manifest.toml");
        let _: ObjectEditRequest = toml::from_str(txt).unwrap();

        let txt = include_str!("../assets/object_edit_manifest.toml");
        let _: ObjectEditRequest = toml::from_str(txt).unwrap();
    }

    #[test]
    fn edit_manifest() {
        let edit_request = ObjectEditRequest {
            name: "obj_player".to_string(),
            events: vec![
                "create".to_string(),
                "animation_end".to_string(),
                "destroy".to_string(),
            ],
            parent: Some("obj_depth".to_string()),
            sprite: Some("spr_player".to_string()),
            folder: Some("Objects/Player".to_string()),
            visible: Some(true),
            tags: Some(vec!["Dungeon".to_string()]),
        };

        let doc = manifest_maker(
            edit_request.clone(),
            (include_str!("../assets/object_edit_manifest.toml"))
                .parse()
                .unwrap(),
        );

        let obj_edit_request: ObjectEditRequest = toml::from_str(&doc).unwrap();

        assert_eq!(obj_edit_request, edit_request);
    }
}
