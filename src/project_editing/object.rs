use std::{collections::HashMap, fs, process::ExitCode};

use camino::Utf8Path;
use colored::Colorize;
use yy_boss::{Resource, YypBoss};
use yy_typings::{CommonData, EventType, EventTypeConvertErrors, Object, ObjectEvent};

use crate::input::ObjectEditRequest;

pub fn add_object(request: ObjectEditRequest) -> ExitCode {
    let Some(mut yyp_boss) = super::create_yyp_boss(|p| YypBoss::new(p, &[Resource::Object]))
    else {
        return ExitCode::FAILURE;
    };

    yyp_boss
        .quick_name()
        .expect("bad yyp entry -- couldn't add.");

    let vfs = request
        .folder
        .map(|v| match super::find_vfs_path(&yyp_boss, &v) {
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
                "error".bright_red(),
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
                "error".bright_red(),
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
            println!("{}: failed to parse event_name {}", "error".bright_red(), e);

            return ExitCode::FAILURE;
        }
    };

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
        if e == yy_boss::ResourceManipulationError::NameCollision {
            println!(
                "{}: resource already exists. try `adam edit {}` instead.",
                "error".bright_red(),
                request.name
            );
        } else {
            println!("{}: {}", "error".bright_red(), e);
        }

        return ExitCode::FAILURE;
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
        println!("{}: could not serialize {}", "error".bright_red(), e);
        return ExitCode::FAILURE;
    }

    println!("{}: {}", "created".bright_green(), request.name);

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
        println!(
            "{}: ./objects/{}/{}.gml",
            "created".bright_green(),
            request.name,
            event.filename()
        );
    }

    ExitCode::SUCCESS
}

