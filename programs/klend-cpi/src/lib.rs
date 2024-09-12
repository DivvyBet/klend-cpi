#![allow(unused)]

use anchor_lang::prelude::*;

#[cfg(feature = "staging")]
declare_id!("SLendK7ySfcEzyaFqy93gDnD3RtrpXJcnRwb6zFHJSh");

#[cfg(not(feature = "staging"))]
declare_id!("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD");

#[program]
pub mod kamino_lending {
    use super::*;

    pub fn deposit_reserve_liquidity(
        ctx: Context<DepositReserveLiquidity>,
        liquidity_amount: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn redeem_reserve_collateral(
        ctx: Context<RedeemReserveCollateral>,
        collateral_amount: u64,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DepositReserveLiquidity<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    #[account(mut)]
    pub reserve: AccountInfo<'info>,

    pub lending_market: AccountInfo<'info>,
    #[account()]
    pub lending_market_authority: AccountInfo<'info>,

    #[account(mut)]
    pub reserve_liquidity_mint: AccountInfo<'info>,

    #[account(mut)]
    pub reserve_liquidity_supply: AccountInfo<'info>,

    #[account(mut)]
    pub reserve_collateral_mint: AccountInfo<'info>,

    #[account(mut)]
    pub user_source_liquidity: AccountInfo<'info>,
    #[account(mut)]
    pub user_destination_collateral: AccountInfo<'info>,

    pub collateral_token_program: AccountInfo<'info>,
    pub liquidity_token_program: AccountInfo<'info>,

    pub instruction_sysvar_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RedeemReserveCollateral<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    pub lending_market: AccountInfo<'info>,

    #[account(mut)]
    pub reserve: AccountInfo<'info>,
    pub lending_market_authority: AccountInfo<'info>,

    #[account(mut)]
    pub reserve_liquidity_mint: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_collateral_mint: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_liquidity_supply: AccountInfo<'info>,

    #[account(mut)]
    pub user_source_collateral: AccountInfo<'info>,
    #[account(mut)]
    pub user_destination_liquidity: AccountInfo<'info>,

    pub collateral_token_program: AccountInfo<'info>,
    pub liquidity_token_program: AccountInfo<'info>,

    pub instruction_sysvar_account: AccountInfo<'info>,
}
