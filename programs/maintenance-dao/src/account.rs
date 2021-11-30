use crate::*;
use anchor_lang::prelude::*;

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
    maintenance_entity: MaintenanceEntity,
    maintainer: Maintainer,
    time: UnixTimestamp, 
    all_parts: bool,
    parts_list: Vec<MachinePart>,

}


#[account]
pub struct BaseAccount {
    pub owner: Pubkey,

    parts: Vec<MachinePart>,
    machines: Vec<Machine<MachinePart>>,
    maintainers: Vec<Maintainer>,
    operators: Vec<Operator>,

}