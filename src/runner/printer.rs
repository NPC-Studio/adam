use super::ac_styler::*;
use super::gm_uri_parse::*;
use std::path::Path;

#[derive(Debug)]
pub struct Printer {
    ac_stylers: Vec<AcStyler>,
    gm_uri_parser: GmUriParser,
}

impl Printer {
    const SHUTDOWN: [&'static str; 3] = [
        "Attempting to set gamepadcount to",
        "Not shutting down steam as it is not initialised",
        "Script_Free called",
    ];

    pub fn new(scripts_directory: &Path) -> Self {
        Self {
            ac_stylers: vec![
                AcStyler {
                    matchers: vec!["error", "ERROR"],
                    style: console::Style::new().red().bright(),
                },
                AcStyler {
                    matchers: vec!["warn", "WARN"],
                    style: console::Style::new().yellow().bright(),
                },
                AcStyler {
                    matchers: vec!["info", "INFO", "debug", "DEBUG"],
                    style: console::Style::new().green().bright(),
                },
                AcStyler {
                    matchers: vec!["trace", "TRACE"],
                    style: console::Style::new().dim().cyan(),
                },
            ],
            gm_uri_parser: GmUriParser::new(scripts_directory),
        }
    }

    pub fn print_line(&mut self, mut msg: String) {
        for styler in self.ac_stylers.iter() {
            styler.style(&mut msg);
        }
        self.gm_uri_parser.parse(&mut msg);

        if Self::SHUTDOWN.iter().any(|v| msg.contains(v)) == false {
            println!("{}", msg);
        }
    }
}
