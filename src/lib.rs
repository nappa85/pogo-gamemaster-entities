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
    pub advanced_settings: Option<AdvancedSettings>,
    pub evolution_quest_template: Option<EvolutionQuestTemplate>,
    pub ar_telemetry_settings: Option<ArTelemetrySettings>,
    pub asset_refresh_proto: Option<AssetRefreshProto>,
    pub avatar_group_order_settings: Option<AvatarGroupOrderSettings>,
    pub avatar_customization: Option<AvatarCustomization>,
    pub level_up_reward_settings: Option<LevelUpRewardSettings>,
    pub background_mode_settings: Option<BackgroundModeSettings>,
    pub badge_settings: Option<BadgeSettings>,
    pub battle_hub_badge_settings: Option<BattleHubBadgeSettings>,
    pub battle_hub_order_settings: Option<BattleHubOrderSettings>,
    pub battle_party_settings: Option<BattlePartySettings>,
    pub battle_settings: Option<BattleSettings>,
    pub battle_visual_settings: Option<BattleVisualSettings>,
    pub beluga_pokemon_whitelist: Option<BelugaPokemonWhitelist>,
    pub boot_settings: Option<BootSettings>,
    pub buddy_activity_settings: Option<BuddyActivitySettings>,
    pub buddy_activity_category_settings: Option<BuddyActivityCategorySettings>,
    pub buddy_emotion_level_settings: Option<BuddyEmotionLevelSettings>,
    pub buddy_encounter_cameo_settings: Option<BuddyEncounterCameoSettings>,
    pub buddy_hunger_settings: Option<BuddyHungerSettings>,
    pub buddy_interaction_settings: Option<BuddyInteractionSettings>,
    pub buddy_level_settings: Option<BuddyLevelSettings>,
    pub buddy_swap_settings: Option<BuddySwapSettings>,
    pub buddy_walk_settings: Option<BuddyWalkSettings>,
    pub buff_settings: Option<BuffSettings>,
    pub bulk_healing_settings: Option<BulkHealingSettings>,
    pub butterfly_collector_settings: Option<ButterflyCollectorSettings>,
    pub campfire_settings: Option<CampfireSettings>,
    pub catch_radius_multiplier_settings: Option<CatchRadiusMultiplierSettings>,
    pub invasion_npc_display_settings: Option<InvasionNpcDisplaySettings>,
    pub combat_competitive_season_settings: Option<CombatCompetitiveSeasonSettings>,
    pub combat_league: Option<CombatLeague>,
    pub combat_league_settings: Option<CombatLeagueSettings>,
    pub combat_type: Option<CombatType>,
    pub combat_ranking_proto_settings: Option<CombatRankingProtoSettings>,
    pub combat_settings: Option<CombatSettings>,
    pub combat_stat_stage_settings: Option<CombatStatStageSettings>,
    pub combat_move: Option<CombatMove>,
    pub contest_settings: Option<ContestSettings>,
    pub conversation_settings: Option<ConversationSettings>,
    pub cross_game_social_settings: Option<CrossGameSocialSettings>,
    pub daily_adventure_incense_settings: Option<DailyAdventureIncenseSettings>,
    pub deep_linking_settings: Option<DeepLinkingSettings>,
    pub egg_hatch_improvements_settings: Option<EggHatchImprovementsSettings>,
    pub egg_transparency_settings: Option<EggTransparencySettings>,
    pub friend_profile_settings: Option<FriendProfileSettings>,
    pub encounter_settings: Option<EncounterSettings>,
    pub pokemon_home_energy_costs: Option<PokemonHomeEnergyCosts>,
    pub evolution_chain_display_settings: Option<EvolutionChainDisplaySettings>,
    pub evolve_preview_settings: Option<EvolvePreviewSettings>,
    pub pokemon_extended_settings: Option<PokemonExtendedSettings>,
    pub extended_primal_settings: Option<ExtendedPrimalSettings>,
    pub external_addressable_assets_settings: Option<ExternalAddressableAssetsSettings>,
    pub feature_unlock_level_settings: Option<FeatureUnlockLevelSettings>,
    pub forms_refactor_settings: Option<FormsRefactorSettings>,
    pub form_settings: Option<FormSettings>,
    pub fort_power_up_level_settings: Option<FortPowerUpLevelSettings>,
    pub friendship_milestone_settings: Option<FriendshipMilestoneSettings>,
    pub geotargeted_quest_settings: Option<GeotargetedQuestSettings>,
    pub gifting_settings: Option<GiftingSettings>,
    pub gui_search_settings: Option<GuiSearchSettings>,
    pub gym_badge_settings: Option<GymBadgeSettings>,
    pub gym_level: Option<GymLevel>,
    pub haptics_settings: Option<HapticsSettings>,
    pub language_settings: Option<LanguageSettings>,
    pub iap_category_display: Option<IapCategoryDisplay>,
    pub iap_settings: Option<IapSettings>,
    pub incident_priority_settings: Option<IncidentPrioritySettings>,
    pub incident_visibility_settings: Option<IncidentVisibilitySettings>,
    pub incubator_flow_settings: Option<IncubatorFlowSettings>,
    pub pokestop_invasion_availability_settings: Option<PokestopInvasionAvailabilitySettings>,
    pub inventory_settings: Option<InventorySettings>,
    pub item_settings: Option<ItemSettings>,
    pub item_inventory_update_settings: Option<ItemInventoryUpdateSettings>,
    pub language_selector_settings: Option<LanguageSelectorSettings>,
    pub location_card_settings: Option<LocationCardSettings>,
    pub loading_screen_settings: Option<LoadingScreenSettings>,
    pub location_card_feature_settings: Option<LocationCardFeatureSettings>,
    pub limited_purchase_sku_settings: Option<LimitedPurchaseSkuSettings>,
    pub lucky_pokemon_settings: Option<LuckyPokemonSettings>,
    pub ob_main_menu_camera_button_settings: Option<ObMainMenuCameraButtonSettings>,
    pub map_display_settings: Option<MapDisplaySettings>,
    pub map_objects_interaction_range_settings: Option<MapObjectsInteractionRangeSettings>,
    pub mega_evo_level_settings: Option<MegaEvoLevelSettings>,
    pub mega_evo_settings: Option<MegaEvoSettings>,
    pub monodepth_settings: Option<MonodepthSettings>,
    pub nearby_pokemon_settings: Option<NearbyPokemonSettings>,
    pub neutral_avatar_settings: Option<NeutralAvatarSettings>,
    pub news_feed_client_settings: Option<NewsFeedClientSettings>,
    pub non_combat_move_settings: Option<NonCombatMoveSettings>,
    pub onboarding_settings: Option<OnboardingSettings>,
    #[serde(rename = "onboardingV2Settings")]
    pub onboarding_v2settings: Option<OnboardingV2Settings>,
    pub party_dark_launch_settings: Option<PartyDarkLaunchSettings>,
    pub party_play_general_settings: Option<PartyPlayGeneralSettings>,
    pub party_recommendation_settings: Option<PartyRecommendationSettings>,
    pub photo_settings: Option<PhotoSettings>,
    pub platypus_rollout_settings: Option<PlatypusRolloutSettings>,
    pub player_level: Option<PlayerLevel>,
    pub pokecoin_purchase_display_gmt: Option<PokecoinPurchaseDisplayGmt>,
    pub pokedex_categories_settings: Option<PokedexCategoriesSettings>,
    pub pokedex_size_stats_system_settings: Option<PokedexSizeStatsSystemSettings>,
    pub pokemon_fx_settings: Option<PokemonFxSettings>,
    pub pokemon_home_settings: Option<PokemonHomeSettings>,
    pub pokemon_scale_settings: Option<PokemonScaleSettings>,
    pub pokemon_tag_settings: Option<PokemonTagSettings>,
    pub type_effective: Option<TypeEffective>,
    pub pokemon_upgrades: Option<PokemonUpgrades>,
    pub popup_control_settings: Option<PopupControlSettings>,
    pub postcard_collection_settings: Option<PostcardCollectionSettings>,
    pub power_up_pokestops_settings: Option<PowerUpPokestopsSettings>,
    pub primal_evo_settings: Option<PrimalEvoSettings>,
    pub vps_event_settings: Option<VpsEventSettings>,
    pub quest_evolution_settings: Option<QuestEvolutionSettings>,
    pub quest_settings: Option<QuestSettings>,
    pub raid_settings: Option<RaidSettings>,
    pub raid_lobby_counter_settings: Option<RaidLobbyCounterSettings>,
    pub recomended_search_settings: Option<RecomendedSearchSettings>,
    pub referral_settings: Option<ReferralSettings>,
    pub squash_settings: Option<SquashSettings>,
    pub routes_nearby_notif_settings: Option<RoutesNearbyNotifSettings>,
    pub routes_party_play_interop_settings: Option<RoutesPartyPlayInteropSettings>,
    pub route_badge_settings: Option<RouteBadgeSettings>,
    pub route_creation_settings: Option<RouteCreationSettings>,
    pub route_discovery_settings: Option<RouteDiscoverySettings>,
    pub route_npc_gift_settings: Option<RouteNpcGiftSettings>,
    pub route_play_settings: Option<RoutePlaySettings>,
    pub route_stamp_category_settings: Option<RouteStampCategorySettings>,
    pub shared_move_settings: Option<SharedMoveSettings>,
    pub shared_non_combat_move_settings: Option<SharedNonCombatMoveSettings>,
    pub smeargle_moves_settings: Option<SmeargleMovesSettings>,
    pub gender_settings: Option<GenderSettings>,
    pub sponsored_geofence_gift_settings: Option<SponsoredGeofenceGiftSettings>,
    pub sticker_metadata: Option<StickerMetadata>,
    pub iap_item_display: Option<IapItemDisplay>,
    pub style_shop_settings: Option<StyleShopSettings>,
    pub in_app_survey_settings: Option<InAppSurveySettings>,
    pub tappable_settings: Option<TappableSettings>,
    pub temporary_evolution_settings: Option<TemporaryEvolutionSettings>,
    pub ticket_gifting_settings: Option<TicketGiftingSettings>,
    pub today_view_settings: Option<TodayViewSettings>,
    pub combat_npc_trainer: Option<CombatNpcTrainer>,
    pub combat_npc_personality: Option<CombatNpcPersonality>,
    pub pokemon_family: Option<PokemonFamily>,
    pub pokemon_settings: Option<PokemonSettings>,
    pub move_settings: Option<MoveSettings>,
    pub pokemon_home_form_reversions: Option<PokemonHomeFormReversions>,
    pub verbose_log_combat_settings: Option<VerboseLogCombatSettings>,
    pub verbose_log_raid_settings: Option<VerboseLogRaidSettings>,
    pub vs_seeker_client_settings: Option<VsSeekerClientSettings>,
    pub vs_seeker_loot: Option<VsSeekerLoot>,
    pub vs_seeker_pokemon_rewards: Option<VsSeekerPokemonRewards>,
    pub vs_seeker_schedule_settings: Option<VsSeekerScheduleSettings>,
    pub weather_affinities: Option<WeatherAffinities>,
    pub weather_bonus_settings: Option<WeatherBonusSettings>,
    #[serde(rename = "adventureSyncV2Gmt")]
    pub adventure_sync_v2gmt: Option<AdventureSyncV2Gmt>,
    pub camera: Option<Camera3>,
    pub impression_tracking_settings: Option<ImpressionTrackingSettings>,
    pub move_sequence_settings: Option<MoveSequenceSettings>,
    pub sticker_category_settings: Option<StickerCategorySettings>,
    pub tutorial_settings: Option<TutorialSettings>,
    pub username_suggestion_settings: Option<UsernameSuggestionSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressablePokemonSettings {
    pub addressable_pokemon_ids: Vec<String>,
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
pub struct AdvancedSettings {
    pub download_all_assets_enabled: bool,
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
    pub with_opponent_pokemon_battle_status: Option<WithOpponentPokemonBattleStatus>,
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
pub struct WithOpponentPokemonBattleStatus {
    pub require_defeat: bool,
    pub opponent_pokemon_type: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Display {
    pub description: String,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArTelemetrySettings {
    pub measure_battery: bool,
    pub battery_sampling_interval_ms: i64,
    pub measure_framerate: bool,
    pub framerate_sampling_interval_ms: i64,
    pub percentage_sessions_to_sample: f64,
    pub enable_ardk_telemetry: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetRefreshProto {
    pub string_refresh_seconds: i64,
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
    pub new_tag_enabled: Option<bool>,
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
    #[serde(default)]
    pub set_names: Vec<String>,
    pub unlock_badge_type: Option<String>,
    pub unlock_badge_level: Option<i64>,
    pub unlock_player_level: Option<i64>,
    pub set_prime_item: Option<bool>,
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
    pub weekly_fitness_goal_reminder_km: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadgeSettings {
    pub badge_type: String,
    pub badge_rank: i64,
    pub targets: Vec<i64>,
    pub event_badge: Option<bool>,
    pub capture_reward: Option<Vec<CaptureReward>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureReward {
    #[serde(default)]
    pub reward_types: Vec<String>,
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
    pub enable_battle_party_saving: bool,
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
pub struct BattleVisualSettings {
    pub enhancements_enabled: bool,
    pub crowd_texture_asset: String,
    pub banner_texture_asset: String,
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
pub struct BootSettings {
    #[serde(rename = "bootSequenceV2Enabled")]
    pub boot_sequence_v2enabled: bool,
    pub boot_lazy_inject_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuddyActivitySettings {
    pub activity: String,
    pub activity_category: String,
    pub max_times_per_day: i64,
    pub num_points_per_action: i64,
    pub num_emotion_points_per_action: i64,
    pub emotion_cooldown_duration_ms: Option<String>,
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
    pub enable_swap_free_evolution: bool,
    pub ob_enable_quick_swap_button: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuddyWalkSettings {
    pub km_required_per_affection_point: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuffSettings {
    pub friendship_lucky_egg_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkHealingSettings {
    pub enabled: bool,
    pub max_pokemons_per_heal: i64,
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
pub struct CampfireSettings {
    pub campfire_enabled: bool,
    pub map_buttons_enabled: bool,
    pub catch_card_enabled: bool,
    pub ar_catch_card_enabled: bool,
    pub catch_card_available_seconds: i64,
    pub catch_card_share_campfire_enabled: bool,
    pub ar_catch_card_share_campfire_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatchRadiusMultiplierSettings {
    pub catch_radius_multiplier_settings_enabled: bool,
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
    pub custom_incident_music: Option<String>,
    pub tutorial_on_loss_string: Option<String>,
    pub is_male: Option<bool>,
    pub tips_type: Option<String>,
    pub backdrop_image_bundle: Option<String>,
    pub custom_combat_music: Option<String>,
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
    pub allow_temp_evos: Option<bool>,
    #[serde(default)]
    pub combat_experiment: Vec<String>,
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
    pub before_timestamp: String,
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
    pub combat_experiment: Vec<String>,
    pub show_quick_swap_buttons_during_countdown: bool,
    pub ob_combat_settings_not_pushed_bool2: bool,
    pub clock_sync_settings: ClockSyncSettings,
    pub combat_feature_flags: CombatFeatureFlags,
    pub flyin_duration_turns: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClockSyncSettings {
    pub sync_attempt_count: i64,
    pub enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatFeatureFlags {
    pub real_device_time_enabled: bool,
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
    pub duration_turns: Option<i64>,
    pub buffs: Option<Buffs>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Buffs {
    pub attacker_defense_stat_stage_change: Option<i64>,
    pub buff_activation_chance: f64,
    pub target_defense_stat_stage_change: Option<i64>,
    pub target_attack_stat_stage_change: Option<i64>,
    pub attacker_attack_stat_stage_change: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContestSettings {
    pub is_feature_enabled: bool,
    pub player_contest_max_entries: i64,
    pub contest_limits: Vec<ContestLimit>,
    pub default_contest_max_entries: i64,
    pub min_cooldown_before_season_end_ms: String,
    pub contest_warmup_and_cooldown_durations_ms: Vec<ContestWarmupAndCooldownDurationsM>,
    pub default_cycle_warmup_duration_ms: String,
    pub default_cycle_cooldown_duration_ms: String,
    pub max_catch_prompt_range: f64,
    pub catch_prompt_timeout_ms: f64,
    pub contest_score_coefficient: Vec<ContestScoreCoefficient>,
    pub contest_length_thresholds: Vec<ContestLengthThreshold>,
    pub is_friends_display_enabled: bool,
    #[serde(rename = "isV2FeatureEnabled")]
    pub is_v2feature_enabled: bool,
    pub is_anticheat_removal_enabled: bool,
    pub is_normalized_score_to_species: bool,
    #[serde(rename = "isV2FocusesEnabled")]
    pub is_v2focuses_enabled: bool,
    pub is_contest_in_nearby_menu: bool,
    pub is_pokemon_scalar_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContestLimit {
    pub contest_metric: ContestMetric,
    pub contest_occurrence: String,
    pub per_contest_max_entries: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContestMetric {
    pub pokemon_metric: String,
    pub ranking_standard: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContestWarmupAndCooldownDurationsM {
    pub contest_occurrence: String,
    pub cycle_warmup_duration_ms: String,
    pub cycle_cooldown_duration_ms: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContestScoreCoefficient {
    pub pokemon_size: PokemonSize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonSize {
    pub height_coefficient: f64,
    pub weight_coefficient: f64,
    pub iv_coefficient: f64,
    pub xxl_adjustment_factor: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContestLengthThreshold {
    pub length: String,
    pub min_duration_ms: String,
    pub max_duration_ms: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationSettings {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossGameSocialSettings {
    pub online_status_enabled_override_level: bool,
    pub niantic_profile_enabled_override_level: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyAdventureIncenseSettings {
    pub enabled: bool,
    pub pokeball_grant_threshold: i64,
    pub pokeball_grant: PokeballGrant,
    pub local_delivery_time: String,
    pub enable_push_notification: bool,
    pub push_notification_hour_of_day: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokeballGrant {
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
    pub actions_that_ignore_min_level: Vec<String>,
    pub actions_that_execute_before_map_loads: Vec<String>,
    pub ios_action_button_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EggHatchImprovementsSettings {
    pub feature_enabled: bool,
    pub boot_delay_ms: i64,
    pub raid_invite_hard_cap_ms: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EggTransparencySettings {
    pub enable_egg_distribution: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendProfileSettings {
    pub enable_trainer_code_tab_v2: Option<bool>,
    pub enable_swiping: Option<bool>,
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
pub struct EvolutionChainDisplaySettings {
    pub pokemon: String,
    #[serde(default)]
    pub evolution_chains: Vec<EvolutionChain>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolutionChain {
    pub evolution_infos: Vec<EvolutionInfo>,
    pub header_message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolutionInfo {
    pub pokemon: String,
    pub form: Option<String>,
    pub gender: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolvePreviewSettings {
    pub enable_normal_evolve_preview: bool,
    pub enable_mega_evolve_preview: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonExtendedSettings {
    pub unique_id: String,
    pub size_settings: SizeSettings,
    pub form: Option<String>,
    #[serde(default)]
    pub temp_evo_overrides: Vec<TempEvoOverride>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SizeSettings {
    pub xxs_lower_bound: f64,
    pub xs_lower_bound: f64,
    pub m_lower_bound: f64,
    pub m_upper_bound: f64,
    pub xl_upper_bound: f64,
    pub xxl_upper_bound: f64,
    pub disable_pokedex_record_display_for_forms: Option<bool>,
    pub xxs_scale_multiplier: Option<f64>,
    pub xs_scale_multiplier: Option<f64>,
    pub xl_scale_multiplier: Option<f64>,
    pub xxl_scale_multiplier: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TempEvoOverride {
    pub temp_evo_id: String,
    pub size_settings: SizeSettings2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SizeSettings2 {
    pub xxs_lower_bound: f64,
    pub xs_lower_bound: f64,
    pub m_lower_bound: f64,
    pub m_upper_bound: f64,
    pub xl_upper_bound: f64,
    pub xxl_upper_bound: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedPrimalSettings {
    pub extended_primals_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalAddressableAssetsSettings {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureUnlockLevelSettings {
    pub lures_unlock_level: i64,
    pub rare_candy_conversion_unlock_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormsRefactorSettings {
    #[serde(rename = "enableShadowV2Gmts")]
    pub enable_shadow_v2gmts: bool,
    pub read_from_new_pokedex_entry_fields: bool,
    pub validate_no_shadows_in_quest_or_invasion_gmts: bool,
    pub validate_no_shadow_or_purified_in_gmts: bool,
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
pub struct FortPowerUpLevelSettings {
    pub level: String,
    pub min_power_up_points_required: Option<i64>,
    #[serde(default)]
    pub powerup_level_rewards: Vec<String>,
    pub additional_level_powerup_duration_ms: Option<i64>,
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
pub struct GiftingSettings {
    pub enable_gift_to_stardust: bool,
    pub stardust_per_gift: i64,
    pub stardust_multiplier: Vec<StardustMultiplier>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StardustMultiplier {
    pub multiplier: f64,
    pub random_weight: f64,
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
pub struct HapticsSettings {
    pub advanced_haptics_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageSettings {
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
    pub prohibit_purchase_in_test_envirnment: bool,
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
pub struct IncidentVisibilitySettings {
    pub hide_incident_for_character: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncubatorFlowSettings {
    pub main_map_icon_enabled: bool,
    pub pokemon_page_icon_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokestopInvasionAvailabilitySettings {
    pub availability_start_minute: String,
    pub availability_end_minute: String,
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
    pub can_raid_pass_overflow_bag_space: bool,
    pub base_postcards: i64,
    pub max_postcards: i64,
    #[serde(rename = "maxStoneACount")]
    pub max_stone_acount: i64,
    pub postcard_expansion_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSettings {
    pub item_id: String,
    pub item_type: String,
    pub category: String,
    pub drop_trainer_level: Option<i64>,
    pub food: Option<Food>,
    pub item_cap: Option<i64>,
    pub global_event_ticket: Option<GlobalEventTicket>,
    pub potion: Option<Potion>,
    pub stardust_boost: Option<StardustBoost>,
    pub incident_ticket: Option<IncidentTicket>,
    pub revive: Option<Revive>,
    pub inventory_upgrade: Option<InventoryUpgrade>,
    pub ignore_inventory_space: Option<bool>,
    pub xp_boost: Option<XpBoost>,
    pub egg_incubator: Option<EggIncubator>,
    pub incense: Option<Incense>,
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
pub struct GlobalEventTicket {
    pub event_start_time: String,
    pub event_end_time: String,
    pub item_bag_description_key: String,
    pub client_event_start_time_utc_ms: String,
    pub client_event_end_time_utc_ms: String,
    pub ticket_item: Option<String>,
    pub event_banner_url: Option<String>,
    pub background_image_url: Option<String>,
    pub giftable: Option<bool>,
    pub gift_item: Option<String>,
    #[serde(rename = "displayV2Enabled")]
    pub display_v2enabled: Option<bool>,
    pub title_image_url: Option<String>,
    pub event_datetime_range_key: Option<String>,
    pub text_rewards_key: Option<String>,
    #[serde(default)]
    pub icon_rewards: Vec<IconReward>,
    pub details_link_key: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IconReward {
    #[serde(rename = "type")]
    pub type_field: String,
    pub avatar_template_id: Option<String>,
    pub item: Option<Item>,
    pub stardust: Option<i64>,
    pub pokemon_encounter: Option<PokemonEncounter>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub item: String,
    pub amount: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonEncounter {
    pub pokemon_id: String,
    pub pokemon_display: Option<PokemonDisplay>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonDisplay {
    pub form: String,
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
pub struct IncidentTicket {
    pub upgrade_requirement_count: Option<i64>,
    pub upgraded_item: Option<String>,
    pub ignore_full_inventory: Option<bool>,
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
pub struct ItemInventoryUpdateSettings {
    pub feature_enabled: bool,
    pub category_proto: Vec<CategoryProto>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryProto {
    pub category: Vec<String>,
    pub category_name: String,
    pub sort_oder: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageSelectorSettings {
    pub language_selector_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationCardSettings {
    pub location_card: String,
    pub image_url: String,
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
    #[serde(rename = "progress_background")]
    pub progress_background: String,
    #[serde(rename = "progress_bar_right")]
    pub progress_bar_right: String,
    #[serde(rename = "progress_bar_left")]
    pub progress_bar_left: String,
    #[serde(rename = "warning_text")]
    pub warning_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationCardFeatureSettings {
    pub enabled: bool,
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
pub struct ObMainMenuCameraButtonSettings {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapDisplaySettings {
    pub show_enhanced_sky: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapObjectsInteractionRangeSettings {
    pub interaction_range_meters: f64,
    pub far_interaction_range_meters: f64,
    pub remote_interaction_range_meters: f64,
    pub white_pulse_radius_meters: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MegaEvoLevelSettings {
    pub level: Option<i64>,
    pub pokemon_id: Option<String>,
    pub progression: Progression,
    pub cooldown: Cooldown,
    pub effects: Effects,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Progression {
    pub points_required: Option<i64>,
    pub points_limit_per_period: i64,
    pub points_per_mega_evo_action: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cooldown {
    pub duration_ms: String,
    pub bypass_cost_initial: i64,
    pub bypass_cost_rounding_value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Effects {
    pub different_type_attack_boost: f64,
    pub same_type_attack_boost: f64,
    pub same_type_extra_catch_candy: i64,
    pub same_type_extra_catch_xp: Option<i64>,
    pub same_type_extra_catch_candy_xl_chance: Option<f64>,
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
    pub enable_mega_level: bool,
    pub enable_mega_evolve_in_lobby: bool,
    pub num_mega_levels: i64,
    pub client_mega_cooldown_buffer_ms: i64,
    pub enable_mega_level_legacy_award: bool,
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
pub struct NearbyPokemonSettings {
    pub ob_enabled: bool,
    pub ob_nearby_pokemon_settings_bool1: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NeutralAvatarSettings {
    pub default_neutral_avatar: DefaultNeutralAvatar,
    pub female_neutral_avatar: FemaleNeutralAvatar,
    pub male_neutral_avatar: MaleNeutralAvatar,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultNeutralAvatar {
    pub articles: Articles,
    pub body_blend: BodyBlend,
    pub head_selection: HeadSelection,
    pub skin_gradient: SkinGradient,
    pub hair_gradient: HairGradient,
    pub nose_selection: NoseSelection,
    pub ear_selection: EarSelection,
    pub mouth_selection: MouthSelection,
    pub face_positions: FacePositions,
    pub eye_gradient: EyeGradient,
    pub eye_selection: EyeSelection,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Articles {
    pub hair: Hair,
    pub shirt: Shirt,
    pub pants: Pants,
    pub hat: Hat,
    pub shoes: Shoes,
    pub backpack: Backpack,
    pub gloves: Gloves,
    pub socks: Socks,
    pub pose: Pose,
    pub eyebrow: Eyebrow,
    pub eyelash: Eyelash,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hair {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shirt {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pants {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hat {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shoes {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Backpack {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gloves {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Socks {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pose {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eyebrow {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eyelash {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyBlend {
    pub bust: f64,
    pub hips: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadSelection {
    pub selection: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinGradient {
    pub color_keys: Vec<ColorKey>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorKey {
    pub key_position: f64,
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HairGradient {
    pub color_keys: Vec<ColorKey2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorKey2 {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub key_position: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoseSelection {
    pub selection: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarSelection {
    pub selection: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MouthSelection {
    pub selection: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FacePositions {
    pub brow_depth: f64,
    pub brow_horizontal: f64,
    pub brow_vertical: f64,
    pub eye_depth: f64,
    pub eye_horizontal: f64,
    pub eye_vertical: f64,
    pub mouth_depth: f64,
    pub mouth_horizontal: f64,
    pub mouth_vertical: f64,
    pub nose_depth: f64,
    pub nose_vertical: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EyeGradient {
    pub color_keys: Vec<ColorKey3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorKey3 {
    pub key_position: Option<f64>,
    pub red: Option<f64>,
    pub green: Option<f64>,
    pub blue: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EyeSelection {
    pub selection: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemaleNeutralAvatar {
    pub articles: Articles2,
    pub body_blend: BodyBlend2,
    pub head_selection: HeadSelection2,
    pub skin_gradient: SkinGradient2,
    pub hair_gradient: HairGradient2,
    pub nose_selection: NoseSelection2,
    pub ear_selection: EarSelection2,
    pub mouth_selection: MouthSelection2,
    pub face_positions: FacePositions2,
    pub eye_gradient: EyeGradient2,
    pub eye_selection: EyeSelection2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Articles2 {
    pub hair: Hair2,
    pub shirt: Shirt2,
    pub pants: Pants2,
    pub hat: Hat2,
    pub shoes: Shoes2,
    pub backpack: Backpack2,
    pub gloves: Gloves2,
    pub socks: Socks2,
    pub belt: Belt,
    pub necklace: Necklace,
    pub pose: Pose2,
    pub eyebrow: Eyebrow2,
    pub eyelash: Eyelash2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hair2 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shirt2 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pants2 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hat2 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shoes2 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Backpack2 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gloves2 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Socks2 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Belt {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Necklace {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pose2 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eyebrow2 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eyelash2 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyBlend2 {
    pub bust: f64,
    pub hips: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadSelection2 {
    pub selection: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinGradient2 {
    pub color_keys: Vec<ColorKey4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorKey4 {
    pub key_position: f64,
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HairGradient2 {
    pub color_keys: Vec<ColorKey5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorKey5 {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub key_position: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoseSelection2 {
    pub selection: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarSelection2 {
    pub selection: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MouthSelection2 {
    pub selection: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FacePositions2 {
    pub brow_depth: f64,
    pub brow_horizontal: f64,
    pub brow_vertical: f64,
    pub eye_depth: f64,
    pub eye_horizontal: f64,
    pub eye_vertical: f64,
    pub mouth_depth: f64,
    pub mouth_horizontal: f64,
    pub mouth_vertical: f64,
    pub nose_depth: f64,
    pub nose_vertical: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EyeGradient2 {
    pub color_keys: Vec<ColorKey6>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorKey6 {
    pub key_position: Option<f64>,
    pub red: Option<f64>,
    pub green: Option<f64>,
    pub blue: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EyeSelection2 {
    pub selection: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaleNeutralAvatar {
    pub articles: Articles3,
    pub body_blend: BodyBlend3,
    pub head_selection: HeadSelection3,
    pub skin_gradient: SkinGradient3,
    pub hair_gradient: HairGradient3,
    pub nose_selection: NoseSelection3,
    pub ear_selection: EarSelection3,
    pub mouth_selection: MouthSelection3,
    pub face_positions: FacePositions3,
    pub eye_gradient: EyeGradient3,
    pub eye_selection: EyeSelection3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Articles3 {
    pub hair: Hair3,
    pub shirt: Shirt3,
    pub pants: Pants3,
    pub hat: Hat3,
    pub shoes: Shoes3,
    pub backpack: Backpack3,
    pub gloves: Gloves3,
    pub socks: Socks3,
    pub pose: Pose3,
    pub eyebrow: Eyebrow3,
    pub eyelash: Eyelash3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hair3 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shirt3 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pants3 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hat3 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shoes3 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Backpack3 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gloves3 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Socks3 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pose3 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eyebrow3 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eyelash3 {
    pub article_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyBlend3 {
    pub musculature: f64,
    pub shoulders: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadSelection3 {
    pub selection: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinGradient3 {
    pub color_keys: Vec<ColorKey7>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorKey7 {
    pub key_position: f64,
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HairGradient3 {
    pub color_keys: Vec<ColorKey8>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorKey8 {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub key_position: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoseSelection3 {
    pub selection: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarSelection3 {
    pub selection: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MouthSelection3 {
    pub selection: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FacePositions3 {
    pub brow_depth: f64,
    pub brow_horizontal: f64,
    pub brow_vertical: f64,
    pub eye_depth: f64,
    pub eye_horizontal: f64,
    pub eye_vertical: f64,
    pub mouth_depth: f64,
    pub mouth_horizontal: f64,
    pub mouth_vertical: f64,
    pub nose_depth: f64,
    pub nose_vertical: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EyeGradient3 {
    pub color_keys: Vec<ColorKey9>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorKey9 {
    pub blue: f64,
    pub key_position: Option<f64>,
    pub red: Option<f64>,
    pub green: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EyeSelection3 {
    pub selection: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsFeedClientSettings {
    pub news_feed_polling_enabled: bool,
    pub news_feed_polling_rate_minutes: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonCombatMoveSettings {
    pub unique_id: String,
    pub cost: Cost,
    pub bonus_effect: BonusEffect,
    pub duration_ms: String,
    pub bonus_type: String,
    pub enable_multi_use: bool,
    pub extra_duration_ms: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cost {
    pub candy_cost: i64,
    pub stardust_cost: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BonusEffect {
    pub time_bonus: Option<TimeBonus>,
    pub space_bonus: Option<SpaceBonus>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeBonus {
    pub affected_items: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceBonus {
    pub pokemon_visible_range_meters: f64,
    pub encounter_range_meters: f64,
    pub server_allowable_encounter_range_meters: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingSettings {
    pub disable_initial_ar_prompt: bool,
    pub ar_prompt_player_level: i64,
    pub adventure_sync_prompt_step: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingV2Settings {
    pub pokedex_id: Vec<String>,
    pub egg_km_until_hatch: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartyDarkLaunchSettings {
    pub rollout_players_per_billion: i64,
    pub create_or_join_wait_probability: Vec<CreateOrJoinWaitProbability>,
    pub probability_to_create_percent: i64,
    pub leave_party_probablity: Vec<LeavePartyProbablity>,
    pub update_location_enabled: bool,
    pub update_location_override_period_ms: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrJoinWaitProbability {
    pub weight: i64,
    pub wait_time_ms: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeavePartyProbablity {
    pub weight: i64,
    pub max_duration_ms: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartyPlayGeneralSettings {
    pub enabled: bool,
    pub min_player_level: i64,
    pub creation_to_start_timeout_ms: String,
    pub compliance_zones_enabled: bool,
    pub enable_party_raid_information: bool,
    pub friend_requests_enabled: bool,
    pub party_expiry_duration_ms: String,
    pub party_expiry_warning_minutes: i64,
    pub enabled_friend_status_increase: bool,
    pub restart_party_rejoin_prompt_enabled: bool,
    #[serde(rename = "partyNewQuestNotificationV2Enabled")]
    pub party_new_quest_notification_v2enabled: bool,
    pub pg_delivery_mechanic: String,
    pub party_catch_tags_enabled: bool,
    pub party_quest_encounter_reward_enabled: bool,
    pub max_stacked_encounter_reward: i64,
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
pub struct PhotoSettings {
    pub screen_capture_size: f64,
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
    pub extended_player_level_threshold: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokecoinPurchaseDisplayGmt {
    pub feature_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokedexCategoriesSettings {
    pub feature_enabled: bool,
    pub pokedex_category_settings_in_order: Vec<PokedexCategorySettingsInOrder>,
    pub client_shiny_form_check: bool,
    pub search_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokedexCategorySettingsInOrder {
    pub pokedex_category: String,
    pub milestone_goal: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokedexSizeStatsSystemSettings {
    pub display_enabled: bool,
    pub pokedex_display_pokemon_tracked_threshold: i64,
    pub record_display_pokemon_tracked_threshold: i64,
    pub num_days_new_bubble_track: f64,
    pub enable_randomized_height_and_weight_for_wild_pokemon: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonFxSettings {
    pub hiding_in_photo: bool,
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
    pub xl_candy_min_pokemon_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PopupControlSettings {
    pub hide_medal_earned_popup_unit_after_first_pokemon: bool,
    pub hide_aware_of_your_surroundings_popup: bool,
    pub hide_weather_warning_popup: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostcardCollectionSettings {
    pub enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerUpPokestopsSettings {
    pub power_up_pokestops_min_player_level: i64,
    pub validate_pokestop_on_fort_search_percent: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimalEvoSettings {
    pub common_temp_settings: CommonTempSettings,
    pub max_candy_hoard_size: i64,
    pub type_boosts: Vec<TypeBoost>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonTempSettings {
    pub evolution_length_ms: String,
    pub num_temp_evo_levels: i64,
    pub enable_buddy_walking_temp_evo_energy_award: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeBoost {
    pub pokemon_id: String,
    pub boost_type: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsEventSettings {
    pub fort_vps_events: Vec<FortVpsEvent>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FortVpsEvent {
    pub fort_id: String,
    pub start_time_ms: String,
    pub end_time_ms: String,
    pub vps_event: VpsEvent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsEvent {
    pub event_type: String,
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
    pub raid_level_music_overrides: Vec<RaidLevelMusicOverride>,
    pub raid_feature_flags: RaidFeatureFlags,
    pub boot_raid_enabled: bool,
    pub friend_requests_enabled: bool,
    pub remote_raid_distance_validation: bool,
    pub popup_time_ms: i64,
    pub failed_friend_invite_info_enabled: bool,
    pub min_players_to_boot: i64,
    pub boot_cutoff_ms: i64,
    pub boot_solo_ms: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaidLevelMusicOverride {
    pub raid_level: String,
    pub battle_music_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaidFeatureFlags {
    pub use_cached_raid_boss_pokemon: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaidLobbyCounterSettings {
    pub polling_enabled: bool,
    pub polling_interval_ms: i64,
    pub subscribe_enabled: bool,
    pub publish_enabled: bool,
    pub map_display_enabled: bool,
    pub nearby_display_enabled: bool,
    pub show_counter_radius_meters: f64,
    #[serde(rename = "subscribeS2Level")]
    pub subscribe_s2level: i64,
    pub max_count_to_update: i64,
    pub subscription_namespace: String,
    pub polling_radius_meters: f64,
    pub publish_cutoff_time_ms: i64,
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
    pub deep_link_url: String,
    pub image_share_referral_enabled: bool,
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
pub struct SquashSettings {
    pub enabled: bool,
    pub daily_squash_limit: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutesNearbyNotifSettings {
    pub max_notifs: i64,
    pub time_between_notifs_ms: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutesPartyPlayInteropSettings {
    pub consumption_interoperable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteBadgeSettings {
    pub target: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteCreationSettings {
    pub max_open_routes: i64,
    pub min_total_distance_m: f64,
    pub max_total_distance_m: f64,
    pub max_name_length: i64,
    pub max_description_length: i64,
    pub min_player_level: i64,
    pub enabled: bool,
    pub enable_immediate_route_ingestion: bool,
    pub min_breadcrumb_distance_delta_meters: i64,
    pub creation_limit_window_days: i64,
    pub creation_limit_per_window: i64,
    pub max_distance_from_anchor_pots_m: f64,
    pub max_distance_warning_distance_meters: i64,
    pub max_recording_speed_meters_per_second: i64,
    pub moderation_enabled: bool,
    pub client_breadcrumb_settings: ClientBreadcrumbSettings,
    pub duration_distance_to_speed_multiplier: f64,
    pub duration_buffer_s: i64,
    pub interaction_range_meters: i64,
    pub max_client_map_panning_distance_m: f64,
    pub resume_range_meters: i64,
    pub max_recall_count_threshold: i64,
    pub allowable_gps_drift_meters: i64,
    pub max_post_punishment_ban_time_ms: String,
    pub max_submission_count_threshold: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientBreadcrumbSettings {
    pub session_duration_m: f64,
    pub update_interval_s: f64,
    pub as_fallback_foreground_reporting_inverval_s: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteDiscoverySettings {
    pub nearby_visible_radius_meters: f64,
    pub popular_routes_fraction: f64,
    pub new_route_threshold: i64,
    pub max_routes_viewable: i64,
    pub max_client_map_panning_distance_meters: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteNpcGiftSettings {
    pub max_nearby_poi_count: i64,
    #[serde(rename = "maxS2CellQueryCount")]
    pub max_s2cell_query_count: i64,
    pub max_nearby_poi_distance_meters: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutePlaySettings {
    pub min_player_level: i64,
    pub route_expiration_minutes: i64,
    pub route_pause_distance_m: i64,
    pub bonus_active_distance_threshold_meters: i64,
    pub margin_percentage: f64,
    pub margin_minimum_meters: i64,
    pub resume_range_meters: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteStampCategorySettings {
    pub category: String,
    pub collection_size: i64,
    pub sort_order: i64,
    pub active: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedMoveSettings {
    pub shadow_third_move_unlock_stardust_multiplier: f64,
    pub shadow_third_move_unlock_candy_multiplier: f64,
    pub purified_third_move_unlock_stardust_multiplier: f64,
    pub purified_third_move_unlock_candy_multiplier: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedNonCombatMoveSettings {
    pub non_combat_move_enabled: bool,
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
    pub ob_pokemon_form: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gender {
    pub genderless_percent: Option<f64>,
    pub male_percent: Option<f64>,
    pub female_percent: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SponsoredGeofenceGiftSettings {
    pub gift_persistence_time_ms: i64,
    pub map_presentation_time_ms: i64,
    pub enable_sponsored_geofence_gift: bool,
    pub fullscreen_disable_exit_button_time_ms: i64,
    pub balloon_gift_settings: BalloonGiftSettings,
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
pub struct StickerMetadata {
    pub sticker_id: String,
    pub sticker_url: Option<String>,
    pub max_count: i64,
    pub pokemon_id: Option<String>,
    pub category: Vec<String>,
    pub release_date: Option<i64>,
    pub region_id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IapItemDisplay {
    pub sku: String,
    pub category: String,
    pub sort_order: Option<i64>,
    pub sale: Option<bool>,
    pub sku_enable_time: Option<String>,
    pub sku_disable_time: Option<String>,
    pub sku_enable_time_utc_ms: Option<String>,
    pub sku_disable_time_utc_ms: Option<String>,
    pub sprite_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub hidden: Option<bool>,
    pub image_url: Option<String>,
    pub max_level: Option<i64>,
    pub show_discount_tag: Option<bool>,
    pub show_strikethrough_price: Option<bool>,
    pub total_value: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StyleShopSettings {
    #[serde(rename = "v2Enabled")]
    pub v2enabled: bool,
    pub sets_enabled: bool,
    pub recommended_item_icon_names: Vec<String>,
    pub cart_disabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InAppSurveySettings {
    pub survey_poll_frequency_s: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TappableSettings {
    pub visible_radius_meters: f64,
    pub spawn_angle_degrees: f64,
    pub movement_respawn_threshold_meters: f64,
    pub buddy_fov_degress: f64,
    pub avg_tappables_in_view: f64,
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
pub struct TicketGiftingSettings {
    pub min_player_level: i64,
    pub daily_player_gifting_limit: i64,
    pub min_required_friendship_level: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodayViewSettings {
    pub version: i64,
    pub today_view_display_order: Vec<String>,
    pub season_view_display_order: Vec<String>,
    pub special_view_display_order: Vec<String>,
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
    pub pokemon_display: Option<PokemonDisplay2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonDisplay2 {
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
    pub pokemon_class: Option<String>,
    pub pokedex_height_m: f64,
    pub pokedex_weight_kg: f64,
    pub height_std_dev: f64,
    pub weight_std_dev: f64,
    pub family_id: String,
    pub km_buddy_distance: f64,
    pub form: Option<String>,
    pub third_move: ThirdMove,
    pub is_transferable: Option<bool>,
    pub is_tradable: Option<bool>,
    pub is_deployable: Option<bool>,
    pub model_scale: Option<f64>,
    #[serde(default)]
    pub animation_time: Vec<f64>,
    pub evolution_pips: Option<i64>,
    pub parent_pokemon_id: Option<String>,
    pub model_height: Option<f64>,
    pub model_scale_v2: Option<f64>,
    pub buddy_offset_male: Option<Vec<f64>>,
    pub buddy_offset_female: Option<Vec<f64>>,
    pub buddy_scale: Option<f64>,
    pub buddy_portrait_offset: Option<Vec<f64>>,
    pub buddy_group_number: Option<i64>,
    #[serde(default)]
    pub evolution_ids: Vec<String>,
    #[serde(default)]
    pub evolution_branch: Vec<EvolutionBranch>,
    pub buddy_size: Option<String>,
    pub raid_boss_distance_offset: Option<f64>,
    #[serde(default)]
    pub elite_cinematic_move: Vec<String>,
    pub candy_to_evolve: Option<i64>,
    pub shadow: Option<Shadow>,
    pub combat_player_focus_camera_angle: Option<Vec<f64>>,
    pub buddy_walked_mega_energy_award: Option<i64>,
    pub combat_opponent_focus_camera_angle: Option<Vec<f64>>,
    pub combat_shoulder_camera_angle: Option<Vec<f64>>,
    pub combat_default_camera_angle: Option<Vec<f64>>,
    pub combat_player_pokemon_position_offset: Option<Vec<f64>>,
    #[serde(default)]
    pub elite_quick_move: Vec<String>,
    pub disable_transfer_to_pokemon_home: Option<bool>,
    #[serde(default)]
    pub temp_evo_overrides: Vec<TempEvoOverride2>,
    #[serde(default)]
    pub form_change: Vec<FormChange>,
    pub exclusive_key_item: Option<ExclusiveKeyItem>,
    pub size_settings: Option<SizeSettings3>,
    #[serde(default)]
    pub allow_noevolve_evolution: Vec<String>,
    #[serde(default)]
    pub non_tm_cinematic_moves: Vec<String>,
    pub buddy_portrait_rotation: Option<Vec<f64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Camera {
    pub disk_radius_m: Option<f64>,
    pub cylinder_radius_m: Option<f64>,
    pub cylinder_height_m: Option<f64>,
    pub shoulder_mode_scale: Option<f64>,
    pub cylinder_ground_m: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encounter {
    pub movement_type: Option<String>,
    pub jump_time_s: Option<f64>,
    pub attack_probability: Option<f64>,
    pub dodge_probability: Option<f64>,
    pub dodge_duration_s: Option<f64>,
    pub min_pokemon_action_frequency_s: Option<f64>,
    pub max_pokemon_action_frequency_s: Option<f64>,
    pub bonus_xl_candy_capture_reward: Option<i64>,
    pub collision_radius_m: Option<f64>,
    pub collision_height_m: Option<f64>,
    pub collision_head_radius_m: Option<f64>,
    pub movement_timer_s: Option<f64>,
    pub attack_timer_s: Option<f64>,
    pub bonus_candy_capture_reward: Option<i64>,
    pub bonus_stardust_capture_reward: Option<i64>,
    pub dodge_distance: Option<f64>,
    pub camera_distance: Option<f64>,
    pub shadow_base_capture_rate: Option<f64>,
    pub shadow_attack_probability: Option<f64>,
    pub shadow_dodge_probability: Option<f64>,
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
pub struct EvolutionBranch {
    pub evolution: Option<String>,
    pub evolution_item_requirement: Option<String>,
    pub evolution_item_requirement_cost: Option<i64>,
    pub candy_cost: Option<i64>,
    pub km_buddy_distance_requirement: Option<f64>,
    #[serde(default)]
    pub quest_display: Vec<QuestDisplay>,
    pub form: Option<String>,
    pub gender_requirement: Option<String>,
    pub only_daytime: Option<bool>,
    pub priority: Option<i64>,
    pub only_dusk_period: Option<bool>,
    pub lure_item_requirement: Option<String>,
    pub temporary_evolution: Option<String>,
    pub temporary_evolution_energy_cost: Option<i64>,
    pub temporary_evolution_energy_cost_subsequent: Option<i64>,
    pub no_candy_cost_via_trade: Option<bool>,
    pub only_nighttime: Option<bool>,
    pub only_upside_down: Option<bool>,
    pub candy_cost_purified: Option<i64>,
    pub evolution_move_requirement: Option<String>,
    pub only_full_moon: Option<bool>,
    pub must_be_buddy: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestDisplay {
    pub quest_requirement_template_id: String,
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
pub struct TempEvoOverride2 {
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
    pub buddy_portrait_rotation: Option<Vec<f64>>,
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
pub struct FormChange {
    pub available_form: Vec<String>,
    pub item: Option<String>,
    pub item_cost_count: Option<i64>,
    pub candy_cost: Option<i64>,
    pub stardust_cost: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExclusiveKeyItem {
    pub item: String,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SizeSettings3 {
    pub xxs_lower_bound: f64,
    pub xs_lower_bound: f64,
    pub m_lower_bound: f64,
    pub m_upper_bound: f64,
    pub xl_upper_bound: f64,
    pub xxl_upper_bound: f64,
    pub xxs_scale_multiplier: f64,
    pub xs_scale_multiplier: f64,
    pub xl_scale_multiplier: f64,
    pub xxl_scale_multiplier: f64,
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
pub struct VerboseLogCombatSettings {
    pub enabled: bool,
    pub enable_core_combat: bool,
    pub enable_combat_challenge_setup: bool,
    pub enable_combat_vs_seeker_setup: bool,
    pub enable_web_socket: bool,
    pub enable_on_application_focus: bool,
    pub enable_on_application_pause: bool,
    pub enable_on_application_quit: bool,
    pub enable_exception_caught: bool,
    pub progress_token_priority: i64,
    pub enable_rpc_error_data: bool,
    pub client_log_decay_time_in_hours: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerboseLogRaidSettings {
    pub enabled: bool,
    pub enable_join_lobby: bool,
    pub enable_leave_lobby: bool,
    pub enable_get_raid_details: bool,
    pub enable_start_raid_battle: bool,
    pub enable_attack_raid: bool,
    pub enable_send_raid_invitation: bool,
    pub enable_on_application_focus: bool,
    pub enable_on_application_pause: bool,
    pub enable_on_application_quit: bool,
    pub enable_exception_caught: bool,
    pub enable_progress_token: bool,
    pub enable_rpc_error_data: bool,
    pub enable_client_prediction_inconsistency_data: bool,
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
    pub item: Option<Item2>,
    pub item_ranking_loot_table_count: Option<i64>,
    pub pokemon_reward: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item2 {
    pub stardust: Option<bool>,
    pub count: i64,
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
    pub pokemon_display: Option<PokemonDisplay3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonDisplay3 {
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
    pub pokemon_display: Option<PokemonDisplay4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonDisplay4 {
    pub form: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VsSeekerScheduleSettings {
    pub enable_combat_hub_main: bool,
    pub enable_combat_league_view: bool,
    pub enable_today_view: bool,
    pub season_schedules: Vec<SeasonSchedule>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeasonSchedule {
    pub season_title: String,
    pub description_key: String,
    pub vs_seeker_schedules: Vec<VsSeekerSchedule>,
    pub blog_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VsSeekerSchedule {
    pub start_time_ms: String,
    pub end_time_ms: String,
    pub vs_seeker_league_tempalte_id: Vec<String>,
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
pub struct ImpressionTrackingSettings {
    pub impression_tracking_enabled: bool,
    pub full_screen_ad_view_tracking_enabled: bool,
    pub pokestop_spinner_interaction_tracking_enabled: bool,
    pub approach_gym_tracking_enabled: bool,
    pub approach_raid_tracking_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveSequenceSettings {
    pub sequence: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickerCategorySettings {
    pub enabled: bool,
    pub sticker_category: Vec<StickerCategory>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickerCategory {
    pub category: String,
    pub sort_order: i64,
    pub active: bool,
    pub preferred_category_icon: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TutorialSettings {
    pub friends_tutorial_enabled: bool,
    pub gifts_tutorial_enabled: bool,
    pub task_help_tutorials_enabled: bool,
    pub revives_and_potions_tutorial_enabled: bool,
    pub razzberry_catch_tutorial_enabled: bool,
    pub lures_tutorial_enabled: bool,
    pub trading_tutorial_enabled: bool,
    pub lucky_trade_tutorial_enabled: bool,
    pub lucky_friend_tutorial_enabled: bool,
    pub pokemon_tagging_tutorial_enabled: bool,
    pub tutorial_item_rewards: Vec<TutorialItemReward>,
    pub type_effectiveness_tips_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TutorialItemReward {
    pub tutorial: String,
    pub item: Vec<Item3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item3 {
    pub item_id: String,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsernameSuggestionSettings {
    pub feature_enabled: bool,
    pub num_suggestions_displayed: i64,
    pub num_suggestions_generated: i64,
}

#[cfg(test)]
mod tests {
    use tokio::fs;

    async fn deserialize<T>(file: &str, url: &str)
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let body = if let Ok(body) = fs::read_to_string(file).await {
            body
        } else {
            let body = reqwest::get(url).await.unwrap().text().await.unwrap();
            fs::write(file, body.as_bytes()).await.unwrap();
            body
        };

        let jd = &mut serde_json::Deserializer::from_str(&body);
        serde_path_to_error::deserialize::<_, T>(jd).unwrap();
    }

    // #[tokio::test]
    // async fn old_url() {
    //     deserialize::<crate::Root>("V2_GAME_MASTER.json", "https://raw.githubusercontent.com/pokemongo-dev-contrib/pokemongo-game-master/master/versions/latest/V2_GAME_MASTER.json").await;
    // }

    #[tokio::test]
    async fn new_url() {
        deserialize::<Vec<crate::TemplateWrapper>>(
            "latest.json",
            "https://raw.githubusercontent.com/PokeMiners/game_masters/master/latest/latest.json",
        )
        .await;
    }
}
