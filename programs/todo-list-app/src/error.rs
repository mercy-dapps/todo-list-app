use anchor_lang::prelude::*;

#[error_code]
pub enum TodoError {
    #[msg("Todo title too long")]
    TitleTooLong,
    #[msg("Unauthorized access.")]
    Unauthorized,
}