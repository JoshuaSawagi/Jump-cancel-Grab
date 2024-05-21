#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros,
    clippy::borrow_interior_mutable_const
)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(non_upper_case_globals)]

use std::sync::atomic::{AtomicBool, Ordering};

use skyline::hooks::InlineCtx;

pub mod globals {
    pub const UNK1: i32 = 0x0; //void value
    pub const UNK2: i32 = 0x1; //void value
    pub const FIGHTER_KIND: i32 = 0x2; //fighter kind
    pub const OBJECT_ID: i32 = 0x3; //object id
    pub const UNK3: i32 = 0x4; //ptr value, very similar to 0x6
    pub const MODULE_ACCESSOR: i32 = 0x5; //module accessor
    pub const UNK4: i32 = 0x6; //void value
    pub const INIT_STATUS_FUNC: i32 = 0x7; //init status func
    pub const IS_STOP: i32 = 0x8; //is stop
    pub const STATUS_KIND_INTERRUPT: i32 = 0x9; //status kind interrupt
    pub const PREV_STATUS_KIND: i32 = 0xA; //prev status kind
    pub const STATUS_KIND: i32 = 0xB; //status kind
    pub const STATUS_COUNT: i32 = 0xC; //status count
    pub const UNK5: i32 = 0xD; //bool value
    pub const CURRENT_FRAME: i32 = 0xE; //current frame
    pub const CURRENT_FRAME_NO_INTERP: i32 = 0xF; //current frame no interp
    pub const UNK6: i32 = 0x10; //ptr value
    pub const UNK7: i32 = 0x11; //ptr value, equal to UNK8
    pub const UNK8: i32 = 0x12; //ptr value
    pub const SUB_STATUS3: i32 = 0x13; //sub status3
    pub const PREV_SUB_STATUS: i32 = 0x14; //prev sub status
    pub const SUB_STATUS: i32 = 0x15; //sub status
    pub const SITUATION_KIND: i32 = 0x16; //situation kind
    pub const PREV_SITUATION_KIND: i32 = 0x17; //prev situation kind
    pub const PREV_STATUS_FRAME: i32 = 0x18; //prev status frame
    pub const UNK9: i32 = 0x19; //i32 value
    pub const STICK_X: i32 = 0x1A; //stick x
    pub const STICK_Y: i32 = 0x1B; //stick y
    pub const FLICK_X: i32 = 0x1C; //flick x
    pub const FLICK_Y: i32 = 0x1D; //flick y
    pub const FLICK_Y_DIR: i32 = 0x1E; //flick y dir
    pub const PAD_FLAG: i32 = 0x1F; //pad flag
    pub const CMD_CAT1: i32 = 0x20; //cmd cat1
    pub const CMD_CAT2: i32 = 0x21; //cmd cat2
    pub const CMD_CAT3: i32 = 0x22; //cmd cat3
    pub const CMD_CAT4: i32 = 0x23; //cmd cat4
    pub const UNK10: i32 = 0x24;
    pub const UNK11: i32 = 0x25;
    pub const CHECK_AIR_SPECIAL_UNIQ: i32 = 0x26; //check air special uniq
    pub const CHECK_GROUND_SPECIAL_UNIQ: i32 = 0x27; //check ground special uniq
    pub const CHECK_GROUND_ATTACK_UNIQ: i32 = 0x28; //check ground attack uniq
    pub const DASH_COMMON_UNIQ: i32 = 0x29; //dash common uniq
    pub const RUN_MAIN_UNIQ: i32 = 0x2A; //run main uniq
    pub const JUMP_SQUAT_MAIN_UNIQ: i32 = 0x2B; //jump squat main uniq
    pub const CHECK_AIR_LANDING_UNIQ: i32 = 0x2C; //check air landing uniq
    pub const CHECK_AIR_ITEM_THROW_UNIQ: i32 = 0x2D; //check air item throw uniq
    pub const CHECK_AIR_ATTACK_UNIQ: i32 = 0x2E; //check air attack uniq
    pub const CHECK_AIR_ESCAPE_UNIQ: i32 = 0x2F; //check air escape uniq
    pub const CHECK_AIR_TREAD_JUMP_UNIQ: i32 = 0x30; //check air tread jump uniq
    pub const CHECK_AIR_WALL_JUMP_UNIQ: i32 = 0x31; //check air wall jump uniq
    pub const CHECK_AIR_JUMP_UNIQ: i32 = 0x32; //check air jump uniq
    pub const CHECK_AIR_JUMP_AERIAL_UNIQ: i32 = 0x33; //check air jump aerial uniq
    pub const GUARD_CONT_UNIQ: i32 = 0x34; //guard cont uniq
    pub const TURN_UNIQ: i32 = 0x35; //turn uniq
    pub const CHECK_AIR_CLIFF_LASSO_UNIQ: i32 = 0x36; //check air cliff lasso uniq
    pub const LANDING_UNIQ_CHECK_STRANS_UNIQ: i32 = 0x37; //landing uniq check strans uniq
    pub const CHECK_SPECIAL_N_UNIQ: i32 = 0x38; //check special n uniq
    pub const CHECK_SPECIAL_S_UNIQ: i32 = 0x39; //check special s uniq
    pub const CHECK_SPECIAL_HI_UNIQ: i32 = 0x3A; //check special hi uniq
    pub const CHECK_SPECIAL_LW_UNIQ: i32 = 0x3B; //check special lw uniq
    pub const CHECK_SPECIAL_COMMAND: i32 = 0x3C; //check special command
    pub const WAZA_CUSTOMIZE_CONTROL: i32 = 0x3D; //waza customize control
    pub const STATUS_END_CONTROL: i32 = 0x3E; //status end control
    pub const UNK12: i32 = 0x3F;
    pub const UNK13: i32 = 0x40;
    pub const UNK14: i32 = 0x41;
    pub const DAMAGE_MOTION_KIND_CALLBACK: i32 = 0x42;
    pub const SUB_UNIQ_DAMAGE_FLY_UNIQ: i32 = 0x43;
    pub const DOWN_DAMAGE_UNIQ: i32 = 0x44;
    pub const THROW_F_STATUS_KIND: i32 = 0x45;
    pub const THROW_B_STATUS_KIND: i32 = 0x46;
    pub const THROW_HI_STATUS_KIND: i32 = 0x47;
    pub const THROW_LW_STATUS_KIND: i32 = 0x48;
    pub const DAMAGE_STOP_MOTION_INTP_FRAME: i32 = 0x49;
    pub const SUB_REBIRTH_UNIQ_INIT_CORE_UNIQ: i32 = 0x4A;
    pub const SUB_REBIRTH_UNIQ_EXEC_UNIQ: i32 = 0x4B;
    pub const SUB_DEAD_UNIQ_INIT_UNIQ: i32 = 0x4C;
    pub const SUB_ROULETTE_SET_SETP_UNIQ: i32 = 0x4D;
    pub const FALL_BRAKE_UNIQ: i32 = 0x4E;
    pub const CHECK_GROUND_GUARD_UNIQ: i32 = 0x4F;
    pub const CHECK_GROUND_CATCH_UNIQ: i32 = 0x50;
    pub const CHECK_COMMAND_WALK_UNIQ: i32 = 0x51;
    pub const CHECK_GROUND_JUMP_MINI_ATTACK: i32 = 0x52;
    pub const CHECK_AIR_ITEM_THROW_POST: i32 = 0x53;
    pub const IS_ITEM_SHOOT_STATUS_UNIQ: i32 = 0x54;
    pub const CHECK_ATTACK_3_UNIQ: i32 = 0x55;
    pub const CHECK_ATTACK_N_UNIQ: i32 = 0x56;
    pub const CHECK_ATTACK_S4_UNIQ: i32 = 0x57;
    pub const CHECK_ATTACK_HI4_UNIQ: i32 = 0x58;
    pub const CHECK_ATTACK_LW4_UNIQ: i32 = 0x59;
    pub const SQUAT_COMMON_UNIQ: i32 = 0x5A;
}

