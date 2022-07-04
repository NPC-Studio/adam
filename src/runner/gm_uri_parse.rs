use regex::Regex;
use std::collections::HashMap;
use std::fmt::Write;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct GmUriParser {
    global_script_regex: Regex,
    object_regex: Regex,
    script_regex: Regex,

    script_files: HashMap<String, String>,
    cache_functions_to_files: HashMap<String, String>,

    e_global_script_regex: Regex,
    e_object_regex: Regex,
    e_script_regex: Regex,
}

impl GmUriParser {
    pub fn new<P: AsRef<Path>>(scripts_directory: P) -> Self {
        let global_script_regex = Regex::new(r#"\w*GlobalScript_(\w*):(\d*)"#).unwrap();
        let object_regex = Regex::new(r#"gml_Object_(\w*):(\d*)"#).unwrap();
        let script_regex = Regex::new(r#"gml_Script_(\w*):(\d*)"#).unwrap();

        let e_global_script_regex =
            Regex::new(r#"\w*GlobalScript_(\w*)\s*\([line ]*(\d*)\)"#).unwrap();
        let e_object_regex = Regex::new(r#"\w*gml_Object_(\w*)\s*\([line ]*(\d*)\)"#).unwrap();
        let e_script_regex = Regex::new(r#"\w?gml_Script_(\w*)\s*\([line ]*(\d*)\)"#).unwrap();

        let mut files = HashMap::new();

        for file in WalkDir::new(scripts_directory)
            .into_iter()
            .filter_map(|v| v.ok())
        {
            if file.path().extension().and_then(|v| v.to_str()) == Some("gml") {
                if let Some(stem) = file.path().file_stem() {
                    if let Ok(v) = std::fs::read_to_string(file.path()) {
                        files.insert(stem.to_string_lossy().to_string(), v);
                    }
                }
            }
        }

        Self {
            global_script_regex,
            object_regex,
            script_regex,
            script_files: files,
            cache_functions_to_files: HashMap::new(),
            e_global_script_regex,
            e_object_regex,
            e_script_regex,
        }
    }

    pub fn parse(&mut self, input: &mut String) {
        if let Some(output) = self
            .parse_global_script(input)
            .or_else(|| self.parse_object(input))
            .or_else(|| self.parse_script(input))
            .or_else(|| self.e_parse_global_script(input))
            .or_else(|| self.e_parse_object(input))
            .or_else(|| self.e_parse_script(input))
        {
            *input = output;
        }
    }

    fn parse_global_script(&self, input: &str) -> Option<String> {
        if let Some(captures) = self.global_script_regex.captures(input) {
            let mut cap_iter = captures.iter();

            let entire_match = cap_iter.next().unwrap().unwrap();

            if let Some(script_name) = cap_iter.next().unwrap() {
                if let Some(line) = cap_iter.next().unwrap() {
                    let mut output = String::with_capacity(input.len());
                    output.push_str(&input[..entire_match.start()]);
                    let line_txt = match line.as_str().parse::<i32>() {
                        Ok(v) => (v + 1).to_string(),
                        Err(_) => line.as_str().to_owned(),
                    };
                    write!(
                        output,
                        "scripts/{name}/{name}.gml:{line}:0",
                        name = script_name.as_str(),
                        line = line_txt,
                    )
                    .unwrap();
                    output.push_str(&input[line.end()..]);

                    return Some(output);
                }
            }
        }

        None
    }

    fn e_parse_global_script(&self, input: &str) -> Option<String> {
        if let Some(captures) = self.e_global_script_regex.captures(input) {
            let mut cap_iter = captures.iter();

            let entire_match = cap_iter.next().unwrap().unwrap();

            if let Some(script_name) = cap_iter.next().unwrap() {
                if let Some(line) = cap_iter.next().unwrap() {
                    let mut output = String::with_capacity(input.len());
                    output.push_str(&input[..entire_match.start()]);
                    let line_txt = match line.as_str().parse::<i32>() {
                        Ok(v) => (v + 1).to_string(),
                        Err(_) => line.as_str().to_owned(),
                    };

                    write!(
                        output,
                        "scripts/{name}/{name}.gml:{line}:0",
                        name = script_name.as_str(),
                        line = line_txt,
                    )
                    .unwrap();
                    output.push_str(&input[entire_match.end()..]);

                    return Some(output);
                }
            }
        }

        None
    }

    fn parse_object(&self, input: &str) -> Option<String> {
        if let Some(captures) = self.object_regex.captures(input) {
            let mut cap_iter = captures.iter();

            let entire_match = cap_iter.next().unwrap().unwrap();

            if let Some(object_name) = cap_iter.next().unwrap() {
                let split = object_name.as_str().split('_').collect::<Vec<_>>();
                // for safety
                if split.len() < 2 {
                    return None;
                }

                let o_name = {
                    let mut name =
                        split
                            .iter()
                            .take(split.len() - 2)
                            .fold(String::new(), |mut accum, v| {
                                accum.push_str(v);
                                accum.push('_');
                                accum
                            });
                    name.pop();
                    name
                };

                let event_name = {
                    let mut name =
                        split
                            .iter()
                            .skip(split.len() - 2)
                            .fold(String::new(), |mut accum, v| {
                                accum.push_str(v);
                                accum.push('_');
                                accum
                            });
                    name.pop();
                    name
                };

                if let Some(line) = cap_iter.next().unwrap() {
                    let mut output = String::with_capacity(input.len());
                    output.push_str(&input[..entire_match.start()]);

                    let line_txt = match line.as_str().parse::<i32>() {
                        Ok(v) => (v + 1).to_string(),
                        Err(_) => line.as_str().to_owned(),
                    };

                    write!(
                        output,
                        "objects/{}/{}.gml:{}:0",
                        o_name, event_name, line_txt,
                    )
                    .unwrap();
                    output.push_str(&input[entire_match.end()..]);

                    return Some(output);
                }
            }
        }

        None
    }

    fn e_parse_object(&self, input: &str) -> Option<String> {
        if let Some(captures) = self.e_object_regex.captures(input) {
            let mut cap_iter = captures.iter();

            let entire_match = cap_iter.next().unwrap().unwrap();

            if let Some(object_name) = cap_iter.next().unwrap() {
                let split = object_name.as_str().split('_').collect::<Vec<_>>();
                // for safety
                if split.len() < 2 {
                    return None;
                }

                let o_name = {
                    let mut name =
                        split
                            .iter()
                            .take(split.len() - 2)
                            .fold(String::new(), |mut accum, v| {
                                accum.push_str(v);
                                accum.push('_');
                                accum
                            });
                    name.pop();
                    name
                };

                let event_name = {
                    let mut name =
                        split
                            .iter()
                            .skip(split.len() - 2)
                            .fold(String::new(), |mut accum, v| {
                                accum.push_str(v);
                                accum.push('_');
                                accum
                            });
                    name.pop();
                    name
                };

                if let Some(line) = cap_iter.next().unwrap() {
                    let mut output = String::with_capacity(input.len());
                    output.push_str(&input[..entire_match.start()]);
                    let line_txt = match line.as_str().parse::<i32>() {
                        Ok(v) => (v + 1).to_string(),
                        Err(_) => line.as_str().to_owned(),
                    };

                    write!(
                        output,
                        "objects/{}/{}.gml:{}:0",
                        o_name, event_name, line_txt,
                    )
                    .unwrap();
                    output.push_str(&input[entire_match.end()..]);

                    return Some(output);
                }
            }
        }

        None
    }

    fn parse_script(&mut self, input: &str) -> Option<String> {
        if let Some(captures) = self.script_regex.captures(input) {
            let mut cap_iter = captures.iter();

            let entire_match = cap_iter.next().unwrap().unwrap();

            if let Some(script_name) = cap_iter.next().unwrap() {
                let script_files = &self.script_files;

                let found_script_fname = self
                    .cache_functions_to_files
                    .entry(script_name.as_str().to_owned())
                    .or_insert_with(|| {
                        let func_finder =
                            Regex::new(&format!(r#"function\s*{}\s*\("#, script_name.as_str()))
                                .unwrap();

                        if let Some(output) = script_files.iter().find_map(|(fname, data)| {
                            if func_finder.is_match(data) {
                                Some(fname.clone())
                            } else {
                                None
                            }
                        }) {
                            output
                        } else {
                            // lol fuk
                            entire_match.as_str().to_owned()
                        }
                    });

                if let Some(line) = cap_iter.next().unwrap() {
                    let mut output = String::with_capacity(input.len());
                    output.push_str(&input[..entire_match.start()]);

                    let line_txt = match line.as_str().parse::<i32>() {
                        Ok(v) => (v + 1).to_string(),
                        Err(_) => line.as_str().to_owned(),
                    };

                    write!(
                        output,
                        "scripts/{name}/{name}.gml:{line}:0",
                        name = found_script_fname,
                        line = line_txt,
                    )
                    .unwrap();
                    output.push_str(&input[line.end()..]);

                    return Some(output);
                }
            }
        }

        None
    }

    fn e_parse_script(&mut self, input: &str) -> Option<String> {
        if let Some(captures) = self.e_script_regex.captures(input) {
            let mut cap_iter = captures.iter();

            let entire_match = cap_iter.next().unwrap().unwrap();

            if let Some(script_name) = cap_iter.next().unwrap() {
                let script_files = &self.script_files;

                let found_script_fname = self
                    .cache_functions_to_files
                    .entry(script_name.as_str().to_owned())
                    .or_insert_with(|| {
                        let func_finder =
                            Regex::new(&format!(r#"function\s*{}\s*\("#, script_name.as_str()))
                                .unwrap();

                        if let Some(output) = script_files.iter().find_map(|(fname, data)| {
                            if func_finder.is_match(data) {
                                Some(fname.clone())
                            } else {
                                None
                            }
                        }) {
                            output
                        } else {
                            // lol fuk
                            entire_match.as_str().to_owned()
                        }
                    });

                if let Some(line) = cap_iter.next().unwrap() {
                    let mut output = String::with_capacity(input.len());
                    output.push_str(&input[..entire_match.start()]);

                    let line_txt = match line.as_str().parse::<i32>() {
                        Ok(v) => (v + 1).to_string(),
                        Err(_) => line.as_str().to_owned(),
                    };

                    write!(
                        output,
                        "scripts/{name}/{name}.gml:{line}:0",
                        name = found_script_fname,
                        line = line_txt,
                    )
                    .unwrap();
                    output.push_str(&input[line.end()..]);

                    return Some(output);
                }
            }
        }

        None
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn parse_global_script() {
        let parser = super::GmUriParser::new("./");

        let output = parser.parse_global_script(
            "[9/26/2020 12:27:12 AM] TRACE gml_Script_set_view_size_Camera_gml_GlobalScript_CameraClass:110 -- Creating new Mistria GUI! [Reason: View Resize]",
        )
        .unwrap();
        assert_eq!(output, "[9/26/2020 12:27:12 AM] TRACE scripts/CameraClass/CameraClass.gml:111:0 -- Creating new Mistria GUI! [Reason: View Resize]");

        assert_eq!(
            parser.parse_global_script("[9/26/2020 12:52:54 AM] TRACE gml_Script_play_track_Boombox_gml_GlobalScript_Boombox:123 -- Playing new music track").unwrap(),
            "[9/26/2020 12:52:54 AM] TRACE scripts/Boombox/Boombox.gml:124:0 -- Playing new music track"
        );
        assert_eq!(
            parser.parse_global_script("[9/26/2020 12:52:54 AM] TRACE gml_Script_anon___add_track_stop_to_chain_Boombox_gml_GlobalScript_Boombox_2441___add_track_stop_to_chain_Boombox_gml_GlobalScript_Boombox:103 -- Set music state to FadeOut").unwrap(),
            "[9/26/2020 12:52:54 AM] TRACE scripts/Boombox/Boombox.gml:103:0 -- Set music state to FadeOut"
        );
        assert_eq!(
            parser.parse_global_script("[9/26/2020 12:52:54 AM] TRACE gml_Script_anon___add_track_stop_to_chain_Boombox_gml_GlobalScript_Boombox_2441___add_track_stop_to_chain_Boombox_gml_GlobalScript_run_my_game:103 -- Set music state to FadeOut").unwrap(),
            "[9/26/2020 12:52:54 AM] TRACE scripts/run_my_game/run_my_game.gml:103:0 -- Set music state to FadeOut"
        );
    }

    #[test]
    fn parse_object() {
        let parser = super::GmUriParser::new("./");

        let output = parser.parse_object("[9/26/2020 11:26:04 AM] TRACE gml_Object_Game_Create_0:256 --   attempted to load save.json").unwrap();
        assert_eq!(
            output, "[9/26/2020 11:26:04 AM] TRACE objects/Game/Create_0.gml:257:0 --   attempted to load save.json"
        );

        let output = parser.parse_object("[9/26/2020 11:26:04 AM] WARN gml_Object_Game_Create_0:1032 -- Gabe is doing some graphic stuff here that he doesn't know where else to put...").unwrap();
        assert_eq!(
            output, "[9/26/2020 11:26:04 AM] WARN objects/Game/Create_0.gml:1032:0 -- Gabe is doing some graphic stuff here that he doesn't know where else to put..."
        );

        let output = parser.parse_object("[9/26/2020 11:26:04 AM] WARN gml_Object_obj_player_Create_0:1032 -- Gabe is doing some graphic stuff here that he doesn't know where else to put...").unwrap();
        assert_eq!(
            output, "[9/26/2020 11:26:04 AM] WARN objects/obj_player/Create_0.gml:1032:0 -- Gabe is doing some graphic stuff here that he doesn't know where else to put..."
        );
    }

    #[test]
    fn parse_script_functions() {
        let mut parser =
            super::GmUriParser::new("C:/Users/jjspi/Documents/Projects/Gms2/SwordAndField/scripts");

        let output = parser.parse_script("[9/26/2020 11:26:04 AM] TRACE gml_Script_Camera:370 --   attempted to load save.json").unwrap();
        assert_eq!(
            output, "[9/26/2020 11:26:04 AM] TRACE scripts/CameraClass/CameraClass.gml:370:0 --   attempted to load save.json"
        );

        let output = parser.parse_script("[9/26/2020 11:26:04 AM] TRACE gml_Script_buffer_to_json_map:370 -- underscore test").unwrap();
        assert_eq!(
            output, "[9/26/2020 11:26:04 AM] TRACE scripts/buffer_to_json_map/buffer_to_json_map.gml:370:0 -- underscore test"
        );
    }

    // #[test]
    // fn parse() {
    //     let mut parser =
    //         super::GmUriParser::new("C:/Users/jjspi/Documents/Projects/Gms2/SwordAndField/scripts");

    //     let output = parser.parse("[9/26/2020 11:26:04 AM] TRACE gml_Script_Camera:370 --   attempted to load save.json").unwrap();
    //     assert_eq!(
    //         output, "[9/26/2020 11:26:04 AM] TRACE scripts/CameraClass/CameraClass.gml:370:0 --   attempted to load save.json"
    //     );

    //     let output = parser.parse("[9/26/2020 11:26:04 AM] TRACE gml_Object_Game_Create_0:256 --   attempted to load save.json").unwrap();
    //     assert_eq!(
    //         output, "[9/26/2020 11:26:04 AM] TRACE objects/Game/Create_0.gml:256:0 --   attempted to load save.json"
    //     );

    //     let output = parser.parse("[9/26/2020 11:26:04 AM] WARN gml_Object_Game_Create_0:1032 -- Gabe is doing some graphic stuff here that he doesn't know where else to put...").unwrap();
    //     assert_eq!(
    //         output, "[9/26/2020 11:26:04 AM] WARN objects/Game/Create_0.gml:1032:0 -- Gabe is doing some graphic stuff here that he doesn't know where else to put..."
    //     );

    //     let output = parser.parse(
    //         "[9/26/2020 12:27:12 AM] TRACE gml_Script_set_view_size_Camera_gml_GlobalScript_CameraClass:110 -- Creating new Mistria GUI! [Reason: View Resize]",
    //     )
    //     .unwrap();
    //     assert_eq!(output, "[9/26/2020 12:27:12 AM] TRACE scripts/CameraClass/CameraClass.gml:110:0 -- Creating new Mistria GUI! [Reason: View Resize]");

    //     assert_eq!(
    //         parser.parse("[9/26/2020 12:52:54 AM] TRACE gml_Script_play_track_Boombox_gml_GlobalScript_Boombox:123 -- Playing new music track").unwrap(),
    //         "[9/26/2020 12:52:54 AM] TRACE scripts/Boombox/Boombox.gml:123:0 -- Playing new music track"
    //     );
    //     assert_eq!(
    //         parser.parse("[9/26/2020 12:52:54 AM] TRACE gml_Script_anon___add_track_stop_to_chain_Boombox_gml_GlobalScript_Boombox_2441___add_track_stop_to_chain_Boombox_gml_GlobalScript_Boombox:103 -- Set music state to FadeOut").unwrap(),
    //         "[9/26/2020 12:52:54 AM] TRACE scripts/Boombox/Boombox.gml:103:0 -- Set music state to FadeOut"
    //     );
    // }
}
