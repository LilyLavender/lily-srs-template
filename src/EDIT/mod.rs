use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
		hash40
    },
    smash_script::*,
	smashline::*
};

// Game, effect, statuses
unsafe extern "C" fn FUNCTIONNAME(agent: &mut L2CAgentBase) {
	
}

// Fighter frame
unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
	unsafe {
		
	}
}

pub fn install() {
	Agent::new("FIGHTERNAME")
		.game_acmd("game_NAME", GAMEFUNCTIONNAME)
		.effect_acmd("effect_NAME", EFFFUNCTIONNAME)
		.on_line(Main, fighter_frame)
		.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, STATUSFUNCTIONNAME)
		.install();
}