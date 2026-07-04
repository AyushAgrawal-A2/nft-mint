use anchor_lang::prelude::*;

#[error_code]
pub enum NftMintError {
    #[msg("Only the counter authority can update this counter")]
    Unauthorized,
}
