use crate::igor::BuildData;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

macro_rules! path {
    ($($arg:tt)*) => {{
        let res = std::path::PathBuf::from(std::fmt::format(format_args!($($arg)*)));
        res
    }}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GmMacros {
    #[serde(rename = "Desktop")]
    desktop: PathBuf,
    #[serde(rename = "Programs")]
    programs: PathBuf,
    #[serde(rename = "MyDocuments")]
    my_documents: PathBuf,
    #[serde(rename = "Favorites")]
    favorites: PathBuf,
    #[serde(rename = "Startup")]
    startup: PathBuf,
    #[serde(rename = "Recent")]
    recent: PathBuf,
    #[serde(rename = "SendTo")]
    send_to: PathBuf,
    #[serde(rename = "StartMenu")]
    start_menu: PathBuf,
    #[serde(rename = "MyMusic")]
    my_music: PathBuf,
    #[serde(rename = "MyVideos")]
    my_videos: PathBuf,
    #[serde(rename = "DesktopDirectory")]
    desktop_directory: PathBuf,
    #[serde(rename = "MyComputer")]
    my_computer: PathBuf,
    #[serde(rename = "NetworkShortcuts")]
    network_shortcuts: PathBuf,
    #[serde(rename = "Fonts")]
    fonts: PathBuf,
    #[serde(rename = "Templates")]
    templates: PathBuf,
    #[serde(rename = "CommonStartMenu")]
    common_startup_menu: PathBuf,
    #[serde(rename = "CommonPrograms")]
    common_programs: PathBuf,
    #[serde(rename = "CommonStartup")]
    common_startup: PathBuf,
    #[serde(rename = "CommonDesktopDirectory")]
    common_desktop_directory: PathBuf,
    #[serde(rename = "ApplicationData")]
    application_data: PathBuf,
    #[serde(rename = "PrinterShortcuts")]
    printer_shortcuts: PathBuf,
    #[serde(rename = "LocalApplicationData")]
    local_application_data: PathBuf,
    #[serde(rename = "InternetCache")]
    internet_cache: PathBuf,
    #[serde(rename = "Cookies")]
    cookies: PathBuf,
    #[serde(rename = "History")]
    history: PathBuf,
    #[serde(rename = "CommonApplicationData")]
    common_application_data: PathBuf,
    #[serde(rename = "Windows")]
    windows: PathBuf,
    #[serde(rename = "System")]
    system: PathBuf,
    #[serde(rename = "ProgramFiles")]
    program_files: PathBuf,
    #[serde(rename = "MyPictures")]
    my_pictures: PathBuf,
    #[serde(rename = "UserProfile")]
    user_profile: PathBuf,
    #[serde(rename = "SystemX86")]
    system_x86: PathBuf,
    #[serde(rename = "ProgramFilesX86")]
    program_files_x86: PathBuf,
    #[serde(rename = "CommonProgramFiles")]
    common_program_files: PathBuf,
    #[serde(rename = "CommonProgramFilesX86")]
    common_program_files_x86: PathBuf,
    #[serde(rename = "CommonTemplates")]
    common_templates: PathBuf,
    #[serde(rename = "CommonDocuments")]
    common_documents: PathBuf,
    #[serde(rename = "CommonAdminTools")]
    common_admin_tools: PathBuf,
    #[serde(rename = "AdminTools")]
    admin_tools: PathBuf,
    #[serde(rename = "CommonMusic")]
    common_music: PathBuf,
    #[serde(rename = "CommonPictures")]
    common_pictures: PathBuf,
    #[serde(rename = "CommonVideos")]
    common_videos: PathBuf,
    #[serde(rename = "Resources")]
    resources: PathBuf,
    #[serde(rename = "LocalizedResources")]
    localized_resources: PathBuf,
    #[serde(rename = "CommonOemLinks")]
    common_oem_links: PathBuf,
    #[serde(rename = "CDBurning")]
    cd_burning: PathBuf,
    #[serde(rename = "TempPath")]
    temp_path: PathBuf,
    exe_path: PathBuf,
    program_dir_name: String,
    program_name: String,
    program_name_pretty: String,
    #[serde(rename = "runtimeURI")]
    runtime_uri: String,
    #[serde(rename = "updateURI")]
    update_uri: String,
    #[serde(rename = "releaseNotesURI")]
    release_notes_uri: String,
    #[serde(rename = "runtimeReleaseNotesURI")]
    runtime_release_notes_uri: String,
    #[serde(rename = "runtimeBaseLocation")]
    runtime_base_location: PathBuf,
    #[serde(rename = "runtimeLocation")]
    runtime_location: PathBuf,
    base_options_dir: PathBuf,
    asset_compiler_path: PathBuf,
    pub igor_path: PathBuf,
    lib_compatibility_path: PathBuf,
    pub runner_path: PathBuf,
    html5_runner_path: PathBuf,
    webserver_path: PathBuf,
    licenses_path: PathBuf,
    java_exe_path: PathBuf,
    adb_exe_path: PathBuf,
    keytool_exe_path: PathBuf,
    openssl_exe_path: PathBuf,
    skin_path: PathBuf,
    user_skin_path: PathBuf,
    user_override_directory: PathBuf,
    default_skin: PathBuf,
    current_skin: PathBuf,
    system_directory: PathBuf,
    system_cache_directory: PathBuf,
    local_directory: PathBuf,
    local_cache_directory: PathBuf,
    temp_directory: PathBuf,
    asset_compiler_cache_directory: PathBuf,
    ide_cache_directory: PathBuf,
    my_projects_directory: PathBuf,
    base_project: PathBuf,
    default_font: String,
    default_style: String,
    default_font_size: String,
    #[serde(rename = "loginURI")]
    login_uri: String,
    #[serde(rename = "accountsURI")]
    accounts_uri: String,
    #[serde(rename = "marketplaceURI")]
    marketplace_uri: String,
    #[serde(rename = "marketplaceAPIURI")]
    marketplace_api_uri: String,
    #[serde(rename = "carouselSlidesURI")]
    carousel_slides_uri: String,
    user_directory: PathBuf,
    user_cache_directory: PathBuf,
    pub project_full_filename: PathBuf,
    pub project_dir: PathBuf,
    pub project_name: String,
    project_cache_directory_name: String,
    options_dir: PathBuf,
}

impl GmMacros {
    #[cfg(target_os = "windows")]
    pub fn new(build_data: &BuildData) -> Self {
        Self {
            favorites: build_data.user_dir.join("Favorites"),
            fonts: path!("C:/Windows/Fonts"),
            templates: path!("C:/Users/jjspi/AppData/Roaming/Microsoft/Windows/Templates"),
            application_data: build_data.user_dir.join("AppData/Roaming"),
            local_application_data: build_data.user_dir.join("AppData/Local"),
            internet_cache: build_data
                .user_dir
                .join("AppData/Local/Microsoft/Windows/INetCache"),
            common_application_data: path!("C:/ProgramData"),
            windows: path!("C:/Windows"),
            system: path!("C:/Windows/system32"),
            program_files: path!("C:/Program Files"),
            system_x86: path!("C:/Windows/SysWOW64"),
            program_files_x86: path!("C:/Program Files (x86)"),
            common_program_files: path!("C:/Program Files/Common Files"),
            common_program_files_x86: path!("C:/Program Files (x86)/Common Files"),
            common_templates: path!("C:/ProgramData/Microsoft/Windows/Templates"),
            temp_path: build_data.user_dir.join("AppData/Local"),
            update_uri: "https://gms.yoyogames.com/update-win.rss".to_owned(),
            java_exe_path: path!("bin/java.exe"),
            adb_exe_path: path!("platform-tools/adb.exe"),
            keytool_exe_path: path!("bin/keytool.exe"),
            openssl_exe_path: path!("bin/openssl.exe"),
            user_skin_path: path!("C:/ProgramData/Skins"),
            user_override_directory: path!("C:/ProgramData/User"),
            current_skin: path!("C:/ProgramData/Skins/Dracula"),
            system_directory: path!("C:/ProgramData/GameMakerStudio2"),
            system_cache_directory: path!("C:/ProgramData/GameMakerStudio2/Cache"),
            local_directory: build_data.user_dir.join("AppData/Roaming/GameMakerStudio2"),
            local_cache_directory: build_data
                .user_dir
                .join("AppData/Roaming/GameMakerStudio2/Cache"),
            ide_cache_directory: build_data
                .user_dir
                .join("AppData/Roaming/GameMakerStudio2/Cache/GMS2IDE"),

            user_directory: build_data.user_dir.join(format!(
                "AppData/Roaming/GameMakerStudio2/{}",
                build_data.user_string
            )),
            user_cache_directory: build_data.user_dir.join(format!(
                "AppData/Roaming/GameMakerStudio2/{}/Cache",
                build_data.user_string
            )),

            ..Self::create_internal(build_data)
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn new(build_data: &BuildData) -> Self {
        let application_data = build_data.user_dir.join(".config");
        let common_app_data = path!("/User/Shared");
        let system_directory = common_app_data.join("GameMakerStudio2");

        Self {
            favorites: build_data.user_dir.join("Library/Favorites"),
            fonts: build_data.user_dir.join("Library/Fonts"),
            templates: build_data.user_dir.join("Templates"),
            application_data: application_data.clone(),
            local_application_data: build_data.user_dir.join(".local/share"),
            internet_cache: build_data.user_dir.join("Library/Caches"),
            common_application_data: common_app_data,
            program_files: path!("/Applications"),
            common_templates: path!("/usr/share/templates"),
            temp_path: build_data
                .user_dir
                .join("/var/folders/v_/r1l809l94_vbd75s98fbd6rr0000gn"),
            update_uri: "https://gms.yoyogames.com/update-mac.rss".to_owned(),
            java_exe_path: path!("bin/java"),
            adb_exe_path: path!("platform-tools/adb"),
            keytool_exe_path: path!("bin/keytool"),
            openssl_exe_path: path!("bin/openssl"),
            user_skin_path: system_directory.join("Skins"),
            user_override_directory: system_directory.join("User"),
            system_directory: system_directory.clone(),
            system_cache_directory: system_directory.join("Cache"),
            local_directory: application_data.join("GameMakerStudio2"),
            local_cache_directory: application_data.join("GameMakerStudio2/Cache"),
            ide_cache_directory: application_data.join("GameMakerStudio2/Cache/GMS2IDE"),

            user_directory: build_data.user_dir.join(format!(
                "AppData/Roaming/GameMakerStudio2/{}",
                build_data.user_string
            )),
            user_cache_directory: build_data.user_dir.join(format!(
                "AppData/Roaming/GameMakerStudio2/{}/Cache",
                build_data.user_string
            )),

            ..Self::create_internal(build_data)
        }
    }
    fn create_internal(build_data: &BuildData) -> Self {
        let BuildData {
            application_path: app_path,
            user_dir,
            runtime_location: runtime,
            project_directory,
            output_folder: out,
            output_kind,
            project_name,
            target_mask: _,
            user_string: _,
            config: _,
            target_file: _,
        } = build_data;

        let build_dir = out.join(output_kind.to_string());

        Self {
            desktop: user_dir.join("Desktop"),
            my_documents: user_dir.join("Documents"),
            my_music: user_dir.join("Music"),
            my_videos: user_dir.join("Videos"),
            desktop_directory: user_dir.join("Desktop"),
            my_pictures: user_dir.join("Pictures"),
            user_profile: user_dir.clone(),

            exe_path: app_path.clone(),
            runtime_location: runtime.clone(),
            base_options_dir: runtime.join("BaseProject/options"),
            asset_compiler_path: runtime.join("bin/GMAssetCompiler.exe"),
            igor_path: runtime.join("bin/Igor.exe"),
            lib_compatibility_path: runtime.join("lib/compatibility.zip"),
            runner_path: runtime.join("windows/Runner.exe"),
            html5_runner_path: runtime.join("html5/scripts.html5.zip"),
            webserver_path: runtime.join("bin/GMWebServer.exe"),
            licenses_path: app_path.join("Licenses"),
            skin_path: app_path.join("GUI/Skins"),
            default_skin: app_path.join("GUI/Skins/Dark"),

            local_cache_directory: build_dir.clone(),
            temp_directory: build_dir.clone(),
            asset_compiler_cache_directory: build_dir.clone(),
            my_projects_directory: build_dir.join("GameMakerStudio2"),
            base_project: runtime.join("BaseProject/BaseProject.yyp"),
            project_full_filename: project_directory.join(project_name).with_extension("yyp"),
            project_dir: project_directory.clone(),
            project_name: project_name.to_owned(),
            project_cache_directory_name: "cache".to_owned(),
            options_dir: project_directory.join("options"),

            ..Self::default()
        }
    }
}

impl Default for GmMacros {
    fn default() -> Self {
        Self {
            desktop: PathBuf::new(),
            programs: path!(""),
            my_documents: PathBuf::new(),
            favorites: PathBuf::new(),
            startup: path!(""),
            recent: path!(""),
            send_to: path!(""),
            start_menu: PathBuf::new(),
            my_music: PathBuf::new(),
            my_videos: PathBuf::new(),
            desktop_directory: PathBuf::new(),
            my_computer: path!(""),
            network_shortcuts: path!(""),
            fonts: PathBuf::new(),
            templates: PathBuf::new(),
            common_startup_menu: path!(""),
            common_programs: path!(""),
            common_startup: path!(""),
            common_desktop_directory: path!(""),
            application_data: PathBuf::new(),
            printer_shortcuts: path!(""),
            local_application_data: PathBuf::new(),
            internet_cache: PathBuf::new(),
            cookies: path!(""),
            history: path!(""),
            common_application_data: PathBuf::new(),
            windows: path!(""),
            system: path!(""),
            program_files: PathBuf::new(),
            my_pictures: PathBuf::new(),
            user_profile: PathBuf::new(),
            system_x86: path!(""),
            program_files_x86: path!(""),
            common_program_files: path!(""),
            common_program_files_x86: path!(""),
            common_templates: PathBuf::new(),
            common_documents: path!(""),
            common_admin_tools: path!(""),
            admin_tools: path!(""),
            common_music: path!(""),
            common_pictures: path!(""),
            common_videos: path!(""),
            resources: path!(""),
            localized_resources: path!(""),
            common_oem_links: path!(""),
            cd_burning: path!(""),
            temp_path: PathBuf::new(),
            exe_path: PathBuf::new(),
            program_dir_name: "GameMakerStudio2".to_owned(),
            program_name: "GameMakerStudio2".to_owned(),
            program_name_pretty: "GameMaker Studio 2".to_owned(),
            runtime_uri: "https://gms.yoyogames.com/Zeus-Runtime.rss".to_owned(),
            update_uri: "https://gms.yoyogames.com/update-mac.rss".to_owned(),
            release_notes_uri: "https://gms.yoyogames.com/ReleaseNotes.html".to_owned(),
            runtime_release_notes_uri: "https://gms.yoyogames.com/release-notes-runtime.html"
                .to_owned(),
            runtime_base_location: PathBuf::new(),
            runtime_location: PathBuf::new(),
            base_options_dir: PathBuf::new(),
            asset_compiler_path: PathBuf::new(),
            igor_path: PathBuf::new(),
            lib_compatibility_path: PathBuf::new(),
            runner_path: PathBuf::new(),
            html5_runner_path: PathBuf::new(),
            webserver_path: PathBuf::new(),
            licenses_path: PathBuf::new(),
            java_exe_path: PathBuf::new(),
            adb_exe_path: PathBuf::new(),
            keytool_exe_path: PathBuf::new(),
            openssl_exe_path: PathBuf::new(),
            skin_path: PathBuf::new(),
            user_skin_path: PathBuf::new(),
            user_override_directory: PathBuf::new(),
            default_skin: PathBuf::new(),
            current_skin: PathBuf::new(),
            system_directory: PathBuf::new(),
            system_cache_directory: PathBuf::new(),
            local_directory: PathBuf::new(),
            local_cache_directory: PathBuf::new(),
            temp_directory: PathBuf::new(),
            asset_compiler_cache_directory: PathBuf::new(),
            ide_cache_directory: PathBuf::new(),
            my_projects_directory: PathBuf::new(),
            base_project: PathBuf::new(),
            default_font: "Open Sans".to_string(),
            default_style: "Regular".to_string(),
            default_font_size: "9".to_string(),
            login_uri: "https://api.yoyogames.com".to_owned(),
            accounts_uri: "https://accounts.yoyogames.com".to_string(),
            marketplace_uri: "https://marketplace.yoyogames.com".to_string(),
            marketplace_api_uri: "https://api.yoyogames.com".to_string(),
            carousel_slides_uri: "https://api.yoyogames.com/api/2/slideshow.json".to_string(),
            user_directory: PathBuf::new(),
            user_cache_directory: PathBuf::new(),
            project_full_filename: PathBuf::new(),
            project_dir: PathBuf::new(),
            project_name: String::new(),
            project_cache_directory_name: String::new(),
            options_dir: PathBuf::new(),
        }
    }
}
