use crate::igor::BuildData;
use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

macro_rules! path {
    ($($arg:tt)*) => {{
        camino::Utf8PathBuf::from(std::fmt::format(format_args!($($arg)*)))
    }}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GmMacros {
    #[serde(rename = "Desktop")]
    desktop: Utf8PathBuf,
    #[serde(rename = "Programs")]
    programs: Utf8PathBuf,
    #[serde(rename = "MyDocuments")]
    my_documents: Utf8PathBuf,
    #[serde(rename = "Favorites")]
    favorites: Utf8PathBuf,
    #[serde(rename = "Startup")]
    startup: Utf8PathBuf,
    #[serde(rename = "Recent")]
    recent: Utf8PathBuf,
    #[serde(rename = "SendTo")]
    send_to: Utf8PathBuf,
    #[serde(rename = "StartMenu")]
    start_menu: Utf8PathBuf,
    #[serde(rename = "MyMusic")]
    my_music: Utf8PathBuf,
    #[serde(rename = "MyVideos")]
    my_videos: Utf8PathBuf,
    #[serde(rename = "DesktopDirectory")]
    desktop_directory: Utf8PathBuf,
    #[serde(rename = "MyComputer")]
    my_computer: Utf8PathBuf,
    #[serde(rename = "NetworkShortcuts")]
    network_shortcuts: Utf8PathBuf,
    #[serde(rename = "Fonts")]
    fonts: Utf8PathBuf,
    #[serde(rename = "Templates")]
    templates: Utf8PathBuf,
    #[serde(rename = "CommonStartMenu")]
    common_startup_menu: Utf8PathBuf,
    #[serde(rename = "CommonPrograms")]
    common_programs: Utf8PathBuf,
    #[serde(rename = "CommonStartup")]
    common_startup: Utf8PathBuf,
    #[serde(rename = "CommonDesktopDirectory")]
    common_desktop_directory: Utf8PathBuf,
    #[serde(rename = "ApplicationData")]
    application_data: Utf8PathBuf,
    #[serde(rename = "PrinterShortcuts")]
    printer_shortcuts: Utf8PathBuf,
    #[serde(rename = "LocalApplicationData")]
    local_application_data: Utf8PathBuf,
    #[serde(rename = "InternetCache")]
    internet_cache: Utf8PathBuf,
    #[serde(rename = "Cookies")]
    cookies: Utf8PathBuf,
    #[serde(rename = "History")]
    history: Utf8PathBuf,
    #[serde(rename = "CommonApplicationData")]
    common_application_data: Utf8PathBuf,
    #[serde(rename = "Windows")]
    windows: Utf8PathBuf,
    #[serde(rename = "System")]
    system: Utf8PathBuf,
    #[serde(rename = "ProgramFiles")]
    program_files: Utf8PathBuf,
    #[serde(rename = "MyPictures")]
    my_pictures: Utf8PathBuf,
    #[serde(rename = "UserProfile")]
    user_profile: Utf8PathBuf,
    #[serde(rename = "SystemX86")]
    system_x86: Utf8PathBuf,
    #[serde(rename = "ProgramFilesX86")]
    program_files_x86: Utf8PathBuf,
    #[serde(rename = "CommonProgramFiles")]
    common_program_files: Utf8PathBuf,
    #[serde(rename = "CommonProgramFilesX86")]
    common_program_files_x86: Utf8PathBuf,
    #[serde(rename = "CommonTemplates")]
    common_templates: Utf8PathBuf,
    #[serde(rename = "CommonDocuments")]
    common_documents: Utf8PathBuf,
    #[serde(rename = "CommonAdminTools")]
    common_admin_tools: Utf8PathBuf,
    #[serde(rename = "AdminTools")]
    admin_tools: Utf8PathBuf,
    #[serde(rename = "CommonMusic")]
    common_music: Utf8PathBuf,
    #[serde(rename = "CommonPictures")]
    common_pictures: Utf8PathBuf,
    #[serde(rename = "CommonVideos")]
    common_videos: Utf8PathBuf,
    #[serde(rename = "Resources")]
    resources: Utf8PathBuf,
    #[serde(rename = "LocalizedResources")]
    localized_resources: Utf8PathBuf,
    #[serde(rename = "CommonOemLinks")]
    common_oem_links: Utf8PathBuf,
    #[serde(rename = "CDBurning")]
    cd_burning: Utf8PathBuf,
    #[serde(rename = "TempPath")]
    temp_path: Utf8PathBuf,
    exe_path: Utf8PathBuf,
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
    runtime_base_location: Utf8PathBuf,
    #[serde(rename = "runtimeLocation")]
    runtime_location: Utf8PathBuf,
    base_options_dir: Utf8PathBuf,
    asset_compiler_path: Utf8PathBuf,
    pub igor_path: Utf8PathBuf,
    lib_compatibility_path: Utf8PathBuf,
    pub runner_path: Utf8PathBuf,
    pub x64_runner_path: Utf8PathBuf,
    html5_runner_path: Utf8PathBuf,
    webserver_path: Utf8PathBuf,
    licenses_path: Utf8PathBuf,
    java_exe_path: Utf8PathBuf,
    adb_exe_path: Utf8PathBuf,
    keytool_exe_path: Utf8PathBuf,
    openssl_exe_path: Utf8PathBuf,
    skin_path: Utf8PathBuf,
    user_skin_path: Utf8PathBuf,
    user_override_directory: Utf8PathBuf,
    default_skin: Utf8PathBuf,
    current_skin: Utf8PathBuf,
    system_directory: Utf8PathBuf,
    system_cache_directory: Utf8PathBuf,
    local_directory: Utf8PathBuf,
    local_cache_directory: Utf8PathBuf,
    temp_directory: Utf8PathBuf,
    asset_compiler_cache_directory: Utf8PathBuf,
    ide_cache_directory: Utf8PathBuf,
    my_projects_directory: Utf8PathBuf,
    base_project: Utf8PathBuf,
    default_font: String,
    default_style: String,
    default_font_size: String,
    oauth_opera_server: String,
    gxc_server: String,
    gxc_scopes: String,
    feature_flags_enable: String,
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
    pub user_directory: Utf8PathBuf,
    user_cache_directory: Utf8PathBuf,
    pub project_full_filename: Utf8PathBuf,
    pub project_dir: Utf8PathBuf,
    pub project_name: String,
    project_cache_directory_name: String,
    options_dir: Utf8PathBuf,
}

impl GmMacros {
    #[cfg(target_os = "windows")]
    pub fn new(build_data: &BuildData) -> Self {
        Self {
            igor_path: build_data
                .runtime_location
                .join("bin/igor/windows/x64/Igor.exe"),
            asset_compiler_path: build_data
                .runtime_location
                .join("bin/assetcompiler/windows/x64/GMAssetCompiler"),
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

            ..Self::create_internal(build_data)
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn new(build_data: &BuildData) -> Self {
        let application_data = build_data.user_dir.join(".config");
        let common_app_data = path!("/Users/Shared");
        let system_directory = common_app_data.join("GameMakerStudio2");

        let igor_path = if cfg!(target_arch = "aarch64") {
            build_data.runtime_location.join("bin/igor/osx/arm64/Igor")
        } else {
            build_data.runtime_location.join("bin/igor/osx/x64/Igor")
        };

        let asset_compiler_path = if cfg!(target_arch = "aarch64") {
            build_data
                .runtime_location
                .join("bin/assetcompiler/osx/arm64/GMAssetCompiler.dll")
        } else {
            build_data
                .runtime_location
                .join("bin/assetcompiler/osx/x64/GMAssetCompiler.dll")
        };

        Self {
            igor_path,
            asset_compiler_path,
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
                .join("var/folders/v_/r1l809l94_vbd75s98fbd6rr0000gn"),
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

            ..Self::create_internal(build_data)
        }
    }
    fn create_internal(build_data: &BuildData) -> Self {
        let BuildData {
            folders,
            project_filename,
            project_directory,
            user_dir,
            runtime_location,
            application_path,
            target_mask: _,
            license_folder: _,
            output_kind: _,
            config: _,
        } = build_data;

        Self {
            desktop: user_dir.join("Desktop"),
            my_documents: user_dir.join("Documents"),
            my_music: user_dir.join("Music"),
            my_videos: user_dir.join("Videos"),
            desktop_directory: user_dir.join("Desktop"),
            my_pictures: user_dir.join("Pictures"),
            user_profile: user_dir.clone(),

            exe_path: application_path.clone(),
            runtime_location: runtime_location.clone(),
            base_options_dir: runtime_location.join("BaseProject/options"),
            lib_compatibility_path: runtime_location.join("lib/compatibility.zip"),
            runner_path: runtime_location.join("windows/Runner.exe"),
            x64_runner_path: runtime_location.join("windows/x64/Runner.exe"),
            html5_runner_path: runtime_location.join("html5/scripts.html5.zip"),
            webserver_path: runtime_location.join("bin/GMWebServer.exe"),
            licenses_path: application_path.join("Licenses"),
            skin_path: application_path.join("GUI/Skins"),
            default_skin: application_path.join("GUI/Skins/Dark"),

            local_cache_directory: folders.cache.clone(),
            temp_directory: folders.tmp.clone(),
            // for some reason we need this here -- it will append `cache` and find the
            // `folders.cache` that way itself! that's chaos!
            asset_compiler_cache_directory: folders.main.clone(),
            my_projects_directory: user_dir.join("GameMakerStudio2"),
            base_project: runtime_location.join("BaseProject/BaseProject.yyp"),
            project_full_filename: project_directory
                .join(project_filename)
                .with_extension("yyp"),
            project_dir: project_directory.clone(),
            project_name: project_filename.to_owned(),
            project_cache_directory_name: "cache".to_owned(),
            options_dir: project_directory.join("options"),

            user_directory: build_data.license_folder.clone(),
            user_cache_directory: build_data.license_folder.join("cache"),

            ..Self::default()
        }
    }
}

impl Default for GmMacros {
    fn default() -> Self {
        Self {
            desktop: Utf8PathBuf::new(),
            programs: path!(""),
            my_documents: Utf8PathBuf::new(),
            favorites: Utf8PathBuf::new(),
            startup: path!(""),
            recent: path!(""),
            send_to: path!(""),
            start_menu: Utf8PathBuf::new(),
            my_music: Utf8PathBuf::new(),
            my_videos: Utf8PathBuf::new(),
            desktop_directory: Utf8PathBuf::new(),
            my_computer: path!(""),
            network_shortcuts: path!(""),
            fonts: Utf8PathBuf::new(),
            templates: Utf8PathBuf::new(),
            common_startup_menu: path!(""),
            common_programs: path!(""),
            common_startup: path!(""),
            common_desktop_directory: path!(""),
            application_data: Utf8PathBuf::new(),
            printer_shortcuts: path!(""),
            local_application_data: Utf8PathBuf::new(),
            internet_cache: Utf8PathBuf::new(),
            cookies: path!(""),
            history: path!(""),
            common_application_data: Utf8PathBuf::new(),
            windows: path!(""),
            system: path!(""),
            program_files: Utf8PathBuf::new(),
            my_pictures: Utf8PathBuf::new(),
            user_profile: Utf8PathBuf::new(),
            system_x86: path!(""),
            program_files_x86: path!(""),
            common_program_files: path!(""),
            common_program_files_x86: path!(""),
            common_templates: Utf8PathBuf::new(),
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
            temp_path: Utf8PathBuf::new(),
            exe_path: Utf8PathBuf::new(),
            program_dir_name: "GameMakerStudio2".to_owned(),
            program_name: "GameMakerStudio2".to_owned(),
            program_name_pretty: "GameMaker Studio 2".to_owned(),
            runtime_uri: "https://gms.yoyogames.com/Zeus-Runtime.rss".to_owned(),
            update_uri: "https://gms.yoyogames.com/update-mac.rss".to_owned(),
            release_notes_uri: "https://gms.yoyogames.com/ReleaseNotes.html".to_owned(),
            runtime_release_notes_uri: "https://gms.yoyogames.com/release-notes-runtime.html"
                .to_owned(),
            runtime_base_location: Utf8PathBuf::new(),
            runtime_location: Utf8PathBuf::new(),
            base_options_dir: Utf8PathBuf::new(),
            asset_compiler_path: Utf8PathBuf::new(),
            igor_path: Utf8PathBuf::new(),
            lib_compatibility_path: Utf8PathBuf::new(),
            runner_path: Utf8PathBuf::new(),
            x64_runner_path: Utf8PathBuf::new(),
            html5_runner_path: Utf8PathBuf::new(),
            webserver_path: Utf8PathBuf::new(),
            licenses_path: Utf8PathBuf::new(),
            java_exe_path: Utf8PathBuf::new(),
            adb_exe_path: Utf8PathBuf::new(),
            keytool_exe_path: Utf8PathBuf::new(),
            openssl_exe_path: Utf8PathBuf::new(),
            skin_path: Utf8PathBuf::new(),
            user_skin_path: Utf8PathBuf::new(),
            user_override_directory: Utf8PathBuf::new(),
            default_skin: Utf8PathBuf::new(),
            current_skin: Utf8PathBuf::new(),
            system_directory: Utf8PathBuf::new(),
            system_cache_directory: Utf8PathBuf::new(),
            local_directory: Utf8PathBuf::new(),
            local_cache_directory: Utf8PathBuf::new(),
            temp_directory: Utf8PathBuf::new(),
            asset_compiler_cache_directory: Utf8PathBuf::new(),
            ide_cache_directory: Utf8PathBuf::new(),
            my_projects_directory: Utf8PathBuf::new(),
            base_project: Utf8PathBuf::new(),
            default_font: "Open Sans".to_string(),
            default_style: "Regular".to_string(),
            default_font_size: "9".to_string(),
            oauth_opera_server: "https://oauth2.opera-api.com/oauth2/v1/".to_string(),
            gxc_server: "https://api.gmx.dev/".to_string(),
            gxc_scopes: "user+https://api.gmx.dev/gms:read+https://api.gmx.dev/gms:write"
                .to_string(),
            feature_flags_enable: "text".to_string(),
            login_uri: "https://api.yoyogames.com".to_owned(),
            accounts_uri: "https://accounts.yoyogames.com".to_string(),
            marketplace_uri: "https://marketplace.yoyogames.com".to_string(),
            marketplace_api_uri: "https://api.yoyogames.com".to_string(),
            carousel_slides_uri: "https://api.yoyogames.com/api/2/slideshow.json".to_string(),
            user_directory: Utf8PathBuf::new(),
            user_cache_directory: Utf8PathBuf::new(),
            project_full_filename: Utf8PathBuf::new(),
            project_dir: Utf8PathBuf::new(),
            project_name: String::new(),
            project_cache_directory_name: String::new(),
            options_dir: Utf8PathBuf::new(),
        }
    }
}
