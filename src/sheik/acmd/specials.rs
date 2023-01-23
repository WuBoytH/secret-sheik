use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "sheik_needle", script = "game_move", category = ACMD_GAME, low_priority )]
unsafe fn game_move(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        // Damage increased from 1.5 to 2.5
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 1.5, 60, 167, 0, 0, 1.5, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        // Histun increased by 10.0
        AttackModule::set_add_reaction_frame_revised(weapon.module_accessor, 0, 10.0, false);
    }
    // Removed damage decrease so damage stays at 2.5
    // frame(weapon.lua_state_agent, 5.0);
    // if macros::is_excute(weapon) {
    //     macros::ATK_POWER(weapon, 0, 0.8);
    // }
}

#[acmd_script( agent = "sheik", script = "game_speciallwattack", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        // Damage increased from 11 to 16
        // BKB increased from 26 to 40
        // Hitbox size increased by 0.5
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 16.0, 361, 97, 0, 40, 4.0, 5.5, 0.0, 0.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 16.0, 361, 97, 0, 40, 3.5, -3.0, 0.0, 0.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_keep_rumble(fighter.module_accessor, 0, true);
        AttackModule::set_attack_keep_rumble(fighter.module_accessor, 1, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

#[acmd_script( agent = "sheik", script = "game_speciallwattackreturn", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwattackreturn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        // Damage increased from 13 to 18
        // BKB increased from 28 to 42
        // Hitbox size increased by 0.5
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 18.0, 361, 97, 0, 42, 4.0, 5.5, 0.0, 0.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 18.0, 361, 97, 0, 42, 3.5, -3.0, 0.0, 0.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_keep_rumble(fighter.module_accessor, 0, true);
        AttackModule::set_attack_keep_rumble(fighter.module_accessor, 1, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_move,
        game_speciallwattack,
        game_speciallwattackreturn
    );
}