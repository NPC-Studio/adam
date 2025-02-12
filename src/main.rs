#![allow(clippy::bool_comparison)]
#![deny(rust_2018_idioms)]
#![deny(rust_2021_compatibility)]
#![allow(clippy::assigning_clones)]

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
compile_error!("we only support `windows` and `macos` targets!");

use clap::Parser;
use std::{io::BufRead, process::ExitCode};

type AnyResult<T = ()> = color_eyre::eyre::Result<T>;

mod igor;
use igor::{OutputKind, TargetFolders};

mod input;
use input::{ClapOperation, UserConfigOptions};

mod gm_artifacts;
use gm_artifacts::DEFAULT_PLATFORM_DATA;

mod project_editing;

mod runner;
use runner::{PlatformOptions, RunOptions, TaskOptions};

fn main() -> ExitCode {
    color_eyre::install().unwrap();
    let inputs = input::InputOpts::parse();

    // we have a few things that aren't really about building projects,
    // because this app has grown!
    match inputs.subcmd {
        ClapOperation::UserConfig(v) => match v {
            UserConfigOptions::View => {
                let config: input::Manifest = confy::load("adam", None).unwrap();

                println!("{}", toml::to_string_pretty(&config).unwrap());
                return ExitCode::SUCCESS;
            }
            UserConfigOptions::Path => {
                println!(
                    "{}",
                    confy::get_configuration_file_path("adam", None)
                        .unwrap()
                        .display()
                );
                return ExitCode::SUCCESS;
            }

            UserConfigOptions::Edit { name, value } => {
                let value = match name.as_str() {
                    "verbosity" | "ignore_cache" => {
                        let v: usize = match value.parse() {
                            Ok(v) => v,
                            Err(e) => {
                                println!(
                                    "{} invalid value: {:?}",
                                    console::style("error").bright().red(),
                                    e
                                );

                                return ExitCode::FAILURE;
                            }
                        };

                        serde_json::Value::Number(v.into())
                    }
                    "beta" | "no_user_folder" => {
                        let v: bool = match value.parse() {
                            Ok(v) => v,
                            Err(e) => {
                                println!(
                                    "{} invalid value: {:?}",
                                    console::style("error").bright().red(),
                                    e
                                );

                                return ExitCode::FAILURE;
                            }
                        };

                        serde_json::Value::Bool(v)
                    }
                    "test_env_variables" => {
                        serde_json::Value::Array(vec![serde_json::Value::String(value)])
                    }
                    "x64_windows" => {
                        println!(
                            "{}: `x64_windows` is deprecated",
                            console::style("error").bright().red(),
                        );

                        return ExitCode::FAILURE;
                    }
                    _ => serde_json::Value::String(value),
                };

                let mut config: input::Manifest = confy::load("adam", None).unwrap();

                let json_flash = serde_json::json!({ name: value });
                let edit = match serde_json::from_value::<input::Manifest>(json_flash) {
                    Ok(v) => v,
                    Err(e) => {
                        println!(
                            "{}: invalid input: {:?}",
                            console::style("error").bright().red(),
                            e
                        );
                        return ExitCode::FAILURE;
                    }
                };
                edit.apply_on(&mut config);

                confy::store("adam", None, config).unwrap();
                println!(
                    "{}: user configuration has been saved.",
                    console::style("success").green().bright(),
                );
                return ExitCode::SUCCESS;
            }
        },
        ClapOperation::Folder(vfs) => {
            return project_editing::folder_request(vfs);
        }
        ClapOperation::Script(data) => {
            return project_editing::add_script(data);
        }
        ClapOperation::Object(data) => {
            return project_editing::add_object(data);
        }
        ClapOperation::Edit(edit_manifest) => {
            let current_dir = std::env::current_dir().unwrap();
            let target_folder = camino::Utf8PathBuf::from_path_buf(
                current_dir.join(
                    edit_manifest
                        .output_folder
                        .as_deref()
                        .unwrap_or_else(|| camino::Utf8Path::new("target")),
                ),
            )
            .unwrap();

            return project_editing::edit_manifest(
                edit_manifest.asset_name,
                edit_manifest.view,
                &target_folder,
            );
        }
        ClapOperation::Remove { name } => {
            return project_editing::remove(name);
        }
        ClapOperation::Rename {
            current_name,
            new_name,
        } => {
            return project_editing::rename(current_name, new_name);
        }
        ClapOperation::Reserialize => return project_editing::reserialize(),

        _ => {}
    }

    let mut config: input::Manifest = match confy::load("adam", None) {
        Ok(v) => v,
        Err(e) => {
            println!(
                "{}: user-config was invalid ({}). replacing with default...",
                e,
                console::style("warning").green().bright()
            );

            input::Manifest::default()
        }
    };
    let patch_config = input::Manifest::find_manifest(inputs.manifest.as_ref()).unwrap_or_default();

    patch_config.apply_on(&mut config);

    let mut runtime_options = {
        let platform: PlatformOptions = PlatformOptions {
            gms2_application_location: DEFAULT_PLATFORM_DATA.stable_application_path.into(),
            runtime_location: DEFAULT_PLATFORM_DATA.stable_runtime_location.into(),
            visual_studio_path: Default::default(),
            user_license_folder: Default::default(),
            compiler_cache: DEFAULT_PLATFORM_DATA.stable_cached_data.clone(),
        };
        let task = TaskOptions::default();

        RunOptions {
            task,
            platform,
            no_compile: None,
        }
    };
    let mut check_options = None;
    config.write_to_options(&mut runtime_options, &mut check_options);

    let (mut options, operation) =
        match input::parse_inputs(inputs.subcmd, runtime_options, &mut check_options) {
            Ok(v) => v,
            Err(e) => {
                println!(
                    "{} parsing inputs: {}",
                    console::style("error").bright().red(),
                    e
                );
                return ExitCode::FAILURE;
            }
        };

    if options.task.no_build_script {
        check_options = None;
    }

    if let Err(e) = options.platform.canonicalize() {
        println!(
            "{}: invalid {} path (file does not exist). Is everything installed correctly?",
            console::style("adam error").bright().red(),
            console::style(e).bold()
        );

        return ExitCode::FAILURE;
    }

    if options.task.yyc {
        if let Err(_e) = options.platform.canonicalize_yyc() {
            println!(
                "{}: invalid yyc path `{}` (file does not exist). Is everything installed correctly?",
                console::style("adam error").bright().red(),
                console::style(options.platform.visual_studio_path).bold()
            );

            return ExitCode::FAILURE;
        }
    }

    let application_data = match igor::ApplicationData::new() {
        Ok(v) => v,
        Err(e) => {
            println!(
                "{}: {}",
                console::style("adam error").bright().red(),
                console::style(e).bold()
            );

            return ExitCode::FAILURE;
        }
    };

    // tell the runner where we are
    std::env::set_var(
        "ADAM_PROJECT_PATH",
        application_data.current_directory.as_os_str(),
    );

    // handle a clean, extract the build_data
    let run_kind = match operation {
        input::Operation::Run(inner) => {
            if let Some(check_options) = check_options {
                if runner::run_check(&options.task, check_options).is_err() {
                    return ExitCode::FAILURE;
                }
            }
            inner
        }
        input::Operation::Check => {
            let check_options = match check_options {
                Some(v) => v,
                None => {
                    println!(
                        "{}: no script given to run via CLI or config",
                        console::style("error").bright().red()
                    );

                    return ExitCode::FAILURE;
                }
            };
            if runner::run_check(&options.task, check_options).is_ok() {
                return ExitCode::SUCCESS;
            } else {
                return ExitCode::FAILURE;
            }
        }
        input::Operation::Clean => {
            // clean up the output folder...
            if let Err(e) = std::fs::remove_dir_all(
                application_data
                    .current_directory
                    .join(&options.task.output_folder),
            ) {
                println!("{} on clean: {}", console::style("error").bright().red(), e);
                return ExitCode::FAILURE;
            }
            return ExitCode::SUCCESS;
        }
    };

    // fire any specific behavior to this run kind
    if let input::RunKind::Test(value) = &run_kind {
        for var in options.task.test_env_variables.iter() {
            std::env::set_var(var, "1");
        }

        // we set this fella every time too
        std::env::set_var("ADAM_TEST", value);
    }

    // crazy branch right here: if we're a no_compile run op, then we get outta there!
    if let Some(no_compile) = &options.no_compile {
        // if we're on macOS, yell and say we can't do that!

        let mut igor = std::process::Command::new(format!(
            "{}/{}/x64/Runner.exe  ",
            &options.platform.runtime_location,
            gm_artifacts::PLATFORM_KIND,
        ));

        let data_win_path = if no_compile.as_str().is_empty() {
            format!(
                "./{}/{}/output/data.win",
                options.task.output_folder,
                if options.task.yyc { "yyc" } else { "vm" },
            )
        } else {
            no_compile.to_string()
        };

        igor.arg("-game")
            .arg(data_win_path)
            .stdout(std::process::Stdio::piped());

        if options.task.verbosity > 0 {
            println!("{:?}", igor);
        }

        let mut child = igor.spawn().unwrap();
        let reader = std::io::BufReader::new(child.stdout.as_mut().unwrap()).lines();
        for line in reader.map_while(Result::ok) {
            println!("{}", line.trim());
        }

        let success = match child.wait() {
            Ok(e) => e.success(),
            Err(_) => false,
        };

        let (style_value, exit_code) = if success {
            (console::style("ok").green().bright(), ExitCode::SUCCESS)
        } else {
            (console::style("FAILED").red().bright(), ExitCode::FAILURE)
        };
        println!("adam test result: {}", style_value);

        return exit_code;
    }

    // check if we have a valid yyc bat
    if options.task.yyc {
        if cfg!(not(target_os = "windows")) {
            println!(
                "{}: {}\nPlease log a feature request at https://github.com/NPC-Studio/adam/issues",
                console::style("adam error",).bright().red(),
                console::style("adam does not support macOS YYC compilation, yet.").bold(),
            );
            return ExitCode::FAILURE;
        }

        if options.platform.visual_studio_path.exists() == false {
            println!(
                "{}: {}.\n\
            Supplied path in preferences was \"{}\" but it did not exist.\n\
            To use yyc, we must have a visual studio .bat file.\n\
        Please specify a path in the Gms2 IDE. \n\
        For more information, see \
        https://help.yoyogames.com/hc/en-us/articles/227860547-GMS2-Required-SDKs",
                console::style("error").bright().red(),
                console::style("no valid path to visual studio .bat build file").bold(),
                options.platform.visual_studio_path,
            );

            return ExitCode::FAILURE;
        }
    }

    let output_kind = if options.task.yyc {
        igor::OutputKind::Yyc
    } else {
        igor::OutputKind::Vm
    };

    let Some(project_filename) = application_data.project_name else {
        println!(
            "{}: {}",
            console::style("adam error").bright().red(),
            console::style("could not find a .yyp in the current directory!").bold()
        );

        return ExitCode::FAILURE;
    };

    let folders = match TargetFolders::new(
        &application_data.current_directory,
        &options.task.output_folder,
        output_kind,
        &project_filename,
    ) {
        Ok(v) => v,
        Err(e) => {
            println!(
                "{} on creating output folders: {}",
                console::style("error").bright().red(),
                e
            );
            return ExitCode::FAILURE;
        }
    };

    let build_data = igor::BuildData {
        folders,
        output_kind,
        project_filename,
        project_directory: application_data.current_directory,
        // user_dir: options.platform.user_data.clone(),
        user_dir: Default::default(),
        license_folder: options.platform.user_license_folder.clone(),
        runtime_location: options.platform.runtime_location.clone(),
        target_mask: DEFAULT_PLATFORM_DATA.target_mask,
        application_path: options.platform.gms2_application_location.clone(),
        config: options.task.config.clone(),
    };

    let gm_build = gm_artifacts::GmBuild::new(&build_data);
    let macros = gm_artifacts::GmMacros::new(&build_data);

    let visual_studio_path = options.platform.visual_studio_path.clone();

    // clear the temp files...
    if let Err(e) = build_data.folders.clear_tmp() {
        println!(
            "{} creating temp folder: {}",
            console::style("error").bright().red(),
            e
        );
        return ExitCode::FAILURE;
    }

    let build_location = build_data.folders.cache.join("build.bff");

    // write in the build.bff
    std::fs::write(
        &build_location,
        serde_json::to_string_pretty(&gm_build).unwrap(),
    )
    .unwrap();

    // write in the preferences
    let preferences = if build_data.output_kind == OutputKind::Yyc {
        gm_artifacts::GmPreferences::new(visual_studio_path.as_std_path().to_owned())
    } else {
        gm_artifacts::GmPreferences::default()
    };
    std::fs::write(
        &gm_build.preferences,
        serde_json::to_string_pretty(&preferences).unwrap(),
    )
    .unwrap();

    // write in the targetoptions
    std::fs::write(
        &gm_build.target_options,
        serde_json::to_string_pretty(&gm_artifacts::GmTargetOptions {
            runtime: build_data.output_kind,
        })
        .unwrap(),
    )
    .unwrap();

    // write in the steamoptions -- we just use defaults here...
    std::fs::write(
        &gm_build.steam_options,
        serde_json::to_string_pretty(&gm_artifacts::GmSteamOptions::default()).unwrap(),
    )
    .unwrap();

    // and we write the macros finally
    std::fs::write(
        &gm_build.macros,
        serde_json::to_string_pretty(&macros).unwrap(),
    )
    .unwrap();

    // on macos, we don't have a good way to debug out, so this is the best we got.
    #[cfg(not(target_os = "windows"))]
    {
        use interprocess::local_socket::LocalSocketListener;

        let socket_name = gm_build.temp_folder.join("ipc_log.log");
        let socket_name = socket_name.to_str().unwrap();
        std::env::set_var("ADAM_IPC_SOCKET", socket_name);

        if let Ok(listener) = LocalSocketListener::bind(socket_name) {
            std::thread::Builder::new()
                .name("adam-ipc".into())
                .spawn(move || {
                    for mut stream in listener.incoming().filter_map(|v| v.ok()) {
                        loop {
                            use std::io::Read;

                            let mut size: [u8; 8] = [0; 8];
                            let Ok(_) = stream.read_exact(&mut size) else {
                                break;
                            };
                            let size = u64::from_ne_bytes(size);
                            if size == 0 {
                                continue;
                            }

                            let mut bytes = vec![0; size as usize];
                            let Ok(_) = stream.read_exact(&mut bytes) else {
                                break;
                            };

                            let str = std::str::from_utf8(&bytes).unwrap();
                            print!("{}", str);
                        }
                    }
                })
                .unwrap();
        }
    }

    let success = runner::run_command(&build_location, macros, options, &run_kind);
    if success {
        if run_kind.is_test() {
            println!(
                "adam test result: {}",
                console::style("ok").green().bright()
            );
        } else {
            println!("adam {}", console::style("complete").green().bright());
        }

        ExitCode::SUCCESS
    } else {
        if run_kind.is_test() {
            println!(
                "adam test result: {}",
                console::style("FAILED").red().bright()
            );
        } else {
            println!("adam {}", console::style("FAILED").red().bright());
        }

        ExitCode::FAILURE
    }
}
