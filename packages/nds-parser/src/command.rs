use {
    crate::{
        error::ParseError,
        text::Text,
    },
    std::{
        convert::Infallible,
        ops::RangeInclusive,
        path::PathBuf,
        str::FromStr,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClearTextType {
    FillBottomScreen,
    TextBufferInclHistory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableStorageType {
    /// Common variable for the all saves
    Global,

    /// Variable specific for concrete save
    Local,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IfRhs {
    Number(u16),
    Variable(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableModifier {
    Assign,
    Add,
    Sub,
    GtOrEq,
    LtOrEq,
    Gt,
    Lt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChoiceOption {
    Variable(String),
    Option(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundLooping {
    Infinite,
    StopCurrentlyPlaying,
    Count(u16),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MusicFile {
    StopPlaying,
    Path(PathBuf),
}

#[derive(Debug, Clone)]
pub enum Command {
    BgLoad {
        file: PathBuf,
        fadetime: u16,
    },
    SetImg {
        file: PathBuf,
        coordinates: (u16, u16),
    },

    Sound {
        file: PathBuf,
        looping: SoundLooping,
    },
    Music {
        file: MusicFile,
    },

    Choice {
        options: Vec<ChoiceOption>,
    },

    SetVar {
        name: String,

        accumulator: u16,
        modifier: VariableModifier,
        storage: VariableStorageType,
    },

    If {
        name: String,
        rhs: IfRhs,
    },

    Jump {
        file: PathBuf,
        label: Option<String>,
    },
    Delay {
        frames: u16,
    },

    Random {
        variable: String,
        range: RangeInclusive<u16>,
    },

    Text(Text),

    Label(String),
    Goto(String),
    ClearText(ClearTextType),

    EndIf,
}

impl FromStr for VariableModifier {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "=" => Self::Assign,
            "+" => Self::Add,
            "-" => Self::Sub,
            ">" => Self::Gt,
            "<" => Self::Lt,
            ">=" => Self::GtOrEq,
            "<=" => Self::LtOrEq,

            o => {
                return Err(ParseError::UnknownVariableModifier {
                    modifier: o.to_owned(),
                })
            }
        })
    }
}

impl FromStr for ChoiceOption {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some(name) = s.strip_prefix('$') {
            Self::Variable(name.to_owned())
        } else {
            Self::Option(s.to_owned())
        })
    }
}
