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

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum Part {
    bumper_and_fascia,
    body_panels,
    windshield_and_body_glass,
    liftgate,
    closure_assist,
    mechanisms_and_hinges,
    exterior_door_handles,
    door_glass_regulators,
    seals_body_closures,
    wheel_arch_liners,
    undertray_and_diffuser,
    badges_and_films,
    license_plate_mountings,
    exterior_mirrors,
    exterior_trim,
    underhood_trim,
    liftgate_through_trim,
    front_seat_tracks_and_motors,
    front_seat_assemblies_and_hardware,
    second_row_seat_assemblies_and_hardware,
    front_seat_covers_pads_trims,
    second_row_seat_covers_pads_trims,
    instrument_panel,
    interior_mirror_and_visors,
    trunk_trim,
    door_trim,
    pillar_and_sill_trim,
    center_console,
    headliner,
    luggage_compartment_trim,
    carpeting_and_mats,
    hv_battery_assembly,
    hv_battery_electrical_components,
    battery_and_fuses_twelve_volt,
    lv_battery,
    harnesses,
    electronic_control_modules,
    radar_sensors,
    front_camera,
    interior_camera,
    rear_camera,
    parking_sensors,
    exterior_lights,
    keyless_entry_and_security,
    wipers_and_washers,
    horn,
    radar_heater,
    cabin_hvac,
    refrigerant_system,
    cooling_system,
    thermal_system,
    labels,
    air_bags,
    seat_belts,
    pre_tensioners,
    sensors,
    displays,
    touchscreen,
    car_computer,
    audio_system_speakers_subwoofer_amplifier,
    audio_system_am_fm_hd_radio,
    gps,
    wifi,
    chassis_and_subframes,
    front_suspension,
    rear_suspension,
    air_suspension,
    steering_rack_and_lower_column,
    upper_column_and_steering_wheel,
    brake_discs_calipers,
    brake_pipes_hoses,
    abs_traction_stability_control,
    electromechanical_brake_booster,
    brake_pedal,
    wheels,
    tires,
    front_drive_unit_assembly,
    front_drive_inverter,
    front_drive_unit_inverter,
    front_gearbox_and_halfshafts,
    rear_drive_unit_assembly,
    rear_drive_inverter,
    rear_drive_gearbox_and_halfshafts,
    charge_system_inlet,
    hv_harnesses,
    mobile_connector,
    wall_connector,
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Machine {
    // machine level maintneance includes all parts
    last_maintained: UnixTimestamp,
    scheduled_maintain: UnixTimestamp,
    parts: Vec<MachinePart>,
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MachinePart {
    // part level maintainance incluedes only that part which is reffered to
    kind: Part,
    last_maintained: UnixTimestamp,
    measure_of_wear: u64,

}

// impl MachinePart {
//     pub fn set_
// }


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


