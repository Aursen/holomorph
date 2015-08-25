use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use protocol::variants::CharacterMinimalPlusLookInformationsVariant; use protocol::types::game::context::roleplay::BasicGuildInformations; use protocol::types::game::look::EntityLook; use protocol::variants::TaxCollectorComplementaryInformationsVariant; use protocol::types::game::fight::ProtectedEntityWaitingForHelpInfo;
impl_type!(AdditionalTaxCollectorInformations, 165, collector_caller_name| String, date| i32);
impl_type!(TaxCollectorBasicInformations, 96, first_name_id| VarShort, last_name_id| VarShort, world_x| i16, world_y| i16, map_id| i32, sub_area_id| VarShort);
impl_type!(TaxCollectorComplementaryInformations, 448);
impl_type!(TaxCollectorFightersInformation, 169, collector_id| i32, ally_characters_informations| Vec<CharacterMinimalPlusLookInformationsVariant>, enemy_characters_informations| Vec<CharacterMinimalPlusLookInformationsVariant>);
impl_type!(TaxCollectorGuildInformations, 446, base| TaxCollectorComplementaryInformations, guild| BasicGuildInformations);
impl_type!(TaxCollectorInformations, 167, unique_id| i32, firt_name_id| VarShort, last_name_id| VarShort, additional_infos| AdditionalTaxCollectorInformations, world_x| i16, world_y| i16, sub_area_id| VarShort, state| i8, look| EntityLook, complements| Vec<TaxCollectorComplementaryInformationsVariant>);
impl_type!(TaxCollectorLootInformations, 372, base| TaxCollectorComplementaryInformations, kamas| VarInt, experience| VarLong, pods| VarInt, items_value| VarInt);
impl_type!(TaxCollectorWaitingForHelpInformations, 447, base| TaxCollectorComplementaryInformations, waiting_for_help_info| ProtectedEntityWaitingForHelpInfo);
