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
#[acmd_script( agent = "mario", script = "game_ATTACK_NAME_HERE", category = ACMD_GAME, low_priority )]
unsafe fn example_acmd_script(agent: &mut L2CAgentBase) {
    
}

// Global opff
#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        
    }
}

// Char opff
#[fighter_frame( agent = FIGHTER_KIND_MARIO )]
fn mario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        
    }
}

// Status Script
#[status_script(agent = "mario", status = FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn example_status_script(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    // Game, effect, sound, expression
    smashline::install_acmd_scripts!(
        example_acmd_script
    );
    // Global fighter frame
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame
    );
    // Agent fighter frame
    smashline::install_agent_frames!(
        mario_frame
    );
    // Status Script
    install_status_scripts!(
        example_status_script
    );
}
