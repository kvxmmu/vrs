use thiserror::Error;

#[derive(Debug, Error)]
pub enum TextParseError {
    #[error("Specified empty variable name in text")]
    EmptyVariableName,

    #[error("Invalid text color specified")]
    InvalidColor,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("No arguments in commands")]
    NoArguments,

    #[error("Unknown command: {command}")]
    UnknownCommand { command: String },

    #[error("Failed to parse number in arguments")]
    FailedToParseNumber,

    #[error("Not enough arguments to command")]
    NotEnoughArguments,

    #[error("Unknown variable modifier found: {modifier}")]
    UnknownVariableModifier { modifier: String },

    #[error("Random low ({low}) value is higher than high ({high})")]
    RandomLowIsHigherThanHigh { low: u16, high: u16 },

    #[error(
        "Unknown clear text type specified to cleartext command: {0}"
    )]
    UnknownClearTextType(String),

    #[error("Failed to parse text: {0}")]
    Text(TextParseError),
}
