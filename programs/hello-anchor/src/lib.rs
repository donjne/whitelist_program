use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata},
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::types::DataV2;

declare_id!("6WYUA2DPb3k2cB1eS12ZMLWZuGuG9WdZA9k3e6rGoenU");

#[program]
mod hello_anchor {
    use super::*;

    pub fn init_whitelist(ctx: Context<InitWhitelist>, token_price: u64) -> Result<()> {
        let whitelist_program = &mut ctx.accounts.whitelist_program;
        
        // Initialize the fields
        whitelist_program.whitelist_addresses = Vec::new();  // Start with an empty whitelist
        whitelist_program.is_whitelist_finalized = false;  // Initially not finalized
        whitelist_program.is_sale_active = false;  // Initially sale is not active
        whitelist_program.token_price = token_price;  // Set token price
        whitelist_program.max_tokens_per_address = 0;  // Set max tokens per address to 0 initially
        whitelist_program.total_tokens_sold = 0;  // Initially, no tokens sold
        whitelist_program.token_balance_keys = Vec::new();  // Initialize empty token balance keys
        whitelist_program.token_balance_values = Vec::new();  // Initialize empty token balance values
    
        Ok(())
    }

    pub fn init_token(ctx: Context<InitToken>, metadata: InitTokenParams) -> Result<()> {
        let token_data: DataV2 = DataV2 {
            name: metadata.name,
            symbol: metadata.symbol,
            uri: metadata.uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        let seeds = &["mint".as_bytes(), &[ctx.bumps.mint]];
        let signer = [&seeds[..]];

        let metadata_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                payer: ctx.accounts.payer.to_account_info(),
                update_authority: ctx.accounts.mint.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                metadata: ctx.accounts.metadata.to_account_info(),
                mint_authority: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &signer,
        );

        create_metadata_accounts_v3(metadata_ctx, token_data, false, true, None)?;

        msg!("Token mint created successfully.");

        Ok(())
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, quantity: u64) -> Result<()> {
        let seeds = &["mint".as_bytes(), &[ctx.bumps.mint]];
        let signer = [&seeds[..]];

        let whitelist_program = &mut ctx.accounts.whitelist_program;

        if !whitelist_program.is_sale_active {
            return Err(ErrorCodes::SaleNotActive.into());
        }

        if !whitelist_program.whitelist_addresses.contains(&ctx.accounts.destination.key()) {
            return Err(ErrorCodes::NotWhitelisted.into());
        }

        let mut current_balance = 0;
        for (i, key) in whitelist_program.token_balance_keys.iter().enumerate() {
            if key == &ctx.accounts.destination.key() {
                current_balance = whitelist_program.token_balance_values[i];
                break;
            }
        }

        if current_balance + quantity > whitelist_program.max_tokens_per_address {
            return Err(ErrorCodes::MaxTokensExceeded.into());
        }

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.destination.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
                &signer,
            ),
            quantity,
        )?;

        let mut updated = false;
        for (i, key) in whitelist_program.token_balance_keys.iter().enumerate() {
            if key == &ctx.accounts.destination.key() {
                whitelist_program.token_balance_values[i] += quantity;
                updated = true;
                break;
            }
        }

        if !updated {
            whitelist_program.token_balance_keys.push(ctx.accounts.destination.key());
            whitelist_program.token_balance_values.push(current_balance + quantity);
        }

        Ok(())
    }
}

#[account]
pub struct WhitelistProgram {
    pub whitelist_addresses: Vec<Pubkey>,
    pub is_whitelist_finalized: bool,
    pub is_sale_active: bool,
    pub token_price: u64,
    pub max_tokens_per_address: u64,
    pub total_tokens_sold: u64,
    pub token_balance_keys: Vec<Pubkey>,
    pub token_balance_values: Vec<u64>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitTokenParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
}

#[derive(Accounts)]
pub struct InitWhitelist<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init, 
        payer = payer, 
        space = 8 + 1024,
        seeds = [b"whitelist"], bump
    )]
    pub whitelist_program: Account<'info, WhitelistProgram>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(params: InitTokenParams)]
pub struct InitToken<'info> {
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [b"mint"],
        bump,
        payer = payer,
        mint::decimals = params.decimals,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut, signer)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub whitelist_program: Account<'info, WhitelistProgram>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(
        mut,
        seeds = [b"mint"],
        bump,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub destination: Account<'info, TokenAccount>,
    #[account(mut, signer)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub whitelist_program: Account<'info, WhitelistProgram>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[error_code]
pub enum ErrorCodes {
    #[msg("Whitelist is already finalized.")]
    WhitelistFinalized,
    #[msg("Address is already whitelisted.")]
    AlreadyWhitelisted,
    #[msg("Address is not whitelisted.")]
    NotWhitelisted,
    #[msg("Token sale is not active.")]
    SaleNotActive,
    #[msg("Exceeded maximum tokens per address.")]
    MaxTokensExceeded,
}
