use {
    crate::error::JumpToLabelError,
    nds_parser::command::Command,
    std::collections::BTreeMap,
};

#[derive(Debug, Clone)]
pub enum ScriptControlFlow<'a> {
    /// Execute specified command
    Execute(&'a Command),

    /// Execution is stopped
    Stopped,
}

#[derive(Debug, Clone)]
pub struct Script {
    /// Labels for fast lookup (goto & jumps)
    labels: BTreeMap<String, usize>,

    /// Script commands
    commands: Vec<Command>,

    /// Current script position
    cursor: usize,
}

impl Script {
    /// Get current command
    pub fn peek(&self) -> ScriptControlFlow<'_> {
        self.peek_from(self.cursor)
    }

    pub fn peek_from(&self, from: usize) -> ScriptControlFlow<'_> {
        if let Some(command) = self.commands.get(from) {
            ScriptControlFlow::Execute(command)
        } else {
            ScriptControlFlow::Stopped
        }
    }

    /// Advance cursor position by `acc` and return previous
    /// position
    pub fn advance_by(&mut self, acc: usize) -> usize {
        let prev = self.cursor;
        self.cursor += acc;
        prev
    }

    /// Fetch current command and advance cursor position by
    /// `1`
    pub fn next_command(&mut self) -> ScriptControlFlow<'_> {
        let prev_position = self.advance_by(1);
        let cf = self.peek_from(prev_position);
        cf
    }
}

impl Script {
    /// Jump to specific label inside the script
    pub fn jump_to_label(
        &mut self,
        label: &str,
    ) -> Result<(), JumpToLabelError> {
        if let Some(&cursor) = self.labels.get(label) {
            self.adjust_cursor_to(cursor);

            Ok(())
        } else {
            Err(JumpToLabelError::LabelNotFound)
        }
    }

    /// Get cursor position
    pub const fn cursor(&self) -> usize {
        self.cursor
    }

    /// Set cursor
    pub fn adjust_cursor_to(&mut self, cursor: usize) {
        self.cursor = cursor;
    }
}

impl Script {
    /// # Safety
    ///
    /// since we are not checking the `labels` index
    /// correctness this is considered as unsafe operation.
    /// if `labels` has incorrect labels mapping then
    /// correctness is not guaranteed
    pub unsafe fn with_labels(
        commands: Vec<Command>,
        labels: BTreeMap<String, usize>,
    ) -> Self {
        Self {
            labels,
            commands,

            cursor: 0,
        }
    }

    pub fn new(commands: Vec<Command>) -> Self {
        Self {
            labels: Self::lookup_labels(&commands),

            commands,
            cursor: 0,
        }
    }

    fn lookup_labels(commands: &[Command]) -> BTreeMap<String, usize> {
        commands
            .iter()
            .enumerate()
            .filter(|(_index, command)| {
                matches!(command, Command::Label(..))
            })
            .map(|(index, command)| match command {
                Command::Label(l) => (l.clone(), index),

                // SAFETY: this is safe since we checked Command::Label
                // pattern above
                _ => unsafe { std::hint::unreachable_unchecked() },
            })
            .collect()
    }
}
