
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Root {
    pub result: String,
    pub template: Vec<TemplateWrapper>,
    #[serde(rename = "batchId")]
    pub batch_id: String,
    #[serde(rename = "experimentId")]
    pub experiment_id: Vec<u16>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemplateWrapper {
    #[serde(rename = "templateId")]
    pub template_id: Option<String>,
    pub data: Template,
}

#[derive(Debug, Deserialize)]
// #[serde(deny_unknown_fields)]//minimalistic approach
pub struct Template {
    #[serde(rename = "templateId")]
    pub template_id: Option<String>,
    #[serde(rename = "combatMove")]
    pub combat_move: Option<CombatMove>,
    #[serde(rename = "playerLevel")]
    pub player_level: Option<PlayerLevel>,
    pub pokemon: Option<PokemonSettings>,
    #[serde(rename = "combatStatStageSettings")]
    pub combat_stat_stage_settings: Option<CombatStatStageSettings>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CombatMove {
    #[serde(rename = "uniqueId")]
    pub unique_id: String,
    pub r#type: String,
    pub power: Option<f32>,
    #[serde(rename = "vfxName")]
    pub vfx_name: String,
    #[serde(rename = "energyDelta")]
    pub energy_delta: Option<i8>,
    #[serde(rename = "durationTurns")]
    pub duration_turns: Option<u8>,
    pub buffs: Option<Buffs>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Buffs {
    #[serde(rename = "attackerAttackStatStageChange")]
    pub attacker_attack_stat_stage_change: Option<i8>,
    #[serde(rename = "attackerDefenseStatStageChange")]
    pub attacker_defense_stat_stage_change: Option<i8>,
    #[serde(rename = "targetAttackStatStageChange")]
    pub target_attack_stat_stage_change: Option<i8>,
    #[serde(rename = "targetDefenseStatStageChange")]
    pub target_defense_stat_stage_change: Option<i8>,
    #[serde(rename = "buffActivationChance")]
    pub buff_activation_chance: f32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PlayerLevel {
    #[serde(rename = "rankNum")]
    pub rank_num: Vec<u8>,
    #[serde(rename = "requiredExp")]
    pub required_exp: Vec<u32>,
    #[serde(rename = "cpMultiplier")]
    pub cp_multiplier: Vec<f64>,
    #[serde(rename = "maxEggPlayerLevel")]
    pub max_egg_player_level: u8,
    #[serde(rename = "maxEncounterPlayerLevel")]
    pub max_encounter_player_level: u8,
    #[serde(rename = "maxQuestEncounterPlayerLevel")]
    pub max_quest_encounter_player_level: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PokemonSettings {
    #[serde(rename = "uniqueId")]
    pub unique_id: String,
    #[serde(rename = "modelScale")]
    pub model_scale: Option<f32>,
    pub type1: String,
    pub type2: Option<String>,
    pub camera: CameraSetting,
    pub encounter: Encounter,
    pub stats: Stats,
    #[serde(rename = "quickMoves")]
    pub quick_moves: Option<Vec<String>>,
    #[serde(rename = "cinematicMoves")]
    pub cinematic_moves: Option<Vec<String>>,
    #[serde(rename = "animTime")]
    pub anim_time: Option<Vec<f32>>,
    pub evolution: Option<Vec<String>>,
    #[serde(rename = "evolutionPips")]
    pub evolution_pips: Option<u8>,
    #[serde(rename = "pokemonClass")]
    pub pokemon_class: Option<String>,
    #[serde(rename = "pokedexHeightM")]
    pub pokedex_height_m: Option<f32>,
    #[serde(rename = "pokedexWeightKg")]
    pub pokedex_weight_kg: Option<f32>,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "heightStdDev")]
    pub height_std_dev: Option<f32>,
    #[serde(rename = "weightStdDev")]
    pub weight_std_dev: Option<f32>,
    #[serde(rename = "familyId")]
    pub family_id: String,
    #[serde(rename = "candyToEvolve")]
    pub candy_to_evolve: Option<u16>,
    #[serde(rename = "kmBuddyDistance")]
    pub km_buddy_distance: f32,
    #[serde(rename = "buddySize")]
    pub buddy_size: Option<String>,
    #[serde(rename = "modelHeight")]
    pub model_height: f32,
    #[serde(rename = "evolutionBranch")]
    pub evolution_branch: Option<Vec<EvolutionBranch>>,
    #[serde(rename = "modelScaleV2")]
    pub model_scale_v2: f32,
    pub form: Option<String>,
    #[serde(rename = "buddyOffsetMale")]
    pub buddy_offset_male: Option<Vec<f32>>,
    #[serde(rename = "buddyOffsetFemale")]
    pub buddy_offset_female: Option<Vec<f32>>,
    #[serde(rename = "buddyScale")]
    pub buddy_scale: Option<f32>,
    #[serde(rename = "buddyPortraitOffset")]
    pub buddy_portrait_offset: Option<Vec<f32>>,
    #[serde(rename = "thirdMove")]
    pub third_move: ThirdMove,
    #[serde(rename = "isTransferable")]
    pub is_transferable: Option<bool>,
    #[serde(rename = "isDeployable")]
    pub is_deployable: Option<bool>,
    #[serde(rename = "isTradable")]
    pub is_tradable: Option<bool>,
    #[serde(rename = "combatShoulderCameraAngle")]
    pub combat_shoulder_camera_angle: Option<Vec<f32>>,
    #[serde(rename = "combatDefaultCameraAngle")]
    pub combat_default_camera_angle: Option<Vec<f32>>,
    #[serde(rename = "combatPlayerFocusCameraAngle")]
    pub combat_player_focus_camera_angle: Option<Vec<f32>>,
    pub shadow: Option<Shadow>,
    #[serde(rename = "combatPlayerPokemonPositionOffset")]
    pub combat_player_pokemon_position_offset: Option<Vec<f32>>,
    #[serde(rename = "combatOpponentFocusCameraAngle")]
    pub combat_opponent_focus_camera_angle: Option<Vec<f32>>,
    #[serde(rename = "buddyGroupNumber")]
    pub buddy_group_number: Option<u8>,
    #[serde(rename = "raidBossDistanceOffset")]
    pub raid_boss_distance_offset: Option<f32>,
    #[serde(rename = "eliteQuickMove")]
    pub elite_quick_move: Option<Vec<String>>,
    #[serde(rename = "eliteCinematicMove")]
    pub elite_cinematic_move: Option<Vec<String>>,
    #[serde(rename = "tempEvoOverrides")]
    pub temp_evo_overrides: Option<Vec<TempEvoOverrides>>,
    #[serde(rename = "buddyWalkedMegaEnergyAward")]
    pub buddy_walked_mega_energy_award: Option<u8>,
    #[serde(rename = "disableTransferToPokemonHome")]
    pub disable_transfer_to_pokemon_home: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TempEvoOverrides {
    #[serde(rename = "tempEvoId")]
    pub temp_evo_id: String,
    pub stats: Stats,
    #[serde(rename = "averageHeightM")]
    pub average_height_m: f32,
    #[serde(rename = "averageWeightKg")]
    pub average_weight_kg: f32,
    #[serde(rename = "typeOverride1")]
    pub type_override1: String,
    #[serde(rename = "typeOverride2")]
    pub type_override2: Option<String>,
    pub camera: Option<CameraSetting>,
    #[serde(rename = "modelScaleV2")]
    pub model_scale_v2: Option<f32>,
    #[serde(rename = "modelHeight")]
    pub model_height: Option<f32>,
    #[serde(rename = "buddyOffsetMale")]
    pub buddy_offset_male: Option<Vec<f32>>,
    #[serde(rename = "buddyOffsetFemale")]
    pub buddy_offset_female: Option<Vec<f32>>,
    #[serde(rename = "buddyPortraitOffset")]
    pub buddy_portrait_offset: Option<Vec<f32>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CameraSetting {
    #[serde(rename = "diskRadiusM")]
    pub disk_radius_m: Option<f32>,
    #[serde(rename = "cylRadiusM")]
    pub cyl_radius_m: Option<f32>,
    #[serde(rename = "cylHeightM")]
    pub cyl_height_m: Option<f32>,
    #[serde(rename = "cylGroundM")]
    pub cyl_ground_m: Option<f32>,
    #[serde(rename = "shoulderModeScale")]
    pub shoulder_mode_scale: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Encounter {
    #[serde(rename = "baseCaptureRate")]
    pub base_capture_rate: Option<f32>,
    #[serde(rename = "baseFleeRate")]
    pub base_flee_rate: Option<f32>,
    #[serde(rename = "collisionRadiusM")]
    pub collision_radius_m: f32,
    #[serde(rename = "collisionHeightM")]
    pub collision_height_m: f32,
    #[serde(rename = "collisionHeadRadiusM")]
    pub collision_head_radius_m: f32,
    #[serde(rename = "movementType")]
    pub movement_type: Option<String>,
    #[serde(rename = "movementTimerS")]
    pub movement_timer_s: Option<f32>,
    #[serde(rename = "jumpTimeS")]
    pub jump_time_s: Option<f32>,
    #[serde(rename = "attackTimerS")]
    pub attack_timer_s: Option<f32>,
    #[serde(rename = "bonusCandyCaptureReward")]
    pub bonus_candy_capture_reward: Option<u8>,
    #[serde(rename = "bonusStardustCaptureReward")]
    pub bonus_stardust_capture_reward: Option<u16>,
    #[serde(rename = "attackProbability")]
    pub attack_probability: Option<f32>,
    #[serde(rename = "dodgeProbability")]
    pub dodge_probability: Option<f32>,
    #[serde(rename = "dodgeDurationS")]
    pub dodge_duration_s: Option<f32>,
    #[serde(rename = "dodgeDistance")]
    pub dodge_distance: Option<f32>,
    #[serde(rename = "cameraDistance")]
    pub camera_distance: f32,
    #[serde(rename = "minPokemonActionFrequencyS")]
    pub min_pokemon_action_frequency_s: f32,
    #[serde(rename = "maxPokemonActionFrequencyS")]
    pub max_pokemon_action_frequency_s: f32,
    #[serde(rename = "bonusXlCandyCaptureReward")]
    pub bonus_xl_candy_capture_reward: Option<u8>,
}

#[derive(Debug, Deserialize, PartialEq, Copy, Clone)]
#[serde(deny_unknown_fields)]
pub struct Stats {
    #[serde(default, rename = "baseStamina")]
    pub base_stamina: u16,
    #[serde(default, rename = "baseAttack")]
    pub base_attack: u16,
    #[serde(default, rename = "baseDefense")]
    pub base_defense: u16,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvolutionBranch {
    pub evolution: Option<String>,
    #[serde(rename = "evolutionItemRequirement")]
    pub evolution_item_requirement: Option<String>,
    #[serde(rename = "candyCost")]
    pub candy_cost: Option<u16>,
    #[serde(rename = "kmBuddyDistanceRequirement")]
    pub km_buddy_distance_requirement: Option<f32>,
    #[serde(rename = "mustBeBuddy")]
    pub must_be_buddy: Option<bool>,
    #[serde(rename = "onlyDaytime")]
    pub only_daytime: Option<bool>,
    #[serde(rename = "onlyNighttime")]
    pub only_nighttime: Option<bool>,
    pub priority: Option<u8>,
    pub form: Option<String>,
    #[serde(rename = "noCandyCostViaTrade")]
    pub no_candy_cost_via_trade: Option<bool>,
    #[serde(rename = "lureItemRequirement")]
    pub lure_item_requirement: Option<String>,
    #[serde(rename = "genderRequirement")]
    pub gender_requirement: Option<String>,
    #[serde(rename = "temporaryEvolution")]
    pub temporary_evolution: Option<String>,
    #[serde(rename = "temporaryEvolutionEnergyCost")]
    pub temporary_evolution_energy_cost: Option<u8>,
    #[serde(rename = "temporaryEvolutionEnergyCostSubsequent")]
    pub temporary_evolution_energy_cost_subsequent: Option<u8>,
    #[serde(rename = "questDisplay")]
    pub quest_display: Option<Vec<QuestDisplay>>,
    #[serde(rename = "onlyUpsideDown")]
    pub only_upside_down: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuestDisplay {
    #[serde(rename = "questRequirementTemplateId")]
    pub quest_requirement_template_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThirdMove {
    #[serde(rename = "stardustToUnlock")]
    pub stardust_to_unlock: Option<u32>,
    #[serde(rename = "candyToUnlock")]
    pub candy_to_unlock: u32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Shadow {
    #[serde(rename = "purificationStardustNeeded")]
    pub purification_stardust_needed: u16,
    #[serde(rename = "purificationCandyNeeded")]
    pub purification_candy_needed: u8,
    #[serde(rename = "purifiedChargeMove")]
    pub purified_charge_move: String,
    #[serde(rename = "shadowChargeMove")]
    pub shadow_charge_move: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct CombatStatStageSettings {
    #[serde(rename = "minimumStatStage")]
    pub minimum_stat_stage: i8,
    #[serde(rename = "maximumStatStage")]
    pub maximum_stat_stage: i8,
    #[serde(rename = "attackBuffMultiplier")]
    pub attack_buff_multiplier: Vec<f64>,
    #[serde(rename = "defenseBuffMultiplier")]
    pub defense_buff_multiplier: Vec<f64>,
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn it_works() {
        reqwest::get("https://raw.githubusercontent.com/pokemongo-dev-contrib/pokemongo-game-master/master/versions/latest/V2_GAME_MASTER.json")
            .await
            .unwrap()
            .json::<crate::Root>()
            .await
            .unwrap();
    }
}
