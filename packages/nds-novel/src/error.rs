use {
    nds_parser::error::ParseError,
    std::io,
    thiserror::Error,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ResourceLoadError {
    #[error("Requested file was not found")]
    FileNotFound,
}

#[derive(Debug, Error)]
pub enum LoadScriptError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum JumpToLabelError {
    #[error("Specified label was not found")]
    LabelNotFound,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum NovelLoadError {
    #[error("background directory was not found")]
    NoBackgroundDirectory,

    #[error("sound directory was not found")]
    NoSoundDirectory,

    #[error("foreground directory was not found")]
    NoForegroundDirectory,

    #[error("script directory was not found")]
    NoScriptDirectory,

    #[error("icon-high.png was not found")]
    NoHighResolutionIcon,

    #[error("icon.png was not found")]
    NoLowResolutionIcon,

    #[error("thumbnail.png was not found")]
    NoThumbnail,

    #[error("info.txt does not contain title")]
    NoTitle,

    #[error("Invalid img.ini file")]
    InvalidImgIni,
}
