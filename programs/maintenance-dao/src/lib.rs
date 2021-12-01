use anchor_lang::{prelude::*, solana_program::clock::{UnixTimestamp, Epoch}};
use anchor_lang::{solana_program::{log, slot_history}};
pub mod account;
use crate::account::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod maintenance_dao {
    use super::*;
    pub fn initialize(ctx: Context<MaintenanceContext>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        Ok(())
    }


    // // maintain context
    // pub fn run_maintenance_all() {}
    // pub fn run_maintenance_single() {}
    // pub fn run_maintenance_choice() {}
    // pub fn schedule_maintenance() {}
    // pub fn schedule_maintenance() {}

    // // part context
    // pub fn add_part() {}
    // pub fn remove_part() {}

    // // employee context
    // pub fn change_maintainer() {}
    // pub fn change_operator() {}

}

#[derive(Accounts)]
pub struct MaintenanceContext<'info> {
    #[account(init, payer = user, space = 10000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct PartContext<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct EmployeeContext<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ContractContext<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}