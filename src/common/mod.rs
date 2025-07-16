use smash::lib::{L2CValue, L2CAgent};
use skyline::nro::{self, NroInfo};
use smash::app::BattleObjectModuleAccessor;
use smash::app::lua_bind::*;
use smash::hash40;
use smash::app::utility::get_kind;
use smash::app::utility::*;
use smash::lua2cpp::*;
use smash::lib::lua_const::*;
use smash::phx::*;
use smash::app::*;
use smash::app;
use smashline::*;

pub unsafe extern "C" fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    JostleModule::set_team(fighter.module_accessor, 0);
}
