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

// Game acmd script
#[acmd_script( agent = "AGENTNAME", script = "GAME_NAME", category = ACMD_GAME, low_priority )]
unsafe fn ACMDSCRIPTNAME(agent: &mut L2CAgentBase) {

}

// Global opff
#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {

    }
}

// Char opff
#[fighter_frame( agent = FIGHTER_KIND_ )]
fn AGENTFRAMENAME(fighter: &mut L2CFighterCommon) {
    unsafe {

	}
}

// Status Script
#[status_script(agent = "AGENT", status = STATUS, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn STATUSSCRIPTNAME(fighter: &mut L2CFighterCommon) -> L2CValue {

}

pub fn install() {
	// Game, effect, sound, expression
    smashline::install_acmd_scripts!(
        ACMDSCRIPTNAME
    );
	// Global fighter frame
    smashline::install_agent_frame_callbacks!(
		global_fighter_frame
    );
	// Agent fighter frame
    smashline::install_agent_frames!(
        AGENTFRAMENAME
    );
	// Status Script
    install_status_scripts!(
		STATUSSCRIPTNAME
	);
}
