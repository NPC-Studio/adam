#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    pub operation: Operation,
    pub yyp_path: std::path::PathBuf,
    pub igor_path: std::path::PathBuf,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operation {
    Build,
    Run,
}
