use {
    crate::error::TextParseError,
    std::str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Foreground {
    /// Actually greish
    Black = 30,

    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Purple = 35,
    Cyan = 36,
    White = 37,

    Regular = 39,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextType {
    Plain(String),
    Variable(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextSpan {
    pub text: TextType,
    pub color: Foreground,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Text {
    Spans {
        spans: Vec<TextSpan>,
        click_to_advance: bool,
    },
    BlankLine {
        click_to_advance: bool,
    },
}

// Start: \x1b[<N>;1m
// Set to regular color: \x1b[0m

macro_rules! define_parsers {
    ($(
        let $name:ident $iterator:ident $last:ident $color:ident = $expr:expr
    )*) => {
        $(
            #[allow(clippy::ptr_arg)]
            fn $name<I>(
                $iterator: &mut I,
                $last: &mut String,
                $color: &mut Foreground,
            ) -> Result<Option<TextSpan>, TextParseError>
            where I: Iterator<Item = char>
            {
                $expr
            }
        )*
    };
}

define_parsers! {
    let parse_variable it __ current_color = {
        fn can_be_used(c: char) -> bool {
            !matches!(
                c,
                ' '  |
                '$'  |
                '{'  |
                '}'  |
                ';'  |
                ','  |
                '.'  |
                '='  |
                '('  |
                ')'  |
                '\\' |
                '/'
            )
        }

        let name: String = it.take_while(|&c| can_be_used(c)).collect();
        if name.is_empty() {
            Err(TextParseError::EmptyVariableName)
        } else {
            Ok(TextSpan {
                text: TextType::Variable(name),
                color: *current_color,
            }.into())
        }
    }

    let parse_color it last current_color = {
        let until_bracket: String = it.take_while(|&c| c != '[').collect();
        if until_bracket != "x1b" {
            last.push('\\');
            last.push_str(&until_bracket);
            last.push('[');
            return Ok(None);
        }

        let number_str: String = it.take(2).collect();

        if number_str == "0m" {
            *current_color = Foreground::Regular;
        } else {
            // skip 3 characters,
            // I don't care about correctness for the remaining part O_o
            let _ = it.nth(2);

            *current_color = Foreground::try_from(
                number_str.parse::<u16>()
                          .map_err(|_| TextParseError::InvalidColor)?
            )?;
        }

        //*current_color = Foreground::try_from(color_number)?;

        Ok(None)
    }

    let parse_braced_variable it last current_color = {
        let cchar = it.next();
        if !matches!(cchar, Some('$')) {
            last.push(cchar.unwrap());
            return Ok(None);
        }

        Ok(TextSpan {
            text: TextType::Variable(it.take_while(|c| *c != '}').collect()),
            color: *current_color,
        }.into())
    }
}

impl FromStr for Text {
    type Err = TextParseError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        match s {
            "~" => {
                return Ok(Text::BlankLine {
                    click_to_advance: false,
                })
            }
            "!" => {
                return Ok(Text::BlankLine {
                    click_to_advance: true,
                })
            }

            _ => {}
        };
        let click_to_advance = if s.starts_with('@') {
            s = &s[1..];
            false
        } else {
            true
        };

        let mut spans = Vec::new();
        let mut last = String::new();
        let mut chars = s.chars();
        let mut current_color = Foreground::Regular;

        while let Some(chr) = chars.next() {
            let span_opt = match chr {
                '$' => parse_variable(
                    &mut chars,
                    &mut last,
                    &mut current_color,
                )?,
                '{' => parse_braced_variable(
                    &mut chars,
                    &mut last,
                    &mut current_color,
                )?,
                '\\' => {
                    let prev = current_color;
                    let color = parse_color(
                        &mut chars,
                        &mut last,
                        &mut current_color,
                    );
                    if color.is_ok() && !last.is_empty() {
                        spans.push(TextSpan {
                            text: TextType::Plain(last.clone()),
                            color: prev,
                        });
                        last.clear();

                        None
                    } else {
                        color?
                    }
                }

                _ => {
                    last.push(chr);
                    continue;
                }
            };

            if let Some(span) = span_opt {
                if !last.is_empty() {
                    spans.push(TextSpan {
                        text: TextType::Plain(last.clone()),
                        color: current_color,
                    });
                    last.clear();
                }

                spans.push(span);
            }
        }

        if !last.is_empty() {
            spans.push(TextSpan {
                text: TextType::Plain(last),
                color: current_color,
            });
        }

        Ok(Self::Spans {
            spans,
            click_to_advance,
        })
    }
}

impl TryFrom<u16> for Foreground {
    type Error = TextParseError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if matches!(value, 30..=37 | 39) {
            union U {
                i: u16,
                e: Foreground,
            }

            // SAFETY: this is safe since `Foreground`
            // is declared as repr(u16) and integer range
            // we already checked above
            Ok(unsafe { U { i: value }.e })
        } else {
            Err(TextParseError::InvalidColor)
        }
    }
}
