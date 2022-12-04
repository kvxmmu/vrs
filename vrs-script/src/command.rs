use {
    crate::text::{
        Text,
        TextSpan,
    },
    std::ops::Range,
};

pub type ScriptSpan = Range<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopingCount {
    Infinite,
    Stop,
    Times(u16),
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

#[derive(Debug, Clone)]
pub enum RhsIfItem {
    Variable(ScriptSpan),
    Number(u16),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetVariable {
    pub variable: ScriptSpan,
    pub modifier: VariableModifier,
}

#[derive(Debug)]
pub enum Command {
    /// Load background image with fadetime.
    /// Default fadetime is 16
    BgLoad { fadetime: u16 },

    /// Place/shift image on the screen starting from upper
    /// left
    SetImg {
        path: ScriptSpan,
        x_shift: u16,
        y_shift: u16,
    },

    /// Play `file` `looping` times
    Sound {
        file: ScriptSpan,
        looping: LoopingCount,
    },

    /// Plays & loops music. Stops any currently playing
    /// music if file is None
    Music { file: Option<ScriptSpan> },

    /// Displays text to the lower screen
    Text(Text),

    /// Displays choices on the lower screen
    Choice { options: Vec<Vec<TextSpan>> },

    /// Sets variable into the local save memory.
    SetVar(VariableModifier),

    /// Sets variable into the global save memory
    GSetVar(SetVariable),

    /// If condition is true, keep reading, else - skip
    If {
        variable: ScriptSpan,
        then: Vec<Command>,
    },

    /// Jump to defined .scr and start reading from label if
    /// specified
    Jump {
        file: ScriptSpan,
        label: Option<ScriptSpan>,
    },

    /// Pause actions for X frames (audio still plays
    /// normally)
    Delay { frames: u16 },

    /// Sets variable `variable` to a number in `range`
    Random {
        variable: ScriptSpan,
        range: Range<u16>,
    },

    /// Create field within the script that can be jumped to
    Label { name: ScriptSpan },

    /// Jumps to the label in the same file
    Goto { label: ScriptSpan },

    /// Works same as `text ~` if clear_history == false,
    /// otherwise completely clears the text buffer
    /// (including history)
    ClearText { clear_history: bool },
}
