use anchor_lang::prelude::error_code;

#[error_code]
pub enum TodoListError {
    #[msg("The title is too long")]
    TextTooLong,
}