use camino::Utf8PathBuf;
use clap::Parser;

use crate::{DEFAULT_PLATFORM_DATA, RunOptions};

/// A CLI intended for use by humans and machines to build GameMakerStudio 2 projects.
#[derive(Parser, Debug)]
#[clap(version, author)]
pub struct InputOpts {
    #[clap(subcommand)]
    pub subcmd: ClapOperation,

    /// The path to a non-standard named manifest file. Possible names are .adam, .adam.json, and adam.toml
    #[clap(short, long)]
    pub manifest: Option<std::path::PathBuf>,
}

#[derive(Parser, Debug)]
pub enum ClapOperation {
    /// Builds a project *without* running it.
    #[clap(alias = "b")]
    #[cfg(target_os = "windows")]
    Build(BuildOptions),

    /// Compiles, if necessary, and then runs a project.
    #[clap(alias = "r")]
    Run(BuildOptions),

    /// Creates a release executable, running `clean` first.
    Release(BuildOptions),

    /// Runs some presumably shorter "check" script. These scripts will also have the following environment variables set:
    ///
    /// `ADAM_CHECK`: 1
    ///
    /// `ADAM_YYC`:  0 or 1
    ///
    /// `ADAM_CONFIG`:  String
    ///
    /// `ADAM_VERBOSITY`:  Number
    ///
    /// `ADAM_OUTPUT_FOLDER`:  String
    ///
    /// `ADAM_IGNORE_CACHE`:  Number
    #[clap(alias = "c")]
    Check {
        /// This is the shell script which we will run.
        ///
        /// This path is relative to the current working directory.
        path_to_run: Option<Utf8PathBuf>,

        #[clap(flatten)]
        build_options: BuildOptions,
    },

    /// Runs the project, enabling any `test_env_variables` and searches for the `test_success_code`, set in the config.
    #[clap(alias = "t")]
    Test {
        #[clap(flatten)]
        build_options: BuildOptions,

        /// We set `ADAM_TEST` to these values, generally after a `--`, such as `adam test -- foo --bar=baz`.
        #[arg(trailing_var_arg = true)]
        adam_test: Vec<String>,
    },

    /// Cleans a project target directory.
    Clean(BuildOptions),

    /// Reserializes all available files, and deletes all unknown files.
    /// Warning: use caution!
    Reserialize,

    /// Views the asset's manifest as a toml file.
    Edit(EditManifest),

    /// Adds or edits a script to the project.
    #[clap(alias = "s")]
    Script(ScriptEditRequest),

    /// Adds or Edits an Object.
    #[clap(alias = "o", alias = "objects")]
    Object(ObjectEditRequest),

    /// Adds or edits a shader to the project.
    Shader(ShaderEditRequest),

    /// Removes an asset of the given name from the project.
    #[clap(visible_alias = "rm")]
    Remove {
        /// The name of the asset to remove, like `spr_player` or `obj_bullet`
        name: String,
    },

    /// Renames an asset from one name to another name.
    Rename {
        /// The current name, such as `obj_player`
        current_name: String,

        /// The new name, such as `objPlayer`
        new_name: String,
    },

    /// Virtual File System commands for a project.
    #[clap(subcommand)]
    Folder(FolderRequest),

    /// Edits the user's personal configuration file
    #[clap(subcommand)]
    UserConfig(UserConfigOptions),
}

#[derive(Debug, Parser)]
pub struct ScriptEditRequest {
    /// The name of the script, such as `FileUtilities`. Do not include `gml` in it.
    pub name: String,

    /// Which folder to place the script in the VFS. If not provided, placed at the root.
    #[clap(short, long)]
    pub folder: Option<String>,
}

#[derive(Debug, Parser)]
pub struct ShaderEditRequest {
    /// The name of the shader, such as `shd_outline`. Do not include any file extension.
    pub name: String,

    /// Which folder to place the script in the VFS.
    ///
    /// If the provided folder, including the default value, does not exist, then it will be placed
    /// at the root of the project.
    #[clap(short, long, default_value = "Shaders")]
    pub folder: String,

    /// What shader type to create.
    #[clap(short, long, default_value_t, value_enum)]
    pub shader_type: ShaderEditShaderType,
}

#[derive(Debug, clap::ValueEnum, PartialEq, Eq, Default, Clone)]
pub enum ShaderEditShaderType {
    #[default]
    GlslEs,
    Glsl,
    Hlsl,
}

#[derive(Debug, Parser, serde::Serialize, serde::Deserialize, Default, PartialEq, Eq, Clone)]
pub struct ObjectEditRequest {
    /// The name of the object, such as `obj_player`.
    pub name: String,

    /// Events to put in the object. Events can be like `create`, `step_0`, or `other_4`. If no number
    /// is provided, `_0` is assumed where appropriate.
    pub events: Vec<String>,

