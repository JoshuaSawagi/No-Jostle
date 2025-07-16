use smashline::*;
use smash::hash40;
use smash_script::*;
use smash::lua2cpp::*;
use smash::phx::Hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use crate::common::global_fighter_frame;

unsafe extern "C" fn global_on_main(fighter: &mut L2CFighterCommon) {
    unsafe {
        global_fighter_frame(fighter);
    }
}


pub fn install() {
	Agent::new("zelda")
	.on_line(Main, global_on_main)
	.install();
}