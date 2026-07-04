use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{verify_sized_collection_item, Metadata, VerifySizedCollectionItem},
    token::Mint,
};

use crate::{EDITION_SEED, METADATA_SEED};

#[derive(Accounts)]
pub struct VerifyNftCollection<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub collection_authority: Signer<'info>,

    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds =[METADATA_SEED, token_metadata_program.key().as_ref(), nft_mint.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: validated via pda seeds
    pub metadata: UncheckedAccount<'info>,

    pub collection_mint: Account<'info, Mint>,

    #[account(
        seeds =[METADATA_SEED, token_metadata_program.key().as_ref(), collection_mint.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: validated via pda seeds
    pub collection_metadata: UncheckedAccount<'info>,

    #[account(
        seeds =[METADATA_SEED, token_metadata_program.key().as_ref(), collection_mint.key().as_ref(), EDITION_SEED],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: validated via pda seeds
    pub collection_edition: UncheckedAccount<'info>,

    pub token_metadata_program: Program<'info, Metadata>,
}

pub fn handle_verify_nft_collection(ctx: Context<VerifyNftCollection>) -> Result<()> {
    verify_sized_collection_item(
        CpiContext::new(
            ctx.accounts.token_metadata_program.key(),
            VerifySizedCollectionItem {
                payer: ctx.accounts.payer.to_account_info(),
                metadata: ctx.accounts.metadata.to_account_info(),
                collection_authority: ctx.accounts.collection_authority.to_account_info(),
                collection_mint: ctx.accounts.collection_mint.to_account_info(),
                collection_metadata: ctx.accounts.collection_mint.to_account_info(),
                collection_master_edition: ctx.accounts.collection_edition.to_account_info(),
            },
        ),
        None,
    )?;
    Ok(())
}
