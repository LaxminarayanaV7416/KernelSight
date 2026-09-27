use std::collections::HashMap;

const PROCFS_DESCRIPTION: &str = include_str!("../procfs_description.json");

struct LinesDetails {}

struct PidFolderConfig {}

struct PidConfig {
    cachable: bool,
    root_required: bool,
    newline_parser: bool,
    tab_parser: bool,
    colon_parser: bool,
    space_parser: bool,
    string_parser: bool,
    lines_details: LinesDetails,
}

struct ProcfsRootConfig {
    is_root: bool,
}

struct ProcfsPidConfig {
    files: HashMap<String, PidConfig>,
    folders: HashMap<String, PidFolderConfig>,
}

struct ProcfsConfig {
    root: ProcfsRootConfig,
    procfs_pid: HashMap<String, ProcfsPidConfig>,
}