pub fn edit_manifest(name: String, view: bool, target_folder: &Utf8Path) -> ExitCode {
    let Some(mut yyp_boss) = super::create_yyp_boss(|p| YypBoss::new(p, &[Resource::Object]))
    else {
        return ExitCode::FAILURE;
    };

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

    if yyp_boss.objects.get(&name).is_none() {
        println!(
            "{}: cannot edit `{}`: object not found.",
            "error".bright_red(),
            name.bold()
        );

        return ExitCode::FAILURE;
    }

    let configuration = match yyp_boss.objects.get(&name) {
        Some(object_data) => gm_object_to_object_configuration(&object_data.yy_resource),
        None => ObjectEditRequest {
            name: name.clone(),
            ..Default::default()
        },
    };

    let doc_str = if view {
        let mut doc = include_str!("../../assets/object_view_manifest.toml")
            .parse()
            .unwrap();
        manifest_maker(&configuration, &mut doc);
        let arr = toml_edit::Array::from_iter(configuration.events);
        doc["events"] = toml_edit::value(arr);
        doc.to_string()
    } else {
        let mut doc: toml_edit::Document = include_str!("../../assets/object_edit_manifest.toml")
            .parse()
            .unwrap();

        let ObjectEditRequest {
            name,
            events,
            parent,
            sprite,
            folder,
            visible,
            tags,
            mask_index,
        } = configuration;

        // we always override these fields
        doc["name"] = toml_edit::value(name);
        doc["visible"] = toml_edit::value(visible.unwrap());

        if let Some(tags) = tags {
            let arr = toml_edit::Array::from_iter(tags);
            doc["tags"] = toml_edit::value(arr);
        }

        // positive assignations...
        if let Some(parent) = &parent {
            doc["parent"] = toml_edit::value(parent);
        }
        if let Some(sprite) = &sprite {
            doc["sprite"] = toml_edit::value(sprite);
        }
        if let Some(mask_index) = &mask_index {
            doc["mask_index"] = toml_edit::value(mask_index);
        }
        if let Some(folder) = &folder {
            doc["folder"] = toml_edit::value(folder);
        } else {
            doc["folder"] = toml_edit::value("root_folder");
        }

        // first, we need to handle events. Do we have any events which AREN'T listed?
        let event_array = doc["events"].as_array_mut().unwrap();
        for e in events.iter().cloned() {
            let exists = event_array.iter().any(|v| v.as_str().unwrap() == e);

            if exists == false {
                let mut formatted = toml_edit::Formatted::new(e);
                formatted.decor_mut().set_prefix("\n    ");
                formatted.fmt();
                event_array.push_formatted(toml_edit::Value::String(formatted));
            }
        }

        let doc_str = doc.to_string();

        let mut output = String::with_capacity(doc_str.len());
        let mut parsing_events = false;
        for line in doc_str.lines() {
            let mut line = line.to_string();

            if parsing_events {
                if line.contains(']') {
                    parsing_events = false;
                } else {
                    let mut txt = line.trim();
                    if let Some(stripped) = txt.strip_suffix(',') {
                        txt = stripped
                    }
                    txt = txt.strip_prefix('"').unwrap();
                    txt = txt.strip_suffix('"').unwrap();

                    let has_event = events.iter().any(|v| v == txt);
                    if has_event == false {
                        let non_whitespace = line.chars().position(|v| !v.is_whitespace()).unwrap();
                        line.insert(non_whitespace, ' ');
                        line.insert(non_whitespace, '#');
                    }
                }
            } else if line.starts_with("events = [") {
                parsing_events = true;
            } else if (parent.is_none() && line.starts_with("parent ="))
                || (sprite.is_none() && line.starts_with("sprite ="))
                || (mask_index.is_none() && line.starts_with("mask_index ="))
            {
                line.insert(0, ' ');
                line.insert(0, '#');
            }
            output.push_str(&line);
            output.push('\n');
        }

        output
    };

    if target_folder.exists() == false && std::fs::create_dir_all(target_folder).is_err() {
        println!("{}: couldn't make target folder", "error".bright_red());
    }

    let path = target_folder.join("object_manifest.toml");
    std::fs::write(&path, doc_str).unwrap();

    if view == false {
        println!("opening in editor...close editor window to proceed, or press Q to cancel");
    }

    #[derive(Debug, Clone, Copy)]
    enum Message {
        Closed,
        Canceled,
    }

    let (sndr, rcvr) = std::sync::mpsc::channel::<Message>();

    // make the process thread..
    {
        let mut process_builder = std::process::Command::new("code");
        process_builder.arg(&path);

        if view == false {
            process_builder.arg("--wait");
        }
        let sndr = sndr.clone();

        std::thread::spawn(move || {
            // do it!
            let o = process_builder.output();
            if o.is_err() {
                println!("{}: couldn't spawn child process", "error".bright_red());
                sndr.send(Message::Canceled).unwrap();
            } else {
                sndr.send(Message::Closed).unwrap();
            }
        });
    };

    // clear the last line...
    let term = console::Term::stdout();

    // make the cancel thread..
    {
        let term = term.clone();

        std::thread::spawn(move || loop {
            if term.read_char().map_or(false, |c| c == 'q') {
                sndr.send(Message::Canceled).unwrap();
                break;
            }
        });
    }

    let msg = rcvr.recv().unwrap();
    term.clear_last_lines(1).unwrap();

    match msg {
        Message::Closed => {}
        Message::Canceled => {
            return ExitCode::SUCCESS;
        }
    }

    if view == false {
        let Ok(Ok(request)) =
            fs::read_to_string(&path).map(|v| toml::from_str::<ObjectEditRequest>(&v))
        else {
            println!("{}: couldn't parse object manifest", "error".bright_red());

            return ExitCode::FAILURE;
        };

        // we don't need this to succeed, but it'd be better if it did!
        let _ = std::fs::remove_file(&path);

        // first, handle a rename
        if request.name != name {
            if let Err(e) = yyp_boss.rename_resource::<Object>(&name, request.name.clone()) {
                println!("{}: couldn't rename because {}", "error".bright_red(), e)
            }
        }

        let root_folder = yyp_boss.project_metadata().root_file.clone();

        let ObjectEditRequest {
            name,
            events,
            parent,
            sprite,
            folder,
            visible,
            tags,
            mask_index,
        } = request;

        // okay now we transform the event list...
        let event_result: Result<Vec<EventType>, EventTypeConvertErrors> = events
            .into_iter()
            .map(|event_name| EventType::from_human_readable(&event_name))
            .collect();

        let new_events = match event_result {
            Ok(v) => v,
            Err(e) => {
                println!("{}: failed to parse event_name {}", "error".bright_red(), e);

                return ExitCode::FAILURE;
            }
        };

        let vfs = match folder {
            Some(folder_name) => {
                if folder_name == "root_folder" {
                    root_folder
                } else {
                    let Some(path) = super::find_vfs_path(&yyp_boss, &folder_name) else {
                        println!(
                            "{}: folder `{}` not found.",
                            "error".bright_red(),
                            folder_name
                        );

                        return ExitCode::FAILURE;
                    };

                    path
                }
            }
            None => root_folder,
        };

        let sprite = match sprite {
            Some(sprite) => {
                let Some(sprite) = yyp_boss
                    .yyp()
                    .resources
                    .iter()
                    .find(|v| v.id.name == sprite)
                else {
                    println!(
                        "{}: no sprite named `{}` found",
                        "error".bright_red(),
                        sprite
                    );
                    return ExitCode::FAILURE;
                };

                Some(sprite.id.clone())
            }
            None => None,
        };

        let mask_index = match mask_index {
            Some(mask_index) => {
                let Some(mask_index) = yyp_boss
                    .yyp()
                    .resources
                    .iter()
                    .find(|v| v.id.name == mask_index)
                else {
                    println!(
                        "{}: no sprite for mask_index named `{}` found",
                        "error".bright_red(),
                        mask_index
                    );
                    return ExitCode::FAILURE;
                };

                Some(mask_index.id.clone())
            }
            None => None,
        };

        let parent = match parent {
            Some(parent) => {
                let maybe_parent = yyp_boss.yyp().resources.iter().find_map(|v| {
                    if v.id.name == parent {
                        Some(v.id.clone())
                    } else {
                        None
                    }
                });

                let Some(parent) = maybe_parent else {
                    println!(
                        "{}: no object named `{}` found",
                        "error".bright_red(),
                        parent
                    );
                    return ExitCode::FAILURE;
                };

                Some(parent)
            }
            None => None,
        };

        let obj_data = unsafe { yyp_boss.objects.get_mut(&name).unwrap() };

        obj_data.yy_resource.parent = vfs;
        obj_data.yy_resource.sprite_id = sprite;
        obj_data.yy_resource.sprite_mask_id = mask_index;
        obj_data.yy_resource.parent_object_id = parent;
        obj_data.yy_resource.visible = visible.unwrap_or(true);
        obj_data.yy_resource.tags = tags.unwrap_or_default();

        let assoc = obj_data.associated_data.as_mut().unwrap();
        obj_data.yy_resource.event_list.retain(|value| {
            if new_events.contains(&value.event_type) == false {
                // farewell!
                assoc.remove(&value.event_type);

                false
            } else {
                true
            }
        });

        for event_type in new_events {
            let exists = obj_data
                .yy_resource
                .event_list
                .iter()
                .any(|v| v.event_type == event_type);

            if exists {
                continue;
            }

            obj_data.yy_resource.event_list.push(ObjectEvent {
                event_type,
                ..Default::default()
            });

            assoc.insert(event_type, String::new());
        }

        yyp_boss.objects.mark_for_serialization(&name).unwrap();
        if let Err(e) = yyp_boss.serialize() {
            println!(
                "{}: failed to serialize project because {}",
                "error".bright_red(),
                e
            );

            return ExitCode::FAILURE;
        }

        println!(
            "{}: updated object `{}`",
            "success".bright_green(),
            name.bold()
        );
    }

    ExitCode::SUCCESS
}

