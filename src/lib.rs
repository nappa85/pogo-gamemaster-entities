use serde::{Deserialize, Serialize};

// generated with https://transform.tools/json-to-rust-serde

#[derive(Debug, Deserialize)]
//#[serde(deny_unknown_fields)]
pub struct Root {
    pub result: String,
    pub template: Vec<TemplateWrapper>,
    #[serde(rename = "batchId")]
    pub batch_id: String,
    #[serde(rename = "experimentId")]
    pub experiment_id: Vec<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateWrapper {
    pub template_id: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub template_id: String,
    pub addressable_pokemon_settings: Option<AddressablePokemonSettings>,
    pub address_book_import_settings: Option<AddressBookImportSettings>,
    pub ob_advanced_settings: Option<ObAdvancedSettings>,
    pub ar_telemetry_settings: Option<ArTelemetrySettings>,
    pub ob_asset_refresh_settings: Option<ObAssetRefreshSettings>,
    pub avatar_group_order_settings: Option<AvatarGroupOrderSettings>,
    pub avatar_customization: Option<AvatarCustomization>,
    pub level_up_reward_settings: Option<LevelUpRewardSettings>,
    pub background_mode_settings: Option<BackgroundModeSettings>,
    pub badge_settings: Option<BadgeSettings>,
    pub battle_hub_badge_settings: Option<BattleHubBadgeSettings>,
    pub battle_hub_order_settings: Option<BattleHubOrderSettings>,
    pub battle_party_settings: Option<BattlePartySettings>,
    pub battle_settings: Option<BattleSettings>,
    pub ob_battle_visual_settings: Option<ObBattleVisualSettings>,
    pub beluga_pokemon_whitelist: Option<BelugaPokemonWhitelist>,
    pub buddy_activity_settings: Option<BuddyActivitySettings>,
    pub buddy_activity_category_settings: Option<BuddyActivityCategorySettings>,
    pub buddy_emotion_level_settings: Option<BuddyEmotionLevelSettings>,
    pub buddy_encounter_cameo_settings: Option<BuddyEncounterCameoSettings>,
    pub buddy_hunger_settings: Option<BuddyHungerSettings>,
    pub buddy_interaction_settings: Option<BuddyInteractionSettings>,
    pub buddy_level_settings: Option<BuddyLevelSettings>,
    pub buddy_swap_settings: Option<BuddySwapSettings>,
    pub buddy_walk_settings: Option<BuddyWalkSettings>,
    pub butterfly_collector_settings: Option<ButterflyCollectorSettings>,
    pub ob_campfire_settings: Option<ObCampfireSettings>,
    pub evolution_quest_template: Option<EvolutionQuestTemplate>,
    pub invasion_npc_display_settings: Option<InvasionNpcDisplaySettings>,
    pub combat_competitive_season_settings: Option<CombatCompetitiveSeasonSettings>,
    pub combat_league: Option<CombatLeague>,
    pub combat_league_settings: Option<CombatLeagueSettings>,
    pub combat_type: Option<CombatType>,
    pub combat_ranking_proto_settings: Option<CombatRankingProtoSettings>,
    pub combat_settings: Option<CombatSettings>,
    pub combat_stat_stage_settings: Option<CombatStatStageSettings>,
    pub combat_move: Option<CombatMove>,
    pub cross_game_social_settings: Option<CrossGameSocialSettings>,
    pub ob_daily_adventure_incense_settings: Option<ObDailyAdventureIncenseSettings>,
    pub deep_linking_settings: Option<DeepLinkingSettings>,
    pub ob_egg_hatch_improvement_settings: Option<ObEggHatchImprovementSettings>,
    pub egg_transparency_settings: Option<EggTransparencySettings>,
    pub friend_profile_settings: Option<FriendProfileSettings>,
    pub encounter_settings: Option<EncounterSettings>,
    pub pokemon_home_energy_costs: Option<PokemonHomeEnergyCosts>,
    pub ob_evolution_chain_display_settings: Option<ObEvolutionChainDisplaySettings>,
    pub ob_evolve_preview_settings: Option<ObEvolvePreviewSettings>,
    pub pokemon_extended_settings: Option<PokemonExtendedSettings>,
    pub external_addressable_assets_settings: Option<ExternalAddressableAssetsSettings>,
    pub ex_raid_settings: Option<ExRaidSettings>,
    pub ob_feature_unlock_settings: Option<ObFeatureUnlockSettings>,
    pub ob_forms_refactor_settings: Option<ObFormsRefactorSettings>,
    pub form_settings: Option<FormSettings>,
    pub ob_fort_power_up_settings: Option<ObFortPowerUpSettings>,
    pub friendship_milestone_settings: Option<FriendshipMilestoneSettings>,
    pub geotargeted_quest_settings: Option<GeotargetedQuestSettings>,
    pub ob_gifting_settings: Option<ObGiftingSettings>,
    pub gui_search_settings: Option<GuiSearchSettings>,
    pub gym_badge_settings: Option<GymBadgeSettings>,
    pub gym_level: Option<GymLevel>,
    pub ob_game_master_language_settings: Option<ObGameMasterLanguageSettings>,
    pub iap_category_display: Option<IapCategoryDisplay>,
    pub iap_settings: Option<IapSettings>,
    pub incident_priority_settings: Option<IncidentPrioritySettings>,
    pub ob_invasion_character_settings: Option<ObInvasionCharacterSettings>,
    pub pokestop_invasion_availability_settings: Option<PokestopInvasionAvailabilitySettings>,
    pub inventory_settings: Option<InventorySettings>,
    pub item_settings: Option<ItemSettings>,
    pub item_inventory_update_settings: Option<ItemInventoryUpdateSettings>,
    pub ob_language_selector_settings: Option<ObLanguageSelectorSettings>,
    pub loading_screen_settings: Option<LoadingScreenSettings>,
    pub limited_purchase_sku_settings: Option<LimitedPurchaseSkuSettings>,
    pub lucky_pokemon_settings: Option<LuckyPokemonSettings>,
    pub map_display_settings: Option<MapDisplaySettings>,
    pub ob_interaction_range_settings: Option<ObInteractionRangeSettings>,
    pub ob_mega_level_settings: Option<ObMegaLevelSettings>,
    pub mega_evo_settings: Option<MegaEvoSettings>,
    pub monodepth_settings: Option<MonodepthSettings>,
    pub news_feed_client_settings: Option<NewsFeedClientSettings>,
    pub onboarding_settings: Option<OnboardingSettings>,
    #[serde(rename = "onboardingV2Settings")]
    pub onboarding_v2settings: Option<OnboardingV2Settings>,
    pub party_recommendation_settings: Option<PartyRecommendationSettings>,
    pub ob_photo_settings: Option<ObPhotoSettings>,
    pub platypus_rollout_settings: Option<PlatypusRolloutSettings>,
    pub player_level: Option<PlayerLevel>,
    pub pokecoin_purchase_display_gmt: Option<PokecoinPurchaseDisplayGmt>,
    pub ob_pokedex_categories_settings: Option<ObPokedexCategoriesSettings>,
    pub pokedex_size_stats_settings: Option<PokedexSizeStatsSettings>,
    pub pokemon_home_settings: Option<PokemonHomeSettings>,
    pub pokemon_scale_settings: Option<PokemonScaleSettings>,
    pub pokemon_tag_settings: Option<PokemonTagSettings>,
    pub type_effective: Option<TypeEffective>,
    pub pokemon_upgrades: Option<PokemonUpgrades>,
    pub ob_popup_control_settings: Option<ObPopupControlSettings>,
    pub ob_post_card_collection_settings: Option<ObPostCardCollectionSettings>,
    pub ob_power_up_poi_settings: Option<ObPowerUpPoiSettings>,
    pub ob_push_gateway_settings: Option<ObPushGatewaySettings>,
    pub quest_evolution_settings: Option<QuestEvolutionSettings>,
    pub quest_settings: Option<QuestSettings>,
    pub raid_settings: Option<RaidSettings>,
    pub recomended_search_settings: Option<RecomendedSearchSettings>,
    pub referral_settings: Option<ReferralSettings>,
    pub route_creation_settings: Option<RouteCreationSettings>,
    pub route_discovery_settings: Option<RouteDiscoverySettings>,
    pub route_play_settings: Option<RoutePlaySettings>,
    pub ob_route_stamp_category_settings: Option<ObRouteStampCategorySettings>,
    pub ob_shared_move_settings: Option<ObSharedMoveSettings>,
    pub smeargle_moves_settings: Option<SmeargleMovesSettings>,
    pub gender_settings: Option<GenderSettings>,
    pub sponsored_geofence_gift_settings: Option<SponsoredGeofenceGiftSettings>,
    pub sticker_metadata: Option<StickerMetadata>,
    pub iap_item_display: Option<IapItemDisplay>,
    pub ob_in_app_survey_settings: Option<ObInAppSurveySettings>,
    pub temporary_evolution_settings: Option<TemporaryEvolutionSettings>,
    pub ob_ticket_gifting_settings: Option<ObTicketGiftingSettings>,
    pub combat_npc_trainer: Option<CombatNpcTrainer>,
    pub combat_npc_personality: Option<CombatNpcPersonality>,
    pub pokemon_family: Option<PokemonFamily>,
    pub pokemon_settings: Option<PokemonSettings>,
    pub move_settings: Option<MoveSettings>,
    pub pokemon_home_form_reversions: Option<PokemonHomeFormReversions>,
    pub ob_verbose_combat_setting: Option<ObVerboseCombatSetting>,
    pub ob_verbose_raid_settings: Option<ObVerboseRaidSettings>,
    pub vs_seeker_client_settings: Option<VsSeekerClientSettings>,
    pub vs_seeker_loot: Option<VsSeekerLoot>,
    pub vs_seeker_pokemon_rewards: Option<VsSeekerPokemonRewards>,
    pub ob_vs_seeker_schedule_settings: Option<ObVsSeekerScheduleSettings>,
    pub weather_affinities: Option<WeatherAffinities>,
    pub weather_bonus_settings: Option<WeatherBonusSettings>,
    #[serde(rename = "adventureSyncV2Gmt")]
    pub adventure_sync_v2gmt: Option<AdventureSyncV2Gmt>,
    pub camera: Option<Camera3>,
    pub ob_impression_tracking_settings: Option<ObImpressionTrackingSettings>,
    pub move_sequence_settings: Option<MoveSequenceSettings>,
    pub ob_sticker_category_settings: Option<ObStickerCategorySettings>,
    pub ob_tutorial_settings: Option<ObTutorialSettings>,
    pub ob_username_suggestion_settings: Option<ObUsernameSuggestionSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressablePokemonSettings {
    pub ob_addressable_pokemon_id: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressBookImportSettings {
    pub is_enabled: bool,
    pub onboarding_screen_level: i64,
    pub show_opt_out_checkbox: bool,
    pub reprompt_onboarding_for_v1: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObAdvancedSettings {
    pub ob_download_all_assets_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArTelemetrySettings {
    pub measure_battery: bool,
    pub battery_sampling_interval_ms: i64,
    pub measure_framerate: bool,
    pub framerate_sampling_interval_ms: i64,
    pub percentage_sessions_to_sample: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObAssetRefreshSettings {
    pub ob_check_for_new_assets_time_seconds: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarGroupOrderSettings {
    pub group: Vec<Group>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub name: String,
    pub order: i64,
    pub ob_show_new_tag: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarCustomization {
    pub enabled: Option<bool>,
    pub slot: Vec<String>,
    pub bundle_name: Option<String>,
    pub asset_name: String,
    pub group_name: String,
    pub sort_order: i64,
    pub unlock_type: String,
    pub iap_sku: Option<String>,
    pub icon_name: Option<String>,
    pub unlock_badge_type: Option<String>,
    pub unlock_badge_level: Option<i64>,
    pub unlock_player_level: Option<i64>,
    pub avatar_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelUpRewardSettings {
    pub level: i64,
    pub items: Vec<String>,
    pub items_count: Vec<i64>,
    #[serde(default)]
    pub items_unlocked: Vec<String>,
    #[serde(default)]
    pub avatar_template_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundModeSettings {
    #[serde(rename = "weeklyFitnessGoalLevel1DistanceKm")]
    pub weekly_fitness_goal_level1distance_km: f64,
    #[serde(rename = "weeklyFitnessGoalLevel2DistanceKm")]
    pub weekly_fitness_goal_level2distance_km: f64,
    #[serde(rename = "weeklyFitnessGoalLevel3DistanceKm")]
    pub weekly_fitness_goal_level3distance_km: f64,
    #[serde(rename = "weeklyFitnessGoalLevel4DistanceKm")]
    pub weekly_fitness_goal_level4distance_km: f64,
    #[serde(rename = "obWeeklyFitnessGoalLevel5DistanceKm")]
    pub ob_weekly_fitness_goal_level5distance_km: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadgeSettings {
    pub badge_type: String,
    pub badge_rank: i64,
    pub targets: Vec<i64>,
    pub event_badge: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleHubBadgeSettings {
    pub combat_hub_displayed_badges: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleHubOrderSettings {
    pub section: Vec<Section>,
    pub section_group: Vec<SectionGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    pub main_section: String,
    pub subsection: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionGroup {
    pub section: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BattlePartySettings {
    pub max_battle_parties: i64,
    pub overall_parties_cap: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleSettings {
    pub retarget_seconds: f64,
    pub enemy_attack_interval: f64,
    pub attack_server_interval: f64,
    pub round_duration_seconds: f64,
    pub bonus_time_per_ally_seconds: f64,
    pub maximum_attackers_per_battle: i64,
    pub same_type_attack_bonus_multiplier: f64,
    pub maximum_energy: i64,
    pub energy_delta_per_health_lost: f64,
    pub dodge_duration_ms: i64,
    pub minimum_player_level: i64,
    pub swap_duration_ms: i64,
    pub dodge_damage_reduction_percent: f64,
    pub minimum_raid_player_level: i64,
    pub shadow_pokemon_attack_bonus_multiplier: f64,
    pub shadow_pokemon_defense_bonus_multiplier: f64,
    pub purified_pokemon_attack_multiplier_vs_shadow: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObBattleVisualSettings {
    pub ob_battle_visual_stadium_enabled: bool,
    pub ob_stadium_crowd_asset: String,
    pub ob_stadium_banner_asset: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BelugaPokemonWhitelist {
    pub max_allowed_pokemon_pokedex_number: i64,
    pub additional_pokemon_allowed: Vec<String>,
    pub costumes_allowed: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuddyActivitySettings {
    pub activity: String,
    pub activity_category: String,
    pub max_times_per_day: i64,
    pub num_points_per_action: i64,
    pub num_emotion_points_per_action: i64,
    pub emotion_cooldown_duration_ms: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuddyActivityCategorySettings {
    pub activity_category: String,
    pub max_points_per_day: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuddyEmotionLevelSettings {
    pub emotion_level: String,
    pub min_emotion_points_required: Option<i64>,
    pub emotion_animation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuddyEncounterCameoSettings {
    pub buddy_wild_encounter_cameo_chance_percent: f64,
    pub buddy_quest_encounter_cameo_chance_percent: f64,
    pub buddy_raid_encounter_cameo_chance_percent: f64,
    pub buddy_invasion_encounter_cameo_chance_percent: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuddyHungerSettings {
    pub num_hunger_points_required_for_full: i64,
    pub decay_points_per_bucket: i64,
    pub milliseconds_per_bucket: String,
    pub cooldown_duration_ms: String,
    pub decay_duration_after_full_ms: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuddyInteractionSettings {
    pub feed_item_whitelist: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuddyLevelSettings {
    pub level: String,
    pub min_non_cumulative_points_required: Option<i64>,
    #[serde(default)]
    pub unlocked_traits: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuddySwapSettings {
    pub max_swaps_per_day: i64,
    pub ob_buddy_swap_settings_bool1: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuddyWalkSettings {
    pub km_required_per_affection_point: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButterflyCollectorSettings {
    pub enabled: bool,
    pub version: i64,
    pub region: Vec<String>,
    pub daily_progress_from_inventory: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObCampfireSettings {
    pub ob_catch_card_enabled: bool,
    pub ob_catch_card_share_enabled: bool,
    pub ob_catch_card_time_to_share_to_campfire_s: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolutionQuestTemplate {
    pub quest_template_id: String,
    pub quest_type: String,
    pub goals: Vec<Goal>,
    pub context: String,
    pub display: Display,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    pub target: i64,
    #[serde(default)]
    pub condition: Vec<Condition>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    #[serde(rename = "type")]
    pub type_field: String,
    pub with_pokemon_type: Option<WithPokemonType>,
    pub with_throw_type: Option<WithThrowType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithPokemonType {
    pub pokemon_type: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithThrowType {
    pub throw_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Display {
    pub description: String,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvasionNpcDisplaySettings {
    pub trainer_name: String,
    pub avatar: Avatar,
    pub trainer_title: String,
    pub trainer_quote: String,
    pub icon_url: String,
    pub model_name: String,
    pub ob_party_selection_music: Option<String>,
    pub tutorial_on_loss_string: Option<String>,
    pub is_male: Option<bool>,
    pub backdrop_image_bundle: Option<String>,
    pub ob_combat_music: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub avatar: Option<i64>,
    pub skin: Option<i64>,
    pub avatar_hair: Option<String>,
    pub avatar_shirt: Option<String>,
    pub avatar_pants: Option<String>,
    pub avatar_hat: Option<String>,
    pub avatar_shoes: Option<String>,
    pub avatar_eyes: Option<String>,
    pub avatar_backpack: Option<String>,
    pub avatar_gloves: Option<String>,
    pub avatar_socks: Option<String>,
    pub avatar_belt: Option<String>,
    pub avatar_glasses: Option<String>,
    pub avatar_necklace: Option<String>,
    pub avatar_pose: Option<String>,
    pub avatar_face: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatCompetitiveSeasonSettings {
    pub season_end_time_timestamp: Vec<String>,
    pub rating_adjustment_percentage: f64,
    pub ranking_adjustment_percentage: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatLeague {
    pub title: String,
    pub enabled: bool,
    #[serde(default)]
    pub unlock_condition: Vec<UnlockCondition>,
    pub pokemon_condition: Vec<PokemonCondition>,
    pub icon_url: String,
    pub pokemon_count: i64,
    #[serde(default)]
    pub banned_pokemon: Vec<String>,
    pub badge_type: String,
    pub battle_party_combat_league_template_id: Option<String>,
    pub league_type: String,
    #[serde(default)]
    pub ob_combat_refactor_toggle: Vec<String>,
    pub allow_temp_evos: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnlockCondition {
    #[serde(rename = "type")]
    pub type_field: String,
    pub min_pokemon_count: i64,
    pub with_pokemon_cp_limit: Option<WithPokemonCpLimit>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithPokemonCpLimit {
    pub min_cp: i64,
    pub max_cp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonCondition {
    #[serde(rename = "type")]
    pub type_field: String,
    pub with_pokemon_cp_limit: Option<WithPokemonCpLimit2>,
    pub pokemon_caught_timestamp: Option<PokemonCaughtTimestamp>,
    pub pokemon_white_list: Option<PokemonWhiteList>,
    pub with_pokemon_type: Option<WithPokemonType2>,
    pub pokemon_level_range: Option<PokemonLevelRange>,
    pub pokemon_ban_list: Option<PokemonBanList>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithPokemonCpLimit2 {
    pub max_cp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonCaughtTimestamp {
    pub after_timestamp: String,
    pub before_timestamp: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonWhiteList {
    pub pokemon: Vec<Pokemon>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pokemon {
    pub id: String,
    #[serde(default)]
    pub forms: Vec<String>,
    pub form: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithPokemonType2 {
    pub pokemon_type: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonLevelRange {
    pub max_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonBanList {
    pub pokemon: Vec<Pokemon2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pokemon2 {
    pub id: String,
    #[serde(default)]
    pub forms: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatLeagueSettings {
    pub combat_league_template_id: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatType {
    #[serde(rename = "type")]
    pub type_field: String,
    pub nice_level_threshold: f64,
    pub great_level_threshold: f64,
    pub excellent_level_threshold: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatRankingProtoSettings {
    pub rank_level: Vec<RankLevel>,
    pub required_for_rewards: RequiredForRewards,
    pub min_rank_to_display_rating: i64,
    pub min_rating_required: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankLevel {
    pub rank_level: i64,
    pub additional_total_battles_required: Option<i64>,
    pub additional_wins_required: Option<i64>,
    pub min_rating_required: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequiredForRewards {
    pub rank_level: i64,
    pub additional_total_battles_required: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatSettings {
    pub round_duration_seconds: f64,
    pub turn_duration_seconds: f64,
    pub minigame_duration_seconds: f64,
    pub same_type_attack_bonus_multiplier: f64,
    pub fast_attack_bonus_multiplier: f64,
    pub charge_attack_bonus_multiplier: f64,
    pub defense_bonus_multiplier: f64,
    pub minigame_bonus_base_multiplier: f64,
    pub minigame_bonus_variable_multiplier: f64,
    pub max_energy: i64,
    pub defender_minigame_multiplier: f64,
    pub change_pokemon_duration_seconds: f64,
    pub minigame_submit_score_duration_seconds: f64,
    pub quick_swap_cooldown_duration_seconds: f64,
    pub charge_score_base: f64,
    pub charge_score_nice: f64,
    pub charge_score_great: f64,
    pub charge_score_excellent: f64,
    pub super_effective_flyout_duration_turns: i64,
    pub not_very_effective_flyout_duration_turns: i64,
    pub blocked_effective_flyout_duration_turns: i64,
    pub normal_effective_flyout_duration_turns: i64,
    pub shadow_pokemon_attack_bonus_multiplier: f64,
    pub shadow_pokemon_defense_bonus_multiplier: f64,
    pub purified_pokemon_attack_multiplier_vs_shadow: f64,
    pub ob_combat_settings_bool1: bool,
    pub ob_combat_settings_not_pushed_bool2: bool,
    pub ob_combat_settings1: ObCombatSettings1,
    pub ob_combat_settings2: ObCombatSettings2,
    pub ob_combat_settings_number1: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObCombatSettings1 {
    #[serde(rename = "obCombatSettings1Number1")]
    pub ob_combat_settings1number1: i64,
    pub enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObCombatSettings2 {
    #[serde(rename = "obCombatSettings2Bool1")]
    pub ob_combat_settings2bool1: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatStatStageSettings {
    pub minimum_stat_stage: i64,
    pub maximum_stat_stage: i64,
    pub attack_buff_multiplier: Vec<f64>,
    pub defense_buff_multiplier: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatMove {
    pub unique_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub power: Option<f64>,
    pub vfx_name: String,
    pub energy_delta: Option<i64>,
    pub buffs: Option<Buffs>,
    pub duration_turns: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Buffs {
    pub target_attack_stat_stage_change: Option<i64>,
    pub buff_activation_chance: f64,
    pub attacker_attack_stat_stage_change: Option<i64>,
    pub attacker_defense_stat_stage_change: Option<i64>,
    pub target_defense_stat_stage_change: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossGameSocialSettings {
    pub online_status_enabled_override_level: bool,
    pub niantic_profile_enabled_override_level: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObDailyAdventureIncenseSettings {
    pub enabled: bool,
    pub ob_pokeball_threshold_to_reward_loot: i64,
    pub ob_rewards: ObRewards,
    pub ob_daily_adventure_incense_reset_time: String,
    pub ob_daily_adventure_incense_settings_bool1: bool,
    pub ob_pace_multiplier: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObRewards {
    pub loot_item: Vec<LootItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootItem {
    pub item: String,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeepLinkingSettings {
    pub min_player_level_for_external_link: i64,
    pub min_player_level_for_notification_link: i64,
    pub ob_external_action: Vec<String>,
    pub ob_notification_action: Vec<String>,
    pub ob_deep_linking_setting_bool1: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObEggHatchImprovementSettings {
    pub feature_enabled: bool,
    pub ob_egg_hatch_animation_delay_ms: i64,
    pub ob_egg_hatch_animation_interuption_delay_ms: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EggTransparencySettings {
    pub enable_egg_distribution: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendProfileSettings {
    pub enable_swiping: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterSettings {
    pub spin_bonus_threshold: f64,
    pub excellent_throw_threshold: f64,
    pub great_throw_threshold: f64,
    pub nice_throw_threshold: f64,
    pub milestone_threshold: i64,
    pub ar_plus_mode_enabled: bool,
    pub ar_close_proximity_threshold: f64,
    pub ar_low_awareness_threshold: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonHomeEnergyCosts {
    pub pokemon_class: Option<String>,
    pub base: i64,
    pub shiny: i64,
    #[serde(rename = "cp1001To2000")]
    pub cp1001to2000: i64,
    #[serde(rename = "cp2001ToInf")]
    pub cp2001to_inf: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObEvolutionChainDisplaySettings {
    pub pokemon: String,
    pub ob_chain: Vec<ObChain>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObChain {
    pub ob_evolution_chain_entry: Vec<ObEvolutionChainEntry>,
    pub ob_pokedex_header: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObEvolutionChainEntry {
    pub pokemon: String,
    pub gender: Option<String>,
    pub form: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObEvolvePreviewSettings {
    pub ob_enable_evolution_preview: bool,
    pub ob_enable_mega_evolution_preview: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonExtendedSettings {
    pub unique_id: String,
    pub ob_pokemon_size_settings: ObPokemonSizeSettings,
    pub form: Option<String>,
    #[serde(default)]
    pub ob_extended_override_settings: Vec<ObExtendedOverrideSetting>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObPokemonSizeSettings {
    pub ob_pokemon_size_multiplier_scale1: Option<f64>,
    pub ob_pokemon_size_multiplier_scale2: Option<f64>,
    pub ob_pokemon_size_multiplier_scale3: Option<f64>,
    pub ob_pokemon_size_multiplier_scale4: Option<f64>,
    pub ob_pokemon_size_multiplier_scale5: Option<f64>,
    pub ob_pokemon_size_multiplier_scale6: Option<f64>,
    pub ob_pokemon_size_multiplier_scale7: Option<f64>,
    pub ob_pokemon_size_multiplier_scale8: Option<f64>,
    pub ob_pokemon_size_multiplier_scale9: Option<f64>,
    pub ob_pokemon_size_multiplier_scale10: Option<f64>,
    pub ob_pokemon_size_settings_bool2: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObExtendedOverrideSetting {
    pub temp_evolution_id: String,
    pub ob_pokemon_size_settings: ObPokemonSizeSettings2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObPokemonSizeSettings2 {
    pub ob_pokemon_size_multiplier_scale1: f64,
    pub ob_pokemon_size_multiplier_scale2: f64,
    pub ob_pokemon_size_multiplier_scale3: f64,
    pub ob_pokemon_size_multiplier_scale4: f64,
    pub ob_pokemon_size_multiplier_scale5: f64,
    pub ob_pokemon_size_multiplier_scale6: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalAddressableAssetsSettings {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExRaidSettings {
    pub minimum_ex_raid_share_level: String,
    pub undefined_ex_raid_setting: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObFeatureUnlockSettings {
    pub ob_bulk_postcard_delete_enabled: i64,
    pub ob_feature_unlock_settings_number3: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObFormsRefactorSettings {
    pub ob_forms_refactor_settings_bool1: bool,
    pub ob_forms_refactor_settings_bool2: bool,
    pub ob_forms_refactor_settings_bool3: bool,
    pub ob_enable_singular_shadow_form: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormSettings {
    pub pokemon: String,
    #[serde(default)]
    pub forms: Vec<Form>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub form: Option<String>,
    pub asset_bundle_value: Option<i64>,
    pub asset_bundle_suffix: Option<String>,
    pub is_costume: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObFortPowerUpSettings {
    pub level: String,
    pub ob_points_needed_for_level_up: Option<i64>,
    #[serde(default)]
    pub ob_power_up_reward: Vec<String>,
    pub ob_duration_of_power_up_ms: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendshipMilestoneSettings {
    pub min_points_to_reach: Option<i64>,
    pub milestone_xp_reward: i64,
    pub attack_bonus_percentage: f64,
    pub raid_ball_bonus: Option<i64>,
    pub unlocked_trading: Vec<String>,
    pub trading_discount: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeotargetedQuestSettings {
    pub enable_geotargeted_quests: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObGiftingSettings {
    pub ob_convert_items_to_stardust_when_full_enabled: bool,
    pub ob_stardust_to_reward_when_full: i64,
    pub stardust_multiplier: Vec<StardustMultiplier>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StardustMultiplier {
    pub ob_stardust_base_multiplier: f64,
    pub ob_stardust_multiplier_probability: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuiSearchSettings {
    pub gui_search_enabled: bool,
    pub max_number_recent_searches: i64,
    pub max_number_favorite_searches: i64,
    pub max_query_length: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GymBadgeSettings {
    pub target: Vec<i64>,
    pub battle_winning_score_per_defender_cp: f64,
    pub gym_defending_score_per_minute: f64,
    pub berry_feeding_score: i64,
    pub pokemon_deploy_score: i64,
    pub raid_battle_winning_score: i64,
    pub lose_all_battles_score: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GymLevel {
    pub required_experience: Vec<i64>,
    pub leader_slots: Vec<i64>,
    pub trainer_slots: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObGameMasterLanguageSettings {
    pub language: String,
    pub is_enabled: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IapCategoryDisplay {
    pub category: String,
    pub sort_order: Option<i64>,
    pub hidden: Option<bool>,
    pub name: Option<String>,
    pub display_rows: Option<i64>,
    pub banner_enabled: Option<bool>,
    pub banner_title: Option<String>,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IapSettings {
    pub daily_defender_bonus_per_pokemon: Vec<i64>,
    pub daily_defender_bonus_max_defenders: i64,
    pub daily_defender_bonus_currency: Vec<String>,
    pub min_time_between_claims_ms: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncidentPrioritySettings {
    pub incident_priority: Vec<IncidentPriority>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncidentPriority {
    pub priority: i64,
    pub display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObInvasionCharacterSettings {
    pub ob_invasion_character: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokestopInvasionAvailabilitySettings {
    pub availability_start_minute: i64,
    pub availability_end_minute: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventorySettings {
    pub max_pokemon: i64,
    pub max_bag_items: i64,
    pub base_pokemon: i64,
    pub base_bag_items: i64,
    pub base_eggs: i64,
    pub max_team_changes: i64,
    pub team_change_item_reset_period_in_days: String,
    pub max_item_boost_duration_ms: String,
    pub enable_eggs_not_inventory: bool,
    pub special_egg_overflow_spots: i64,
    pub ob_enable_raid_pass_overflow: bool,
    pub ob_base_postcard_storage: i64,
    pub ob_max_postcard_storage: i64,
    #[serde(rename = "obEvolutionStoneAMaxCount")]
    pub ob_evolution_stone_amax_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSettings {
    pub item_id: String,
    pub item_type: String,
    pub category: String,
    pub drop_trainer_level: Option<i64>,
    pub food: Option<Food>,
    pub potion: Option<Potion>,
    pub stardust_boost: Option<StardustBoost>,
    pub revive: Option<Revive>,
    pub inventory_upgrade: Option<InventoryUpgrade>,
    pub ignore_inventory_space: Option<bool>,
    pub xp_boost: Option<XpBoost>,
    pub incident_ticket: Option<IncidentTicket>,
    pub egg_incubator: Option<EggIncubator>,
    pub incense: Option<Incense>,
    pub global_event_ticket: Option<GlobalEventTicket>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Food {
    #[serde(default)]
    pub item_effect: Vec<String>,
    #[serde(default)]
    pub item_effect_percent: Vec<f64>,
    pub growth_percent: Option<f64>,
    pub berry_multiplier: Option<f64>,
    pub remote_berry_multiplier: Option<f64>,
    pub num_buddy_affection_points: Option<i64>,
    pub map_duration_ms: Option<String>,
    pub time_full_duration_ms: Option<String>,
    pub num_buddy_hunger_points: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Potion {
    pub sta_amount: Option<i64>,
    pub sta_percent: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StardustBoost {
    pub stardust_multiplier: f64,
    pub boost_duration_ms: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Revive {
    pub sta_percent: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryUpgrade {
    pub additional_storage: i64,
    pub upgrade_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XpBoost {
    pub xp_multiplier: f64,
    pub boost_duration_ms: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncidentTicket {
    pub upgrade_requirement_count: Option<i64>,
    pub upgraded_item: Option<String>,
    pub ignore_full_inventory: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EggIncubator {
    pub incubator_type: String,
    pub uses: Option<i64>,
    pub distance_multiplier: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Incense {
    pub incense_lifetime_seconds: i64,
    pub spawn_table_probability: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalEventTicket {
    pub event_start_time: String,
    pub event_end_time: String,
    pub item_bag_description_key: String,
    pub ob_ticket1: Option<String>,
    pub ob_ticket_shop_image_url: Option<String>,
    pub client_event_start_time_utc_ms: String,
    pub client_event_end_time_utc_ms: String,
    pub ob_is_ticket_eligible_for_gifting: Option<bool>,
    pub ob_ticket_to_gift: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemInventoryUpdateSettings {
    pub ob_item_category_settings: Vec<ObItemCategorySetting>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObItemCategorySetting {
    pub category: Vec<String>,
    pub category_name: String,
    pub sort_oder: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObLanguageSelectorSettings {
    pub ob_language_selector_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenSettings {
    pub url: String,
    pub display_after_timestamp_ms: String,
    pub color_settings: ColorSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorSettings {
    #[serde(rename = "warning_text")]
    pub warning_text: String,
    #[serde(rename = "progress_background")]
    pub progress_background: String,
    #[serde(rename = "progress_bar_left")]
    pub progress_bar_left: String,
    #[serde(rename = "progress_bar_right")]
    pub progress_bar_right: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitedPurchaseSkuSettings {
    pub purchase_limit: i64,
    pub version: Option<i64>,
    pub chrono_unit: Option<String>,
    pub loot_table_id: Option<String>,
    pub reset_interval: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LuckyPokemonSettings {
    pub power_up_stardust_discount_percent: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapDisplaySettings {
    pub show_enhanced_sky: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObInteractionRangeSettings {
    pub interaction_range_meters: f64,
    pub far_interaction_range_meters: f64,
    pub remote_interaction_range_meters: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObMegaLevelSettings {
    pub level: Option<i64>,
    pub pokemon_id: Option<String>,
    pub ob_mega_level_unlock_settings: ObMegaLevelUnlockSettings,
    pub ob_mega_level_cooldown_settings: ObMegaLevelCooldownSettings,
    pub ob_mega_level_perks: ObMegaLevelPerks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObMegaLevelUnlockSettings {
    pub ob_mega_evolutions_required_to_unlock: Option<i64>,
    #[serde(rename = "obGameMasterSettings2Message1Number2")]
    pub ob_game_master_settings2message1number2: i64,
    #[serde(rename = "obGameMasterSettings2Message1Number3")]
    pub ob_game_master_settings2message1number3: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObMegaLevelCooldownSettings {
    pub duration_ms: String,
    pub ob_max_mega_candy_required: i64,
    #[serde(rename = "obGameMasterSettings2Message2Number3")]
    pub ob_game_master_settings2message2number3: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObMegaLevelPerks {
    pub ob_mega_perk_attack_boost_from_mega_different_type: f64,
    pub ob_mega_perk_attack_boost_from_mega_same_type: f64,
    pub ob_mega_perk_active_mega_bonus_catch_candy: i64,
    pub ob_mega_perk_xp_catch_bonus: Option<i64>,
    pub ob_mega_perk_xl_candy_bonus_chance: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MegaEvoSettings {
    pub evolution_length_ms: String,
    pub attack_boost_from_mega_different_type: f64,
    pub attack_boost_from_mega_same_type: f64,
    pub max_candy_hoard_size: i64,
    pub enable_buddy_walking_mega_energy_award: bool,
    pub active_mega_bonus_catch_candy: i64,
    pub ob_mega_level_settings_shared_bool1: bool,
    pub ob_mega_level_settings_shared_bool2: bool,
    pub ob_max_mega_levels: i64,
    pub ob_mega_evo_settings_number2: Option<i64>,
    pub ob_mega_level_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonodepthSettings {
    pub enable_occlusions: bool,
    pub occlusions_default_on: bool,
    pub occlusions_toggle_visible: bool,
    pub enable_ground_suppression: bool,
    pub min_ground_suppression_thresh: f64,
    pub suppression_channel_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsFeedClientSettings {
    pub is_news_feed_polling_enabled: bool,
    pub get_news_feed_polling_rate_minutes: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingSettings {
    pub ob_onboarding_settings_number1: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingV2Settings {
    pub pokedex_id: Vec<String>,
    pub egg_km_until_hatch: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartyRecommendationSettings {
    pub mode: String,
    pub variance: f64,
    pub third_move_weight: f64,
    pub mega_evo_combat_rating_scale: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObPhotoSettings {
    pub ob_resolution_save_multiplier: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatypusRolloutSettings {
    #[serde(rename = "buddyV2MinPlayerLevel")]
    pub buddy_v2min_player_level: i64,
    pub buddy_multiplayer_min_player_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLevel {
    pub rank_num: Vec<i64>,
    pub required_experience: Vec<i64>,
    pub cp_multiplier: Vec<f64>,
    pub max_egg_player_level: i64,
    pub max_encounter_player_level: i64,
    pub max_quest_encounter_player_level: i64,
    pub ob_max_mega_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokecoinPurchaseDisplayGmt {
    pub feature_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObPokedexCategoriesSettings {
    pub feature_enabled: bool,
    pub ob_special_categories: Vec<ObSpecialCategory>,
    pub ob_pokedex_categories_settings_bool1: bool,
    pub ob_enable_pokedex_search: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObSpecialCategory {
    pub ob_pokedex_category: String,
    pub ob_category_obtained_unlock_requirement: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokedexSizeStatsSettings {
    pub ob_pokedex_size_stat_feature_enabled: bool,
    pub ob_pokemon_size_catch_requirement_to_unlock_stats: i64,
    pub ob_pokemon_weight_catch_requirement_to_unlock_stats: i64,
    pub ob_pokedex_size_stats_settings_float1: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonHomeSettings {
    pub player_min_level: i64,
    pub transporter_max_energy: i64,
    pub energy_sku_id: String,
    pub transporter_energy_gain_per_hour: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonScaleSettings {
    pub pokemon_scale_mode: Option<String>,
    pub min_height: f64,
    pub max_height: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonTagSettings {
    pub min_player_level_for_pokemon_tagging: i64,
    pub color_binding: Vec<ColorBinding>,
    pub max_num_tags_allowed: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorBinding {
    pub color: String,
    pub hex_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeEffective {
    pub attack_scalar: Vec<f64>,
    pub attack_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonUpgrades {
    pub upgrades_per_level: i64,
    pub allowed_levels_above_player: i64,
    pub candy_cost: Vec<i64>,
    pub stardust_cost: Vec<i64>,
    pub shadow_stardust_multiplier: f64,
    pub shadow_candy_multiplier: f64,
    pub purified_stardust_multiplier: f64,
    pub purified_candy_multiplier: f64,
    pub max_normal_upgrade_level: i64,
    pub default_cp_boost_additional_level: i64,
    pub xl_candy_min_player_level: i64,
    pub xl_candy_cost: Vec<i64>,
    pub ob_max_mega_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObPopupControlSettings {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObPostCardCollectionSettings {
    pub enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObPowerUpPoiSettings {
    pub ob_min_player_level_for_scanning: i64,
    pub ob_points_multiplier: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObPushGatewaySettings {
    pub ob_push_gateway_min_level1: i64,
    pub ob_push_gateway_min_level2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestEvolutionSettings {
    pub enable_quest_evolutions: bool,
    pub enable_walking_quest_evolutions: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestSettings {
    pub quest_type: String,
    pub daily_quest: DailyQuest,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyQuest {
    pub buckets_per_day: i64,
    pub streak_length: i64,
    pub bonus_multiplier: Option<f64>,
    pub streak_bonus_multiplier: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaidSettings {
    pub remote_raid_enabled: bool,
    pub max_remote_raid_passes: i64,
    pub remote_damage_modifier: f64,
    pub remote_raids_min_player_level: i64,
    pub max_num_friend_invites: i64,
    pub friend_invite_cutoff_time_sec: i64,
    pub can_invite_friends_in_person: bool,
    pub can_invite_friends_remotely: bool,
    pub max_players_per_lobby: i64,
    pub max_remote_players_per_lobby: i64,
    pub invite_cooldown_duration_millis: String,
    pub max_num_friend_invites_per_action: i64,
    pub unsupported_raid_levels_for_friend_invites: Vec<String>,
    pub unsupported_remote_raid_levels: Vec<String>,
    pub ob_raid_client_setting: Vec<ObRaidClientSetting>,
    pub ob_raid_client_setting2: ObRaidClientSetting2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObRaidClientSetting {
    pub raid_level: String,
    pub ob_raid_client_setting_string1: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObRaidClientSetting2 {
    #[serde(rename = "obRaidClientSettings2Bool1")]
    pub ob_raid_client_settings2bool1: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecomendedSearchSettings {
    pub search_label: String,
    pub search_key: Option<String>,
    pub append_search_string: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferralSettings {
    pub feature_enabled: bool,
    pub recent_features: Vec<RecentFeature>,
    pub add_referrer_grace_period_ms: String,
    pub min_num_days_without_session_for_lapsed_player: i64,
    pub ob_deep_link_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentFeature {
    pub icon_type: String,
    pub feature_name: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteCreationSettings {
    pub max_open_routes: i64,
    pub min_stops_amount: i64,
    pub max_stops_amount: i64,
    pub max_total_distance_m: f64,
    pub min_distance_between_stops_m: f64,
    pub max_distance_between_stops_m: f64,
    pub max_distance_between_checkpoints_m: f64,
    pub max_name_length: i64,
    pub max_description_length: i64,
    pub min_player_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteDiscoverySettings {
    pub nearby_visible_radius_meters: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutePlaySettings {
    pub min_player_level: i64,
    pub route_cooldown_minutes: i64,
    pub route_expiration_minutes: i64,
    pub route_pause_distance_m: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObRouteStampCategorySettings {
    pub category: String,
    pub ob_route_stamp_category_number1: i64,
    pub sort_order: i64,
    pub ob_is_route_stamp_category_default: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObSharedMoveSettings {
    pub sta_percent: f64,
    pub atk_percent: f64,
    pub def_percent: f64,
    pub duration_s: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmeargleMovesSettings {
    pub quick_moves: Vec<String>,
    pub cinematic_moves: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenderSettings {
    pub pokemon: String,
    pub gender: Gender,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gender {
    pub male_percent: Option<f64>,
    pub female_percent: Option<f64>,
    pub genderless_percent: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SponsoredGeofenceGiftSettings {
    pub gift_persistence_time_ms: i64,
    pub map_presentation_time_ms: i64,
    pub enable_sponsored_geofence_gift: bool,
    pub fullscreen_disable_exit_button_time_ms: i64,
    pub balloon_gift_settings: BalloonGiftSettings,
    pub ob_sponsored_geofence_gift_settings_bool1: bool,
    pub ob_sponsored_geofence_gift_settings: Option<ObSponsoredGeofenceGiftSettings>,
    pub ob_sponsored_geofence_gift_details: Option<ObSponsoredGeofenceGiftDetails>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalloonGiftSettings {
    pub enable_balloon_gift: bool,
    pub balloon_auto_dismiss_time_ms: i64,
    pub get_wasabi_ad_rpc_interval_ms: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObSponsoredGeofenceGiftSettings {
    pub ob_sponsored_geofence_gift_string1: String,
    pub ob_sponsored_geofence_gift_string2: String,
    pub ob_sponsored_geofence_gift_string3: String,
    pub ob_sponsored_geofence_gift_string4: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObSponsoredGeofenceGiftDetails {
    pub ads_logo: String,
    pub partner_name: String,
    pub full_screen_static_image: String,
    pub title: String,
    pub description: String,
    pub cta_url: String,
    pub campaign_identifier: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickerMetadata {
    pub sticker_id: String,
    pub sticker_url: Option<String>,
    pub max_count: i64,
    pub pokemon_id: Option<String>,
    pub ob_sticker_category: Vec<String>,
    pub ob_sticker_date: i64,
    pub ob_sticker_sort_order: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IapItemDisplay {
    pub sku: String,
    pub category: String,
    pub sort_order: Option<i64>,
    pub hidden: Option<bool>,
    pub sprite_id: Option<String>,
    pub sku_enable_time: Option<String>,
    pub sku_disable_time: Option<String>,
    pub sku_enable_time_utc_ms: Option<String>,
    pub sku_disable_time_utc_ms: Option<String>,
    pub sale: Option<bool>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObInAppSurveySettings {
    pub ob_in_app_survey_number1: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemporaryEvolutionSettings {
    pub pokemon_id: String,
    pub temporary_evolutions: Vec<TemporaryEvolution>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemporaryEvolution {
    pub temporary_evolution_id: String,
    pub asset_bundle_value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObTicketGiftingSettings {
    pub min_player_level: i64,
    pub ob_max_number_of_gifts_per_day: i64,
    pub ob_ticket_gift_settings_string1: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatNpcTrainer {
    pub trainer_name: String,
    pub combat_league_template_id: String,
    pub combat_personality_id: String,
    pub avatar: Avatar2,
    pub available_pokemon: Vec<AvailablePokemon>,
    pub trainer_title: String,
    pub trainer_quote: String,
    pub icon_url: String,
    pub backdrop_image_bundle: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar2 {
    pub avatar: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailablePokemon {
    pub pokemon_type: String,
    pub pokemon_display: Option<PokemonDisplay>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonDisplay {
    pub form: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatNpcPersonality {
    pub personality_name: String,
    pub super_effective_chance: f64,
    pub special_chance: f64,
    pub offensive_minimum_score: f64,
    pub offensive_maximum_score: f64,
    pub defensive_minimum_score: Option<f64>,
    pub defensive_maximum_score: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonFamily {
    pub family_id: String,
    pub candy_per_xl_candy: i64,
    pub mega_evolvable_pokemon_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonSettings {
    pub pokemon_id: String,
    pub model_scale: Option<f64>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub type2: Option<String>,
    pub camera: Camera,
    pub encounter: Encounter,
    pub stats: Stats,
    #[serde(default)]
    pub quick_moves: Vec<String>,
    #[serde(default)]
    pub cinematic_moves: Vec<String>,
    #[serde(default)]
    pub animation_time: Vec<f64>,
    pub evolution_pips: Option<i64>,
    pub pokedex_height_m: f64,
    pub pokedex_weight_kg: f64,
    pub parent_pokemon_id: Option<String>,
    pub height_std_dev: f64,
    pub weight_std_dev: f64,
    pub family_id: String,
    pub candy_to_evolve: Option<i64>,
    pub km_buddy_distance: f64,
    pub buddy_size: Option<String>,
    pub model_height: f64,
    pub model_scale_v2: f64,
    pub buddy_offset_male: Option<Vec<f64>>,
    pub buddy_offset_female: Option<Vec<f64>>,
    pub buddy_scale: Option<f64>,
    pub buddy_portrait_offset: Option<Vec<f64>>,
    pub third_move: ThirdMove,
    pub is_transferable: Option<bool>,
    pub is_deployable: Option<bool>,
    pub is_tradable: Option<bool>,
    pub buddy_group_number: Option<i64>,
    pub raid_boss_distance_offset: Option<f64>,
    pub combat_player_focus_camera_angle: Option<Vec<f64>>,
    pub shadow: Option<Shadow>,
    #[serde(default)]
    pub elite_cinematic_move: Vec<String>,
    pub pokemon_class: Option<String>,
    pub form: Option<String>,
    #[serde(default)]
    pub evolution_branch: Vec<EvolutionBranch>,
    pub combat_shoulder_camera_angle: Option<Vec<f64>>,
    pub combat_default_camera_angle: Option<Vec<f64>>,
    pub combat_opponent_focus_camera_angle: Option<Vec<f64>>,
    pub combat_player_pokemon_position_offset: Option<Vec<f64>>,
    pub disable_transfer_to_pokemon_home: Option<bool>,
    pub ob_pokemon_size_setting: Option<ObPokemonSizeSetting>,
    #[serde(default)]
    pub form_change: Vec<FormChange>,
    #[serde(default)]
    pub elite_quick_move: Vec<String>,
    #[serde(default)]
    pub evolution_ids: Vec<String>,
    pub buddy_walked_mega_energy_award: Option<i64>,
    #[serde(default)]
    pub temp_evo_overrides: Vec<TempEvoOverride>,
    #[serde(default)]
    pub ob_costume_evolution: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Camera {
    pub disk_radius_m: Option<f64>,
    pub cylinder_radius_m: Option<f64>,
    pub cylinder_height_m: Option<f64>,
    pub cylinder_ground_m: Option<f64>,
    pub shoulder_mode_scale: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encounter {
    pub base_capture_rate: Option<f64>,
    pub base_flee_rate: Option<f64>,
    pub collision_radius_m: Option<f64>,
    pub collision_height_m: Option<f64>,
    pub collision_head_radius_m: Option<f64>,
    pub movement_type: Option<String>,
    pub movement_timer_s: Option<f64>,
    pub jump_time_s: Option<f64>,
    pub attack_timer_s: Option<f64>,
    pub attack_probability: Option<f64>,
    pub dodge_probability: Option<f64>,
    pub dodge_duration_s: Option<f64>,
    pub dodge_distance: Option<f64>,
    pub camera_distance: f64,
    pub min_pokemon_action_frequency_s: f64,
    pub max_pokemon_action_frequency_s: f64,
    pub bonus_candy_capture_reward: Option<i64>,
    pub bonus_stardust_capture_reward: Option<i64>,
    pub bonus_xl_candy_capture_reward: Option<i64>,
    pub ob_shadow_form_base_capture_rate: Option<f64>,
    pub ob_shadow_form_attack_probability: Option<f64>,
    pub ob_shadow_form_dodge_probability: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub base_stamina: Option<i64>,
    pub base_attack: Option<i64>,
    pub base_defense: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThirdMove {
    pub stardust_to_unlock: Option<i64>,
    pub candy_to_unlock: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shadow {
    pub purification_stardust_needed: i64,
    pub purification_candy_needed: i64,
    pub purified_charge_move: String,
    pub shadow_charge_move: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolutionBranch {
    pub evolution: Option<String>,
    pub candy_cost: Option<i64>,
    pub form: Option<String>,
    pub gender_requirement: Option<String>,
    pub only_daytime: Option<bool>,
    pub lure_item_requirement: Option<String>,
    pub no_candy_cost_via_trade: Option<bool>,
    pub only_nighttime: Option<bool>,
    pub evolution_item_requirement: Option<String>,
    pub only_upside_down: Option<bool>,
    #[serde(default)]
    pub quest_display: Vec<QuestDisplay>,
    pub ob_purification_evolution_candy_cost: Option<i64>,
    pub km_buddy_distance_requirement: Option<f64>,
    pub temporary_evolution: Option<String>,
    pub temporary_evolution_energy_cost: Option<i64>,
    pub temporary_evolution_energy_cost_subsequent: Option<i64>,
    pub ob_evolution_branch_bool2: Option<bool>,
    pub priority: Option<i64>,
    pub must_be_buddy: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestDisplay {
    pub quest_requirement_template_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObPokemonSizeSetting {
    pub ob_pokemon_size_multiplier_scale1: f64,
    pub ob_pokemon_size_multiplier_scale2: f64,
    pub ob_pokemon_size_multiplier_scale3: f64,
    pub ob_pokemon_size_multiplier_scale4: f64,
    pub ob_pokemon_size_multiplier_scale5: f64,
    pub ob_pokemon_size_multiplier_scale6: f64,
    pub ob_pokemon_size_multiplier_scale7: Option<f64>,
    pub ob_pokemon_size_multiplier_scale8: Option<f64>,
    pub ob_pokemon_size_multiplier_scale9: Option<f64>,
    pub ob_pokemon_size_multiplier_scale10: Option<f64>,
    pub ob_pokemon_size_settings_bool2: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormChange {
    pub available_form: Vec<String>,
    pub candy_cost: i64,
    pub stardust_cost: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TempEvoOverride {
    pub temp_evo_id: Option<String>,
    pub stats: Option<Stats2>,
    pub average_height_m: Option<f64>,
    pub average_weight_kg: Option<f64>,
    pub type_override1: Option<String>,
    pub type_override2: Option<String>,
    pub camera: Option<Camera2>,
    pub model_scale_v2: Option<f64>,
    pub model_height: Option<f64>,
    pub buddy_offset_male: Option<Vec<f64>>,
    pub buddy_offset_female: Option<Vec<f64>>,
    pub raid_boss_distance_offset: Option<f64>,
    pub buddy_portrait_offset: Option<Vec<f64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats2 {
    pub base_stamina: i64,
    pub base_attack: i64,
    pub base_defense: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Camera2 {
    pub cylinder_radius_m: Option<f64>,
    pub cylinder_height_m: f64,
    pub cylinder_ground_m: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveSettings {
    pub movement_id: String,
    pub animation_id: i64,
    pub pokemon_type: String,
    pub power: Option<f64>,
    pub accuracy_chance: f64,
    pub stamina_loss_scalar: Option<f64>,
    pub trainer_level_min: Option<i64>,
    pub trainer_level_max: Option<i64>,
    pub vfx_name: String,
    pub duration_ms: i64,
    pub damage_window_start_ms: i64,
    pub damage_window_end_ms: i64,
    pub energy_delta: Option<i64>,
    pub is_locked: Option<bool>,
    pub critical_chance: Option<f64>,
    pub heal_scalar: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonHomeFormReversions {
    pub pokemon_id: String,
    pub form_mapping: Vec<FormMapping>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormMapping {
    pub reverted_form: String,
    pub unauthorized_forms: Vec<String>,
    pub reverted_form_string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObVerboseCombatSetting {
    pub enabled: bool,
    pub ob_verbose_combat_settings_bool1: bool,
    pub ob_verbose_combat_settings_bool2: bool,
    pub ob_verbose_combat_settings_bool3: bool,
    pub ob_verbose_combat_settings_bool4: bool,
    #[serde(rename = "obVerboseRaidShared1Bool8")]
    pub ob_verbose_raid_shared1bool8: bool,
    #[serde(rename = "obVerboseRaidShared2Bool9")]
    pub ob_verbose_raid_shared2bool9: bool,
    #[serde(rename = "obVerboseRaidShared3Bool9")]
    pub ob_verbose_raid_shared3bool9: bool,
    #[serde(rename = "obVerboseRaidShared4Bool9")]
    pub ob_verbose_raid_shared4bool9: bool,
    pub ob_verbose_combat_settings_number1: i64,
    pub ob_verbose_combat_settings_bool5: bool,
    pub ob_verbose_combat_settings_number2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObVerboseRaidSettings {
    pub enabled: bool,
    pub ob_verbose_raid_bool1: bool,
    pub ob_verbose_raid_bool2: bool,
    pub ob_verbose_raid_bool4: bool,
    pub ob_verbose_raid_bool5: bool,
    pub ob_verbose_raid_bool6: bool,
    pub ob_verbose_raid_bool7: bool,
    #[serde(rename = "obVerboseRaidShared1Bool8")]
    pub ob_verbose_raid_shared1bool8: bool,
    #[serde(rename = "obVerboseRaidShared2Bool9")]
    pub ob_verbose_raid_shared2bool9: bool,
    #[serde(rename = "obVerboseRaidShared3Bool10")]
    pub ob_verbose_raid_shared3bool10: bool,
    #[serde(rename = "obVerboseRaidShared4Bool11")]
    pub ob_verbose_raid_shared4bool11: bool,
    pub ob_verbose_raid_bool12: bool,
    #[serde(rename = "obVerboseRaidShared5Bool13")]
    pub ob_verbose_raid_shared5bool13: bool,
    pub ob_verbose_raid_bool14: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VsSeekerClientSettings {
    pub allowed_vs_seeker_league_template_id: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VsSeekerLoot {
    pub rank_level: i64,
    pub reward: Vec<Reward>,
    pub reward_track: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reward {
    pub pokemon_reward: Option<bool>,
    pub item: Option<Item>,
    pub item_loot_table: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub count: i64,
    pub stardust: Option<bool>,
    pub item: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VsSeekerPokemonRewards {
    pub available_pokemon: Vec<AvailablePokemon2>,
    pub reward_track: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailablePokemon2 {
    pub guaranteed_limited_pokemon_reward: Option<GuaranteedLimitedPokemonReward>,
    pub unlocked_at_rank: i64,
    pub attack_iv_override: AttackIvOverride,
    pub defense_iv_override: DefenseIvOverride,
    pub stamina_iv_override: StaminaIvOverride,
    pub pokemon: Option<Pokemon4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuaranteedLimitedPokemonReward {
    pub pokemon: Pokemon3,
    pub identifier: String,
    pub per_competitive_combat_season_max_count: Option<i64>,
    pub lifetime_max_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pokemon3 {
    pub pokemon_id: String,
    pub pokemon_display: Option<PokemonDisplay2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonDisplay2 {
    pub form: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackIvOverride {
    pub range: Range,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    pub min: i64,
    pub max: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseIvOverride {
    pub range: Range2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Range2 {
    pub min: i64,
    pub max: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaminaIvOverride {
    pub range: Range3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Range3 {
    pub min: i64,
    pub max: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pokemon4 {
    pub pokemon_id: String,
    pub pokemon_display: Option<PokemonDisplay3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonDisplay3 {
    pub form: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObVsSeekerScheduleSettings {
    pub ob_vs_seeker_schedule_setting_enabled: bool,
    pub ob_vs_seeker_schedule_setting_bool2: bool,
    pub ob_vs_seeker_schedule_setting_bool3: bool,
    pub ob_vs_seeker_schedule: Vec<ObVsSeekerSchedule>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObVsSeekerSchedule {
    pub ob_vs_seeker_season_name: String,
    pub description_key: String,
    pub ob_vs_seeker_schedule_window_details: Vec<ObVsSeekerScheduleWindowDetail>,
    pub ob_vs_seeker_season_blog_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObVsSeekerScheduleWindowDetail {
    pub start_time_ms: String,
    pub end_time_ms: String,
    pub ob_vs_seeker_cups_in_window: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherAffinities {
    pub weather_condition: String,
    pub pokemon_type: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherBonusSettings {
    pub cp_base_level_bonus: i64,
    pub guaranteed_individual_values: i64,
    pub stardust_bonus_multiplier: f64,
    pub attack_bonus_multiplier: f64,
    pub raid_encounter_cp_base_level_bonus: i64,
    pub raid_encounter_guaranteed_individual_values: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdventureSyncV2Gmt {
    pub feature_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Camera3 {
    pub interpolation: Vec<String>,
    pub target_type: Vec<String>,
    pub ease_in_speed: Vec<f64>,
    pub ease_out_speed: Vec<f64>,
    pub duration_seconds: Vec<f64>,
    pub wait_seconds: Vec<f64>,
    pub transition_seconds: Vec<f64>,
    pub angle_degree: Vec<f64>,
    pub angle_offset_degree: Vec<f64>,
    pub pitch_degree: Vec<f64>,
    pub pitch_offset_degree: Vec<f64>,
    pub roll_degree: Vec<f64>,
    pub distance_meters: Vec<f64>,
    pub height_percent: Vec<f64>,
    pub vert_ctr_ratio: Vec<f64>,
    pub next_camera: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObImpressionTrackingSettings {
    pub ob_impression_tracking_settings_bool1: bool,
    pub ob_impression_tracking_settings_bool2: bool,
    pub ob_impression_tracking_settings_bool4: bool,
    pub ob_impression_tracking_settings_bool5: bool,
    pub ob_impression_tracking_settings_bool6: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveSequenceSettings {
    pub sequence: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObStickerCategorySettings {
    pub enabled: bool,
    pub ob_sticker_category: Vec<ObStickerCategory>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObStickerCategory {
    pub category: String,
    pub sort_order: i64,
    pub ob_sticker_category_enabled: bool,
    pub ob_sticker_category_icon_asset_bundle: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObTutorialSettings {
    pub ob_tutorial_settings_bool2: bool,
    pub ob_tutorial_settings_bool3: bool,
    pub ob_tutorial_settings_bool4: bool,
    pub ob_tutorial_settings_bool5: bool,
    pub ob_tutorial_settings_bool6: bool,
    pub ob_tutorial_settings_bool7: bool,
    pub ob_tutorial_settings_bool8: bool,
    pub ob_tutorial_settings_bool9: bool,
    pub ob_tutorial_settings_bool10: bool,
    pub ob_tutorial_settings_bool11: bool,
    pub ob_tutorial_complete_reward: Vec<ObTutorialCompleteReward>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObTutorialCompleteReward {
    pub ob_tutorial: String,
    pub item_reward: Vec<ItemReward>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemReward {
    pub item_id: String,
    pub amount: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObUsernameSuggestionSettings {
    pub ob_feature_enabled: bool,
    pub ob_username_suggestion_number1: i64,
    pub ob_username_suggestion_number2: i64,
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn old_url() {
        reqwest::get("https://raw.githubusercontent.com/pokemongo-dev-contrib/pokemongo-game-master/master/versions/latest/V2_GAME_MASTER.json")
            .await
            .unwrap()
            .json::<crate::Root>()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn new_url() {
        reqwest::get("https://raw.githubusercontent.com/PokeMiners/game_masters/master/latest/latest.json")
            .await
            .unwrap()
            .json::<Vec<crate::TemplateWrapper>>()
            .await
            .unwrap();
    }
}
