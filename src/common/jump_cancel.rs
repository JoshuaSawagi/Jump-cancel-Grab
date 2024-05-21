use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use crate::common::jump_cancel::ControlModule::check_button_on;
use smash::lua2cpp::L2CFighterCommon_status_JumpSquat_common;

// Jump Cancel Grab
#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat_common)]
unsafe extern "C" fn jump_cancel(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    unsafe {
        let status = StatusModule::status_kind(fighter.module_accessor);
        if [
        *FIGHTER_STATUS_KIND_JUMP_SQUAT  
        ].contains(&status) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH){
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), false.into());
            }
        } 
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            jump_cancel,
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}