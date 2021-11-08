use color_eyre::Section;
use std::path::{Path, PathBuf};

use crate::{gm_artifacts::Platform, AnyResult};

/// Loads in the license folder path and the visual studio path.
pub fn load_user_data(
    platform: &Platform,
    license_folder: &mut Option<PathBuf>,
    visual_studio_path: &mut Option<PathBuf>,
) -> AnyResult {
    // we gucci
    if license_folder.is_some() && visual_studio_path.is_some() {
        return Ok(());
    }

    let gms2_data = platform.gms2_data.clone();
    let um_json: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(gms2_data.join("um.json")).with_note(|| {
            format!(
                "Could not read path {}",
                gms2_data.join("um.json").display()
            )
        })?,
    )
    .with_note(|| "Couldn't parse `um.json` file.")?;

    let user_id: usize = um_json.get("userID").unwrap().as_str().unwrap().parse()?;
    let user_name = um_json
        .get("username")
        .unwrap()
        .as_str()
        .unwrap()
        .split('@')
        .next()
        .unwrap()
        .to_owned();

    if license_folder.is_none() {
        *license_folder = Some(
            platform
                .gms2_data
                .join(format!("{}_{}", user_name, user_id)),
        );
    }

    // we need a visual studio path...
    if visual_studio_path.is_none() {
        // the ide can give us one...
        let new_path = std::fs::read_to_string(
            gms2_data.join(&format!("{}_{}/local_settings.json", user_name, user_id)),
        )
        .ok()
        .and_then(|data| {
            let local_settings: serde_json::Value = serde_json::from_str(&data).unwrap();

            local_settings
                .get("machine.Platform Settings.Windows.visual_studio_path")
                .map(|v| {
                    let v = v.as_str().unwrap();
                    Path::new(v).to_owned()
                })
        })
        .unwrap_or_else(|| {
            Path::new("C:/Program Files (x86)/Microsoft Visual Studio 14.0/VC/bin/vcvars32.bat")
                .to_owned()
        });

        *visual_studio_path = Some(new_path);
    }

    Ok(())
}
