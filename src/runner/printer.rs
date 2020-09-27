use super::gm_uri_parse::*;
use aho_corasick::AhoCorasickBuilder;
use std::path::Path;

#[derive(Debug)]
pub struct Printer {
    str_to_style: std::collections::HashMap<String, console::Style>,
    gm_uri_parser: GmUriParser,
    aho_corasick: aho_corasick::AhoCorasick,
}

impl Printer {
    const SHUTDOWN: [&'static str; 3] = [
        "Attempting to set gamepadcount to",
        "Not shutting down steam as it is not initialised",
        "Script_Free called",
    ];

    pub fn new(scripts_directory: &Path) -> Self {
        let str_to_style = maplit::hashmap! {
            "error".to_string() => console::Style::new().red().bright(),
            "warn".to_string() => console::Style::new().yellow().bright(),
            "info".to_string() => console::Style::new().green().bright(),
            "trace".to_string() => console::Style::new().dim().cyan()
        };

        let aho_corasick = AhoCorasickBuilder::new()
            .ascii_case_insensitive(true)
            .build(str_to_style.keys());

        Self {
            str_to_style,
            gm_uri_parser: GmUriParser::new(scripts_directory),
            aho_corasick,
        }
    }

    pub fn print_line(&mut self, msg: String) {
        if Self::SHUTDOWN.iter().any(|v| msg.contains(v)) {
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
        self.gm_uri_parser.parse(&mut output);

        println!("{}", output);
    }
}
