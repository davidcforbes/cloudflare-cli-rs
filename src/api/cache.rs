use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PurgeAll {
    pub purge_everything: bool,
}

#[derive(Debug, Serialize)]
pub struct PurgeFiles {
    pub files: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PurgeTags {
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PurgeHosts {
    pub hosts: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PurgePrefixes {
    pub prefixes: Vec<String>,
}