fn gm_object_to_object_configuration(gm_object: &Object) -> ObjectEditRequest {
    let mut events: Vec<_> = gm_object.event_list.iter().map(|v| v.event_type).collect();
    events.sort();

    ObjectEditRequest {
        name: gm_object.common_data.name.clone(),
        events: events.into_iter().map(|v| v.to_human_readable()).collect(),
        parent: gm_object.parent_object_id.as_ref().map(|v| v.name.clone()),
        sprite: gm_object.sprite_id.as_ref().map(|v| v.name.clone()),
        mask_index: gm_object.sprite_mask_id.as_ref().map(|v| v.name.clone()),
        folder: gm_object
            .parent
            .path
            .0
            .strip_prefix("folders/")
            .map(|v| v.strip_suffix(".yy").unwrap().to_owned()),
        visible: Some(gm_object.visible),
        tags: Some(gm_object.tags.clone()),
    }
}

fn manifest_maker(edit_request: &ObjectEditRequest, base_doc: &mut toml_edit::Document) {
    let ObjectEditRequest {
        name,
        parent,
        sprite,
        folder,
        visible,
        tags,
        events: _,
        mask_index,
    } = edit_request;

    fn opter<T>(doc: &mut toml_edit::Document, t: &Option<T>, case: &str)
    where
        T: Into<toml_edit::Value> + Clone,
    {
        match t.as_ref() {
            Some(v) => {
                doc[case] = toml_edit::value(v.clone());
            }
            None => {
                doc.remove(case);
            }
        }
    }

    base_doc["name"] = toml_edit::value(name);
    opter(base_doc, parent, "parent");
    opter(base_doc, sprite, "sprite");
    opter(base_doc, mask_index, "spriteMaskId");
    opter(base_doc, folder, "folder");
    opter(base_doc, visible, "visible");

    match tags {
        Some(v) => {
            let arr = toml_edit::Array::from_iter(v);
            base_doc["tags"] = toml_edit::value(arr);
        }
        None => {
            // actually empty array is fine here!
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest() {
        let txt = include_str!("../../assets/object_view_manifest.toml");
        let _: ObjectEditRequest = toml::from_str(txt).unwrap();

        let txt = include_str!("../../assets/object_edit_manifest.toml");
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
            mask_index: Some("spr_player_mask".to_string()),
            visible: Some(true),
            tags: Some(vec!["Dungeon".to_string()]),
        };

        let mut doc = (include_str!("../../assets/object_edit_manifest.toml"))
            .parse()
            .unwrap();
        manifest_maker(&edit_request, &mut doc);
        // set the events
        let arr = toml_edit::Array::from_iter(edit_request.events.iter().cloned());
        doc["events"] = toml_edit::value(arr);

        let obj_edit_request: ObjectEditRequest = toml::from_str(&doc.to_string()).unwrap();

        assert_eq!(obj_edit_request, edit_request);
    }
}
