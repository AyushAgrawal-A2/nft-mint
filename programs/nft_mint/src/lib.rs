pub mod constants;
pub mod instructions;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;

declare_id!("JCy7VmaXAS28emJPFNTr8qjvT7BrXXnAs26his91vnhX");

#[program]
pub mod nft_mint {
    use super::*;

    pub fn create_nft(
        ctx: Context<CreateNft>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        crate::instructions::create_nft::handle_create_nft(ctx, name, symbol, uri)
    }

    pub fn create_nft_collection(
        ctx: Context<CreateNftCollection>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        crate::instructions::create_nft_collection::handle_create_nft_collection(
            ctx, name, symbol, uri,
        )
    }

    pub fn verify_nft_collection(ctx: Context<VerifyNftCollection>) -> Result<()> {
        crate::instructions::verify_nft_collection::handle_verify_nft_collection(ctx)
    }
}
