use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum JumpToLabelError {
    #[error("Specified label was not found")]
    LabelNotFound,
}