#[macro_use]
mod common;

unsafe fn calc_nnsdk_offset() -> u64 {
    let mut symbol = 0usize;
    skyline::nn::ro::LookupSymbol(&mut symbol, b"_ZN7android7IBinderD1Ev\0".as_ptr());
    (symbol - 0x240) as u64
}

static mut OFFSET1: u64 = 0;
static mut OFFSET2: u64 = 0;

#[skyline::hook(replace = OFFSET1)]
unsafe fn set_interval_1(window: u64, _: i32) {
    call_original!(window, 0);
}

#[skyline::hook(replace = OFFSET2, inline)]
unsafe fn set_interval_2(ctx: &mut InlineCtx) {
    *ctx.registers[8].x.as_mut() = 0;
}

static mut RUN: AtomicBool = AtomicBool::new(false);

#[skyline::hook(offset = 0x380f9e4, inline)]
unsafe fn vsync_count_thread(_: &skyline::hooks::InlineCtx) {
    RUN.store(true, Ordering::SeqCst);
}

static mut DUMMY_BLOCK: [u8; 0x100] = [0; 0x100];

#[skyline::hook(offset = 0x3746afc, inline)]
unsafe fn run_scene_update(_: &skyline::hooks::InlineCtx) {
    while !RUN.swap(false, Ordering::SeqCst) {
        skyline::nn::hid::GetNpadFullKeyState(DUMMY_BLOCK.as_mut_ptr() as _, &0);
    }
}

std::arch::global_asm!(
    r#"
    .section .nro_header
    .global __nro_header_start
    .word 0
    .word _mod_header
    .word 0
    .word 0
    
    .section .rodata.module_name
        .word 0
        .word 3
        .ascii "hdr"
    .section .rodata.mod0
    .global _mod_header
    _mod_header:
        .ascii "MOD0"
        .word __dynamic_start - _mod_header
        .word __bss_start - _mod_header
        .word __bss_end - _mod_header
        .word __eh_frame_hdr_start - _mod_header
        .word __eh_frame_hdr_end - _mod_header
        .word __nx_module_runtime - _mod_header // runtime-generated module object offset
    .global IS_NRO
    IS_NRO:
        .word 1
    
    .section .bss.module_runtime
    __nx_module_runtime:
    .space 0xD0
    "#
);

#[no_mangle]
pub extern "C" fn main() {
    common::install();
    
    unsafe {
        OFFSET1 = calc_nnsdk_offset() + 0x429d60;
        OFFSET2 = calc_nnsdk_offset() + 0x26e94;
    }

    skyline::install_hooks!(
        set_interval_1,
        set_interval_2,
        run_scene_update,
        vsync_count_thread,
    );
}