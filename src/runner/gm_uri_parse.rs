use once_cell::sync::Lazy;

// gml_Object_Object1_Create_0
/*
gml_Object_Game_Create_0:46
gml_Script_deserialize_Configuration_gml_GlobalScript_Configuration:35
gml_Object_Game_Create_0:256
gml_Script_Camera:370
gml_Script_Camera:373
gml_Script_set_view_size_Camera_gml_GlobalScript_CameraClass:110
gml_Script_target_mistria_gui_Camera_gml_GlobalScript_CameraClass:45
gml_Script_target_window_gui_Camera_gml_GlobalScript_CameraClass:77
gml_GlobalScript_Boombox_3740_play_track_Boombox_gml_GlobalScript_Boombox:141
*/

static SCRIPT_REGEX: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r#"gml_\w*gml_GlobalScript_(\w*):(\d*)"#).unwrap());

#[allow(dead_code)]
fn parse_script(input: &str) -> Option<String> {
    if let Some(captures) = SCRIPT_REGEX.captures(input) {
        let mut cap_iter = captures.iter();

        let entire_match = cap_iter.next().unwrap().unwrap();

        if let Some(script_name) = cap_iter.next().unwrap() {
            if let Some(line) = cap_iter.next().unwrap() {
                let mut output = String::with_capacity(input.len());
                output.push_str(&input[..entire_match.start()]);
                output.push_str(&format!(
                    "scripts/{name}/{name}.gml:{line}:0",
                    name = script_name.as_str(),
                    line = line.as_str(),
                ));
                output.push_str(&input[line.end()..]);

                return Some(output);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_script() {
        let output = super::parse_script(
            "[9/26/2020 12:27:12 AM] TRACE gml_Script_set_view_size_Camera_gml_GlobalScript_CameraClass:110 -- Creating new Mistria GUI! [Reason: View Resize]",
        )
        .unwrap();
        assert_eq!(output, "[9/26/2020 12:27:12 AM] TRACE scripts/CameraClass/CameraClass.gml:110:0 -- Creating new Mistria GUI! [Reason: View Resize]");

        assert_eq!(
            super::parse_script("[9/26/2020 12:52:54 AM] TRACE gml_Script_play_track_Boombox_gml_GlobalScript_Boombox:123 -- Playing new music track").unwrap(),
            "[9/26/2020 12:52:54 AM] TRACE scripts/Boombox/Boombox.gml:123:0 -- Playing new music track"
        );
        assert_eq!(
            super::parse_script("[9/26/2020 12:52:54 AM] TRACE gml_Script_anon___add_track_stop_to_chain_Boombox_gml_GlobalScript_Boombox_2441___add_track_stop_to_chain_Boombox_gml_GlobalScript_Boombox:103 -- Set music state to FadeOut").unwrap(),
            "[9/26/2020 12:52:54 AM] TRACE scripts/Boombox/Boombox.gml:103:0 -- Set music state to FadeOut"
        );

        // 
    }
}
