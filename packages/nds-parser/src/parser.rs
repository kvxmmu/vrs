use {
    crate::{
        command::*,
        error::ParseError,
        text::Text,
    },
    std::str::FromStr,
};

enum OptionalResult<T, E> {
    Result(Result<T, E>),
    None,
}

pub trait ParseScript {
    type Error;
    type Output;

    fn parse_script(&self) -> Result<Self::Output, Self::Error>
    where
        Self: Sized;
}

impl<T, E> OptionalResult<T, E> {
    pub fn default_or_err(self, default: T) -> Result<T, E> {
        match self {
            Self::Result(r) => r,
            Self::None => Ok(default),
        }
    }
}

impl<T, E> From<Option<Result<T, E>>> for OptionalResult<T, E> {
    fn from(value: Option<Result<T, E>>) -> Self {
        if let Some(res) = value {
            Self::Result(res)
        } else {
            Self::None
        }
    }
}

impl<T: AsRef<str>> ParseScript for T {
    type Error = ParseError;
    type Output = Vec<Command>;

    fn parse_script(&self) -> Result<Self::Output, Self::Error>
    where
        Self: Sized,
    {
        let lines = self.as_ref().lines().map(|v| v.trim());
        try_collect_vec(lines.map(parse_command))
    }
}

fn parse_command(line: &str) -> Result<Command, ParseError> {
    if line == "fi" {
        return Ok(Command::EndIf);
    } else if line == "cleartext" {
        return Ok(Command::ClearText(ClearTextType::FillBottomScreen));
    }

    let (command, args) = split_once_required(line)?;

    Ok(match command {
        "label" => Command::Label(args.to_owned()),
        "goto" => Command::Goto(args.to_owned()),

        "cleartext" => Command::ClearText(match args {
            "!" => ClearTextType::TextBufferInclHistory,

            t => {
                return Err(ParseError::UnknownClearTextType(t.to_owned()))
            }
        }),
        "random" => {
            let (variable, low_high) = split_once_required(args)?;
            let (low, high) = split_once_required(low_high)
                .map(|(l, h)| parse_u16_opt(l).zip(parse_u16_opt(h)))?
                .ok_or(ParseError::FailedToParseNumber)?;

            Command::Random {
                variable: variable.to_owned(),
                range: low..=high,
            }
        }
        "delay" => Command::Delay {
            frames: try_parse_u16(args)?,
        },

        "jump" => {
            let (file, label) = split_once_optional(args);

            Command::Jump {
                file: file.into(),
                label: label.map(ToOwned::to_owned),
            }
        }

        "if" => {
            let (variable, eq_and_val) = split_once_required(args)?;
            let (_, value) = split_once_required(eq_and_val)?;

            Command::If {
                name: variable.to_owned(),
                rhs: match value.parse() {
                    Ok(v) => IfRhs::Number(v),
                    _ => IfRhs::Variable(value.to_owned()),
                },
            }
        }
        cmd @ ("setvar" | "gsetvar") => {
            let (variable, modifier_and_value) =
                split_once_required(args)?;
            let (modifier, value) =
                split_once_required(modifier_and_value)?;

            Command::SetVar {
                name: variable.to_owned(),

                modifier: modifier.parse()?,
                accumulator: try_parse_u16(value)?,
                storage: if cmd == "gsetvar" {
                    VariableStorageType::Global
                } else {
                    VariableStorageType::Local
                },
            }
        }

        "choice" => Command::Choice {
            options: args
                .split('|')
                .map(|option| option.parse().unwrap())
                .collect(),
        },

        "text" => {
            let text = Text::from_str(args)?;
            Command::Text(text)
        }

        "music" => {
            let file = match args {
                "~" => MusicFile::StopPlaying,
                path => MusicFile::Path(path.into()),
            };

            Command::Music { file }
        }

        "sound" => {
            if args == "~" {
                Command::Sound(SoundLooping::StopCurrentlyPlaying)
            } else {
                let (file, looping) = split_once_required(args)?;
                let file = file.into();

                Command::Sound(match looping {
                    "-1" => SoundLooping::Infinite { file },
                    n => SoundLooping::Count {
                        file,
                        count: try_parse_u16(n)?,
                    },
                })
            }
        }

        "bgload" => {
            let (file, fadetime) = split_once_optional(args);
            let fadetime =
                try_parse_u16_opt(fadetime).default_or_err(16)?;

            Command::BgLoad {
                file: file.into(),
                fadetime,
            }
        }

        "setimg" => {
            let (file, coords_str) = split_once_required(args)?;
            let coordinates = split_once_required(coords_str)
                .map(|(x, y)| parse_u16_opt(x).zip(parse_u16_opt(y)))?
                .ok_or(ParseError::FailedToParseNumber)?;

            Command::SetImg {
                file: file.into(),
                coordinates,
            }
        }

        cmd => {
            return Err(ParseError::UnknownCommand {
                command: cmd.to_owned(),
            })
        }
    })
}

fn split_once_required(s: &str) -> Result<(&str, &str), ParseError> {
    s.split_once(' ')
        .ok_or(ParseError::NotEnoughArguments)
}

fn split_once_optional(s: &str) -> (&str, Option<&str>) {
    let (left, right) = s.split_at(
        match s.find(' ') {
            Some(pos) => pos,
            None => return (s, None),
        } + 1,
    );

    (&left[..left.len() - 1], Some(right))
}

fn try_collect_vec(
    commands: impl Iterator<Item = Result<Command, ParseError>>,
) -> Result<Vec<Command>, ParseError> {
    let mut vec = Vec::new();
    for command in commands {
        vec.push(command?);
    }

    Ok(vec)
}

fn parse_u16_opt(s: &str) -> Option<u16> {
    s.parse().ok()
}

fn try_parse_u16(s: &str) -> Result<u16, ParseError> {
    s.parse()
        .map_err(|_| ParseError::FailedToParseNumber)
}

fn try_parse_u16_opt(
    s_opt: Option<&str>,
) -> OptionalResult<u16, ParseError> {
    s_opt
        .map(|s| {
            s.parse::<u16>()
                .map_err(|_| ParseError::FailedToParseNumber)
        })
        .into()
}