    /// The parent object to set up some inheritence nonsense, such as `par_npc` or `obj_laser`
    #[clap(short, long)]
    pub parent: Option<String>,

    /// The sprite name to use for this object, such as `spr_player_running`
    #[clap(short, long)]
    pub sprite: Option<String>,

    /// The override sprite mask to set.
    #[clap(short, long)]
    pub mask_index: Option<String>,

    /// Where to place the script within the virtual file system. If not provided, placed at the root.
    #[clap(long)]
    pub folder: Option<String>,

    /// Marks visibility on the object.
    #[clap(long)]
    pub visible: Option<bool>,

    /// Sets the tags on an object. This replaces all the tags on the object if it already exists.
    #[clap(long)]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Parser)]
pub struct EditManifest {
    /// The name of the asset to view. Only objects are currently supported.
    pub asset_name: String,

    /// Views the asset only. This will show the manifest and exit.
    #[clap(short, long)]
    pub view: bool,

    /// The relative path to the output folder. Defaults to `target`.
    #[clap(short, long)]
    pub output_folder: Option<Utf8PathBuf>,
}

#[derive(Parser, Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum UserConfigOptions {
    /// Prints out the User Configuration file. If one does not exist, it is created.
    View,

    /// Prints out the full path to the user configuration file. If one does not exist, it is created.
    Path,

    /// Saves a given key value pair in the user config.
    Edit {
        /// The name of the property.
        name: String,
        /// The value of the property.
        value: String,
    },
}

/// The kinds of things which can be added to a project.
#[derive(Parser, Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum FolderRequest {
    /// View a folder and its contents with the given path. Without a path, will
    /// show the root directory.
    View {
        /// A root directory to look at. If not passed, we use the project root.
        folder: Option<String>,
    },

    /// Moves a file to a new directory.
    #[clap(visible_alias = "mv")]
    Move {
        /// This is the file, such as `obj_player` or `spr_bullet`.
        target: String,
        /// This is the new directory to move it to, such as `Scripts/Player/Data`.
        /// Note: if the directory does not exist, it will be created.
        new_directory: String,
    },

    /// Tries to find the VFS path of the given asset.
    #[clap(alias = "w")]
    Where {
        /// The asset name to find.
        asset_name: String,
    },
}

#[derive(Parser, Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub struct SavePathOptions {
    pub path: Utf8PathBuf,
}

#[derive(clap::Args, Debug, PartialEq, Eq, Clone, Default)]
pub struct BuildOptions {
    /// When set, we will not compile the game first and instead just immediately re_run the game based
    /// on what was previously compiled. This can easily lead to errors.
    #[clap(long)]
    no_compile: bool,

    /// When set, when `no_compile` is given, we will use this location for a data.win file. This should be
    /// a direct path ending in `data.win`.
    #[clap(long)]
    no_compile_data_win_location: Option<Utf8PathBuf>,

    /// When set, no build scripts will be invoked which may be in the manifest.
    #[clap(long)]
    no_build_script: bool,

    /// Uses the YYC instead of the default VM. If this is the case, then we'll need to check
    /// your Visual Studio path on Windows.
    #[clap(long, short)]
    pub yyc: bool,

    /// Option to switch to using the Gms2 Beta. By default, this will use the
    /// `C:/Program Files/GameMaker Studio 2 Beta/GameMakerStudio-Beta.exe` filepath,
    /// but can be overriden with `gms2_install_location` for beta Steam builds.
    #[clap(long)]
    pub beta: bool,

    /// Whether or not to use the x64 variant on windows.
    ///
    /// On non-Windows platforms, this option is meaningless. We do a best effort to detect x64 usage by reading
    /// your options.yy, but we don't currently parse configs deeply, which means that a special config set up
    /// to use x64 won't be discovered. For such a circumstance, use this flag to build correctly.
    ///
    /// In general, it's easiest if you don't use override x64 with certain configs in Gms2.
    #[clap(long, short)]
    pub x64_windows: bool,

    /// If this option is set, then we will not read your `~/.config/GameMakerStudio2` or `%APPDATA%/GameMakerStudio2` folders
    /// at all. If you pass this, then you MUST pass in a `user-license-folder` and (on Windows) a `visual-studio-path`. Otherwise,
    /// adam will exit out with an error.
    #[clap(long)]
    pub no_user_folder: bool,

    /// Specifies a configuration. If not passed, we use `Default` for our Config.
    #[clap(short, long)]
    pub config: Option<String>,

    /// Specifies the target Yyp to build, if there are multiple.
    #[clap(long)]
    pub yyp: Option<String>,

