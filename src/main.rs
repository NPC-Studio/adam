use std::env;

mod gm;
mod input;

const OUTPUT_DIR: &str = "target";

fn main() {
    let user_directory = directories::UserDirs::new()
        .unwrap()
        .home_dir()
        .join("AppData/Roaming/GameMakerStudio2");

    let current_directory = env::current_dir().expect("cannot work in current directory");

    let um_json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&user_directory.join("um.json")).unwrap())
            .unwrap();

    let user_id: usize = um_json
        .get("userID")
        .unwrap()
        .as_str()
        .unwrap()
        .parse()
        .unwrap();

    let user_name = um_json
        .get("username")
        .unwrap()
        .as_str()
        .unwrap()
        .split('@')
        .next()
        .unwrap()
        .to_owned();

    let build_bff = gm::BuildData {
        output_folder: current_directory.join(OUTPUT_DIR),
        output_kind: gm::OutputKind::Vm,
        project_name: String::new(),
        current_directory,
        user_dir: user_directory,
        user_name,
        user_id,
        runtime_location: std::path::Path::new(
            "C:/ProgramData/GameMakerStudio2/Cache/runtimes/runtime-2.3.0.401",
        )
        .to_owned(),
        target_mask: 64,
        application_path: std::path::Path::new(
            "C:/Program Files/GameMaker Studio 2/GameMakerStudio.exe",
        )
        .to_owned(),
    };

    println!("{:#?}", build_bff);
}
