use std::process::ExitCode;

use yy_boss::YypBoss;
use yy_typings::{CommonData, Script};

use crate::input::ScriptEditRequest;

pub fn add_script(script: ScriptEditRequest) -> ExitCode {
    let Some(mut yyp_boss) = super::create_yyp_boss(YypBoss::without_resources) else {
        return ExitCode::FAILURE;
    };
    yyp_boss
        .quick_name()
        .expect("bad yyp entry -- couldn't add.");

    let Some(parent) = super::maybe_find_vfs_path(&yyp_boss, script.vfs) else {
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
