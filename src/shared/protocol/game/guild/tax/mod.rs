use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use protocol::game::character::CharacterMinimalPlusLookInformations; use protocol::game::context::roleplay::BasicGuildInformations;
impl_type!(AbstractTaxCollectorListMessage, 6568, informations| Vec<TaxCollectorInformationsVariant>);
impl_type!(GameRolePlayTaxCollectorFightRequestMessage, 5954, tax_collector_id| i32);
impl_type!(GuildFightJoinRequestMessage, 5717, tax_collector_id| i32);
impl_type!(GuildFightLeaveRequestMessage, 5715, tax_collector_id| i32, character_id| VarInt);
impl_type!(GuildFightPlayersEnemiesListMessage, 5928, fight_id| i32, player_info| Vec<CharacterMinimalPlusLookInformations>);
impl_type!(GuildFightPlayersEnemyRemoveMessage, 5929, fight_id| i32, player_id| VarInt);
impl_type!(GuildFightPlayersHelpersJoinMessage, 5720, fight_id| i32, player_info| CharacterMinimalPlusLookInformations);
impl_type!(GuildFightPlayersHelpersLeaveMessage, 5719, fight_id| i32, player_id| VarInt);
impl_type!(GuildFightTakePlaceRequestMessage, 6235, base| GuildFightJoinRequestMessage, replaced_character_id| i32);
impl_type!(TaxCollectorAttackedMessage, 5918, first_name_id| VarShort, last_name_id| VarShort, world_x| i16, world_y| i16, map_id| i32, sub_area_id| VarShort, guild| BasicGuildInformations);
impl_type!(TaxCollectorAttackedResultMessage, 5635, dead_or_alive| bool, basic_infos| TaxCollectorBasicInformations, guild| BasicGuildInformations);
impl_type!(TaxCollectorErrorMessage, 5634, reason| i8);
impl_type!(TaxCollectorListMessage, 5930, base| AbstractTaxCollectorListMessage, nbcollector_max| i8, fighters_informations| Vec<TaxCollectorFightersInformation>);
impl_type!(TaxCollectorMovementAddMessage, 5917, informations| TaxCollectorInformationsVariant);
impl_type!(TaxCollectorMovementMessage, 5633, hire_or_fire| bool, basic_infos| TaxCollectorBasicInformations, player_id| VarInt, player_name| String);
impl_type!(TaxCollectorMovementRemoveMessage, 5915, collector_id| i32);
impl_type!(TaxCollectorStateUpdateMessage, 6455, unique_id| i32, state| i8);
impl_type!(TopTaxCollectorListMessage, 6565, base| AbstractTaxCollectorListMessage, is_dungeon| bool);
 use protocol::game::character::CharacterMinimalPlusLookInformationsVariant; use protocol::game::fight::ProtectedEntityWaitingForHelpInfo; use protocol::game::context::roleplay::BasicGuildInformations; use protocol::game::look::EntityLook;
impl_type!(AdditionalTaxCollectorInformations, 165, collector_caller_name| String, date| i32);
impl_type!(TaxCollectorBasicInformations, 96, first_name_id| VarShort, last_name_id| VarShort, world_x| i16, world_y| i16, map_id| i32, sub_area_id| VarShort);
impl_type!(TaxCollectorComplementaryInformations, 448);
impl_type!(TaxCollectorFightersInformation, 169, collector_id| i32, ally_characters_informations| Vec<CharacterMinimalPlusLookInformationsVariant>, enemy_characters_informations| Vec<CharacterMinimalPlusLookInformationsVariant>);
impl_type!(TaxCollectorGuildInformations, 446, base| TaxCollectorComplementaryInformations, guild| BasicGuildInformations);
impl_type!(TaxCollectorInformations, 167, unique_id| i32, firt_name_id| VarShort, last_name_id| VarShort, additional_infos| AdditionalTaxCollectorInformations, world_x| i16, world_y| i16, sub_area_id| VarShort, state| i8, look| EntityLook, complements| Vec<TaxCollectorComplementaryInformationsVariant>);
impl_type!(TaxCollectorLootInformations, 372, base| TaxCollectorComplementaryInformations, kamas| VarInt, experience| VarLong, pods| VarInt, items_value| VarInt);
impl_type!(TaxCollectorWaitingForHelpInformations, 447, base| TaxCollectorComplementaryInformations, waiting_for_help_info| ProtectedEntityWaitingForHelpInfo);