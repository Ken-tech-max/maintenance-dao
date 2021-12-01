use crate::*;
use anchor_lang::prelude::*;
use anchor_lang::{prelude::*, solana_program::clock::{UnixTimestamp, Epoch}};
use anchor_lang::{solana_program::{log, slot_history}};



#[zero_copy]
pub struct State {
    // this is where we define
    // what parts exist
    // waht mainteainers exist
    // waht the company is
    // schedule for automatic maintenance // needs oracle with time to invoke
    // 
}

// impl Default for State {

// }

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Machine {
    // machine level maintneance includes all parts
    last_maintained: UnixTimestamp,
    scheduled_maintain: UnixTimestamp,
    parts: MachinePart,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MachinePart {
    // part level maintainance incluedes only that part which is reffered to
    last_maintained: UnixTimestamp,
    measure_of_wear: u64,
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Maintainer {
    skills: Vec<Skill>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Skill {
    pertaining_to: MachinePart,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MaintenanceEntity {
    // contracts: [MaintenanceContract<UnixTimestamp, MachinePart>],
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MaintenanceContract {
    date: UnixTimestamp,
    completed: bool,
}


#[account]
pub struct BaseAccount {
    pub owner: Pubkey,

    parts: Vec<MachinePart>,
    machines: Vec<Machine>,
    maintainers: Vec<Maintainer>,

}


