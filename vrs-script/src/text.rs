use crate::command::ScriptSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextForeground {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    Default,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextSpan {
    pub color: TextForeground,
    pub span: ScriptSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Text {
    Plain { spans: Vec<TextSpan> },
    ShowClickToAdvance { spans: Vec<TextSpan> },

    BlankLine { click_to_advance: bool },
}
