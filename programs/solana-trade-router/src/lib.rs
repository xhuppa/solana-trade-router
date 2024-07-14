use anchor_lang::prelude::*;
use serum_dex::state::Market;
use anchor_spl::token::{self, TokenAccount, Transfer, Token};

#[cfg(not(feature = "no_jemalloc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;
declare_id!("7MBTsqiPjHHt5Xmaq8jjTq93r5msdVy8rxEdyGgAnV3v");

#[program]
pub mod solana_trade_router {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initializing Solana Trade Router program...");
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn route_trade(ctx: Context<RouteTrade>, amount: u64) -> Result<()> {
        msg!("Routing trade...");

        let market_account_info = &ctx.accounts.market;
        let user_account_info = &ctx.accounts.user;
        let user_token_account = &ctx.accounts.user_token_account;
        let program_id = ctx.program_id;

        let market = Market::load(market_account_info, program_id, false).map_err(|e| {
            msg!("Error loading market: {:?}", e);
            ProgramError::Custom(0)
        })?;

        let own_address = market.own_address;
        let vault_signer_nonce = market.vault_signer_nonce;

        msg!("Market own address: {:?}", own_address);
        msg!("Market vault signer nonce: {:?}", vault_signer_nonce);

        // Check users token balance
        let user_balance = user_token_account.amount;
        if user_balance < amount {
            msg!("Insufficient balance in user token account: {:?}", user_balance);
            return Err(ProgramError::InsufficientFunds.into());
        }

        // Transfer tokens to market's vault
        let cpi_accounts = Transfer {
            from: user_token_account.to_account_info(),
            to: market.vault.to_account_info(),
            authority: user_account_info.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        msg!("Successfully routed trade for amount: {:?}", amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct RouteTrade<'info> {
    #[account(mut)]
    pub market: AccountInfo<'info>,
    #[account(signer)]
    pub user: AccountInfo<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub market_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
