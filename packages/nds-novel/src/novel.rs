use {
    crate::script::Script,
    std::{
        collections::HashMap,
        path::PathBuf,
    },
};

#[derive(Debug, Clone)]
pub struct NovelResources {
    pub background: PathBuf,
    pub foreground: PathBuf,
    pub script: PathBuf,
    pub sound: PathBuf,

    pub icon: NovelIconQuality,
}

#[derive(Debug, Clone)]
pub struct NovelIconQuality {
    pub high: PathBuf,
    pub low: PathBuf,
}

pub struct Novel {
    pub resources: NovelResources,
    pub device_resolution: (u16, u16),

    // TODO: Implement index
    scripts_index: HashMap<PathBuf, Script>,
}

impl Novel {}
