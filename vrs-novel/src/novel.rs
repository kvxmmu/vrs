use {
    crate::error::LoadFromDirectoryError,
    std::{
        fs,
        io,
        ops::Deref,
        path::{
            Path,
            PathBuf,
        },
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageSize {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone)]
pub struct NovelIcon {
    pub high: PathBuf,
    pub low: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Novel {
    pub image_size: ImageSize,
    pub icon: NovelIcon,
    pub title: String,
}

impl Novel {
    pub fn from_directory(
        directory: impl Deref<Target = Path>,
    ) -> Result<Self, LoadFromDirectoryError> {
        let info = fs::read_to_string(directory.join("info.txt"))?;
        let img = fs::read_to_string(directory.join("img.ini"))?;

        let title = info
            .split('=')
            .nth(1)
            .ok_or(LoadFromDirectoryError::NoTitle)?
            .trim()
            .to_owned();
        let mut width = None::<u16>;
        let mut height = None::<u16>;

        for size_const in img.split('\n') {
            match size_const.find('=') {
                Some(pos) => match &size_const[..pos] {
                    "width" => {
                        width = size_const[pos + 1..].trim().parse().ok();
                    }

                    "height" => {
                        height = size_const[pos + 1..].trim().parse().ok();
                    }

                    _ => continue,
                },
                None => continue,
            }
        }

        let (width, height) = width
            .zip(height)
            .ok_or(LoadFromDirectoryError::NoImageSizeFound)?;

        let (high_icon, low_icon) =
            (directory.join("icon-high.png"), directory.join("icon.png"));

        if !high_icon.exists() {
            return Err(LoadFromDirectoryError::NoHighResolutionIcon);
        }
        if !low_icon.exists() {
            return Err(LoadFromDirectoryError::NoLowResolutionIcon);
        }

        Ok(Self {
            title,
            image_size: ImageSize { width, height },
            icon: NovelIcon {
                high: high_icon,
                low: low_icon,
            },
        })
    }
}