    /// Verbosity level. Can use multiple times, like '-vv'. >0 disables pretty compiles, >1 enables igor verbosity, >2 enables gmac verbosity
    #[clap(short, long)]
    #[arg(action(clap::ArgAction::Count))]
    pub verbosity: u8,

    /// The relative path to the output folder. Defaults to `target`.
    #[clap(short, long)]
    pub output_folder: Option<Utf8PathBuf>,

    /// Ignore cache. Can use multiples times, like `-ii`. >0 disables quick recompiles, >1 disables all caching.
    #[clap(short, long)]
    #[arg(action(clap::ArgAction::Count))]
    pub ignore_cache: u8,

    /// The path to your Gms2 installation. Defaults to C drive on Windows and Applications on macOS. If you use Steam, you will need to pass in that fullpath to the .exe, or the .app on macOS.
    #[clap(long)]
    pub gms2_install_location: Option<Utf8PathBuf>,

    /// If the non-current runtime is desired, it can be set here. We default right now to `2.3.1.409` on stable and beta.
    #[clap(short, long)]
    pub runtime: Option<String>,

    /// This sets a complete path to the runtime location.
    #[clap(long)]
    pub runtime_location_override: Option<Utf8PathBuf>,

    /// Use this visual studio path, instead of the visual studio path within the `user_folder`
    /// at `~/.config`. This is only relevant on Windows.
    ///
    /// This should be a path to the `.bat` file, such as:
    ///
    /// ```zsh
    /// C:/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Auxiliary/Build/vcvars32.bat
    /// ```
    ///
    /// For more info on this path, see https://help.yoyogames.com/hc/en-us/articles/235186048-Setting-Up-For-Windows
    ///
    /// If this field and `user_license_folder` are both set, then we will not look in your
    /// `user_folder` at all. To ensure we don't do that, pass `-no-user-folder`.
    #[clap(long)]
    pub visual_studio_path: Option<Utf8PathBuf>,

    /// Use this folder for the user_license, instead of the path within the `user_folder`
    /// at `~/.config`.
    ///
    /// If this field and `visual_studio_path` are both set, then we will not look in your
    /// `user_folder` at all.
    #[clap(long)]
    pub user_license_folder: Option<Utf8PathBuf>,

    /// If true, will try to find the PID of a runner game and force it to close.
    #[clap(long)]
    pub close_on_sig_kill: bool,
}

impl BuildOptions {
    pub fn write_to_options(self, run_options: &mut RunOptions) {
        // don't compile it if we don't wanna!
        if self.no_compile {
            run_options.no_compile = Some(self.no_compile_data_win_location.unwrap_or_default());
        }

        if self.no_build_script {
            run_options.task.no_build_script = true;
        }

        if let Some(cfg) = self.config {
            run_options.task.config = cfg;
        }
        if let Some(of) = self.output_folder {
            run_options.task.output_folder = of;
        }
        if let Some(gms2) = self.gms2_install_location {
            run_options.platform.gms2_application_location = gms2;
        }
        if let Some(runtime) = self.runtime {
            let path = run_options
                .platform
                .runtime_location
                .parent()
                .unwrap()
                .join(format!("runtime-{}", runtime));
            run_options.platform.runtime_location = path;
        }
        if let Some(runtime_location_override) = self.runtime_location_override {
            run_options.platform.runtime_location = runtime_location_override;
        }
        if let Some(visual_studio_path) = self.visual_studio_path {
            run_options.platform.visual_studio_path = visual_studio_path;
        }
        if let Some(user_license_folder) = self.user_license_folder {
            run_options.platform.user_license_folder = user_license_folder;
        }

        // Macos never has a visual studio path!
        // good stuff, eh?
        #[cfg(target_os = "macos")]
        {
            run_options.platform.visual_studio_path = Default::default();
        }

        if self.no_user_folder {
            run_options.task.no_user_folder = true;
        }

        if self.beta {
            run_options.platform.gms2_application_location =
                DEFAULT_PLATFORM_DATA.beta_application_path.into();

            run_options.platform.runtime_location =
                DEFAULT_PLATFORM_DATA.beta_runtime_location.into();

            run_options.platform.compiler_cache = DEFAULT_PLATFORM_DATA.beta_cached_data.clone();
        }
        if self.verbosity != 0 {
            run_options.task.verbosity = self.verbosity;
        }

        // if we say to use the yyc, we use the yyc
        if self.yyc {
            run_options.task.yyc = true;
        } else {
            // we just set the visual studio path here..
            run_options.platform.visual_studio_path = Default::default();
        }

        if self.ignore_cache != 0 {
            run_options.task.ignore_cache = self.ignore_cache;
        }

        if self.close_on_sig_kill {
            run_options.task.close_on_sig_kill = self.close_on_sig_kill;
        }
    }
}
