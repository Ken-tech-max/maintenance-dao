use anchor_lang::{prelude::*, solana_program::clock::{UnixTimestamp, Epoch}};
use anchor_lang::{solana_program::{log, slot_history}};
pub mod account;
pub mod context;
pub mod error;
use crate::context::*;
use crate::account::*;
use crate::error::*;



declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod maintenance_dao {
    use super::*;
    pub fn create_factory(
        ctx: Context<MaintenanceContext>, 
        machines: Vec<Machine>,
        maintainers: Vec<Maintainer>,
        initial_maintenance: UnixTimestamp,
    ) -> ProgramResult {
        let base_account= &mut ctx.accounts.base_account;

        if initial_maintenance < Clock::default().unix_timestamp {
            // throw error
        }
       
        //
        Ok(())
}

    // pub fn run_maintenance_all()


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