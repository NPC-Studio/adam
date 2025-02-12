use aho_corasick::AhoCorasickBuilder;
use camino::Utf8Path;
use gml_log_parser::ScriptMappings;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Printer {
    str_to_style: std::collections::HashMap<String, console::Style>,
    script_mappings: ScriptMappings,
    aho_corasick: aho_corasick::AhoCorasick,
}

impl Printer {
    const IGNORE_LINES: [&'static str; 6] = [
        "Attempting to set gamepadcount to",
        "Not shutting down steam as it is not initialised",
        "Script_Free called",
        "ConnectWrap with g_network_async_connect",
        "###game_end###",
        "Unsetting previous scheduler resolution",
    ];

    pub fn new(scripts_directory: &Utf8Path) -> Self {
        let str_to_style = HashMap::from([
            ("error".to_string(), console::Style::new().red().bright()),
            ("warn".to_string(), console::Style::new().yellow().bright()),
            ("info".to_string(), console::Style::new().green().bright()),
            ("trace".to_string(), console::Style::new().dim().cyan()),
        ]);

        let aho_corasick = AhoCorasickBuilder::new()
            .ascii_case_insensitive(true)
            .build(str_to_style.keys());

        Self {
            str_to_style,
            script_mappings: ScriptMappings::from_path(scripts_directory),
            aho_corasick,
        }
    }

    pub fn print_line(&mut self, msg: String) {
        if Self::IGNORE_LINES.iter().any(|v| msg.contains(v)) {
            return;
        }

        let mut output = String::new();
        self.aho_corasick
            .replace_all_with(&msg, &mut output, |_, txt, buff| {
                if let Some(style) = self.str_to_style.get(&txt.to_ascii_lowercase()) {
                    buff.push_str(&style.apply_to(txt).to_string())
                } else {
                    buff.push_str(txt);
                }

                true
            });

        let output =
            if let Some(parsed_output) = gml_log_parser::parse(&output, &self.script_mappings) {
                parsed_output
            } else {
                output
            };

        println!("{}", output);
    }
}
