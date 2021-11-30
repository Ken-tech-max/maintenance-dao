use anchor_lang::{prelude::*, solana_program::clock::{UnixTimestamp, Epoch}};
use anchor_lang::{solana_program::{log, slot_history}};
pub mod account;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod maintenance_dao {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }

    // pub fn create_contract() {}
    //    // ContractContext 

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

// turn this into a pda eventually
#[state(10000)]
pub struct Source {
    ownership_account: BaseAccount,
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

#[error]
pub enum MaintenanceError {
    #[msg("")]
    Maintenance,
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct Machine<MachinePart> {
    components: MachinePart


}

#[account]
pub struct MachinePart {

}

#[account]
pub struct Operator {

}

#[account]
pub struct Maintainer {

}

#[account]
pub struct MaintenanceEntity {
    employees: Box<Vec<Maintainer>>,
    contracts: MaintenanceContract,

}

#[account]
pub struct MaintenanceContract {
    maintenance_entity: Box<MaintenanceEntity>,
    maintainer: Box<Maintainer>,
    time: Box<UnixTimestamp>,
    all_parts: bool,
    parts_list: Box<Vec<MachinePart>>,

}


#[account]
pub struct BaseAccount {
    pub owner: Pubkey,

    parts: Vec<MachinePart>,
    machines: Vec<Machine<MachinePart>>,
    maintainers: Vec<Maintainer>,
    operators: Vec<Operator>,

}

