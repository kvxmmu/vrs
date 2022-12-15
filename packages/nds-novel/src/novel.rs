use {
    crate::{
        error::{
            LoadScriptError,
            NovelLoadError,
        },
        script::Script,
    },
    nds_parser::parser::ParseScript,
    std::{
        fs,
        path::{
            Path,
            PathBuf,
        },
    },
};

macro_rules! resource_delegates {
    ($(
        $name:ident
    ),*) => {
        $(
            pub fn $name(
                &self,
                path: impl core::convert::AsRef<std::path::Path>,
            ) -> Result<PathBuf, $crate::error::ResourceLoadError> {
                let path = self.resources.$name.join(path);
                if path.exists() {
                    Ok(path)
                } else {
                    Err($crate::error::ResourceLoadError::FileNotFound)
                }
            }
        )*
    };
}

#[derive(Debug, Clone)]
pub struct NovelResources {
    pub background: PathBuf,
    pub foreground: PathBuf,
    pub script: PathBuf,
    pub sound: PathBuf,

    pub icon: NovelIcon,
}

#[derive(Debug, Clone)]
pub struct NovelIcon {
    pub high: PathBuf,
    pub low: PathBuf,

    pub thumbnail: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Novel {
    pub title: String,
    pub resources: NovelResources,
    pub device_resolution: (u16, u16),
    // TODO: Implement index, possibly polonius-the-crab can solve my
    // issue, but not today. Fuck NLL.
}

impl Novel {
    pub fn try_load(
        path: impl AsRef<Path>,
    ) -> Result<Self, NovelLoadError> {
        fn try_create_resource(
            path: impl AsRef<Path>,
            next: impl AsRef<Path>,
            error: NovelLoadError,
        ) -> Result<PathBuf, NovelLoadError> {
            let joined = path.as_ref().join(next);
            if joined.exists() {
                Ok(joined)
            } else {
                Err(error)
            }
        }

        fn try_load_info(
            path: impl AsRef<Path>,
        ) -> Result<String, NovelLoadError> {
            let info = try_create_resource(
                path,
                "info.txt",
                NovelLoadError::NoTitle,
            )?;

            fs::read_to_string(info)
                .map_err(|_| NovelLoadError::NoTitle)?
                .lines()
                .filter_map(|l| l.split_once('='))
                .map(|(_, v)| v.to_owned())
                .next()
                .ok_or(NovelLoadError::NoTitle)
        }

        fn try_load_img(
            path: impl AsRef<Path>,
        ) -> Result<(u16, u16), NovelLoadError> {
            let img = try_create_resource(
                path,
                "img.ini",
                NovelLoadError::InvalidImgIni,
            )?;
            let text = fs::read_to_string(img)
                .map_err(|_| NovelLoadError::InvalidImgIni)?;

            let vals: Vec<(&str, u16)> = text
                .lines()
                .map(str::trim)
                .filter(|i| !i.is_empty())
                .filter_map(|line| line.split_once('='))
                .filter_map(|(key, value)| {
                    if let Ok(v) = value.parse::<u16>() {
                        Some((key, v))
                    } else {
                        None
                    }
                })
                .collect();

            let size = vals
                .iter()
                .find(|&&(k, _)| k == "height")
                .zip(vals.iter().find(|&&(k, _)| k == "width"));

            size.map(
                |(&(k1, v1), &(_, v2))| {
                    if k1 == "width" {
                        (v1, v2)
                    } else {
                        (v2, v1)
                    }
                },
            )
            .ok_or(NovelLoadError::InvalidImgIni)
        }

        let background = try_create_resource(
            &path,
            "background",
            NovelLoadError::NoBackgroundDirectory,
        )?;
        let foreground = try_create_resource(
            &path,
            "foreground",
            NovelLoadError::NoForegroundDirectory,
        )?;
        let script = try_create_resource(
            &path,
            "script",
            NovelLoadError::NoScriptDirectory,
        )?;
        let sound = try_create_resource(
            &path,
            "sound",
            NovelLoadError::NoSoundDirectory,
        )?;

        let icon = NovelIcon {
            high: try_create_resource(
                &path,
                "icon-high.png",
                NovelLoadError::NoHighResolutionIcon,
            )?,
            low: try_create_resource(
                &path,
                "icon.png",
                NovelLoadError::NoLowResolutionIcon,
            )?,
            thumbnail: try_create_resource(
                &path,
                "thumbnail.png",
                NovelLoadError::NoThumbnail,
            )?,
        };

        Ok(Self {
            title: try_load_info(&path)?,
            device_resolution: try_load_img(path)?,
            resources: NovelResources {
                background,
                foreground,
                script,
                sound,
                icon,
            },
        })
    }
}

impl Novel {
    resource_delegates!(background, foreground, script, sound);

    pub fn try_load_script(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<Script, LoadScriptError> {
        fs::read_to_string(self.resources.script.join(path))?
            .parse_script()
            .map(Script::new)
            .map_err(|e| e.into())
    }
}
