use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::{invoke, invoke_signed},
    system_instruction::{create_account, transfer},
    sysvar::rent::Rent,
};
use anchor_spl::{
    associated_token::{self, AssociatedToken, Create},
    token::{self, Mint, MintTo, Token},
};
use mpl_token_metadata::instruction::create_master_edition_v3;
use mpl_token_metadata::instruction::create_metadata_accounts_v2;
use mpl_token_metadata::instruction::update_metadata_accounts_v2;
use mpl_token_metadata::state::Collection;
use mpl_token_metadata::state::Creator;
use mpl_token_metadata::state::Uses;

declare_id!("5edrrYVdopsZAywpM65QfTDcSy9UKRJDV7Xg3iDSJyjc");

#[program]
pub mod mpl_testing {
    use super::*;

    pub fn create_nft(ctx: Context<CreateMaster>) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.metadata_mint.clone(),
            to: ctx.accounts.reciever_account.clone(),
            authority: ctx.accounts.update_auth.clone(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, 1)?;

        msg!("TOKENMINTED");

        let ix = create_master_edition_v3(
            *ctx.accounts.mpl_program.key,
            *ctx.accounts.master_edition.key,
            *ctx.accounts.metadata_mint.key,
            *ctx.accounts.update_auth.key,
            *ctx.accounts.update_auth.key,
            *ctx.accounts.metadata_account.key,
            *ctx.accounts.update_auth.key,
            Some(0),
        );

        invoke(
            &ix,
            &[
                ctx.accounts.master_edition.to_account_info(),
                ctx.accounts.metadata_mint.to_account_info(),
                ctx.accounts.update_auth.to_account_info(),
                ctx.accounts.update_auth.to_account_info(),
                ctx.accounts.metadata_account.to_account_info(),
                ctx.accounts.mpl_program.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;
        Ok(())
    }

    pub fn create_meta_data(
        ctx: Context<MetaDataAccount>,
        name: String,
        symbol: String,
        uri: String,
        seller_fee_basis_points: u16,
        is_mutable: bool,
    ) -> Result<()> {
        let creators = vec![Creator {
            address: *ctx.accounts.payer.key,
            verified: false,
            share: 100,
        }];

        let ix = create_metadata_accounts_v2(
            *ctx.accounts.mpl_program.key,
            *ctx.accounts.metadata.key,
            *ctx.accounts.mint.key,
            *ctx.accounts.payer.key,
            *ctx.accounts.payer.key,
            *ctx.accounts.payer.key,
            name,
            symbol,
            uri,
            Some(creators),
            seller_fee_basis_points,
            true,
            is_mutable,
            None,
            None,
        );

        invoke(
            &ix,
            &[
                ctx.accounts.metadata.clone(),
                ctx.accounts.mint.clone(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.mpl_program.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MetaDataAccount<'info> {
    #[account(address = mpl_token_metadata::id())]
    pub mpl_program: AccountInfo<'info>,
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateMaster<'info> {
    #[account(mut)]
    pub master_edition: AccountInfo<'info>,
    #[account(mut)]
    pub metadata_mint: AccountInfo<'info>,
    #[account(mut)]
    pub update_auth: AccountInfo<'info>,
    #[account(address = mpl_token_metadata::id())]
    pub mpl_program: AccountInfo<'info>,
    #[account(mut)]
    pub metadata_account: AccountInfo<'info>,
    #[account(mut)]
    pub reciever_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
