use {
    std::io,
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum LoadFromDirectoryError {
    #[error("I/O Error: {0:?}")]
    Io(#[from] io::Error),

    #[error("No title specified")]
    NoTitle,

    #[error("No desirable image size found")]
    NoImageSizeFound,

    #[error("Invalid image size (possibly not a number)")]
    InvalidImageSize,

    #[error("icon-high.png was not found")]
    NoHighResolutionIcon,

    #[error("icon.png was not found")]
    NoLowResolutionIcon,
}
