use std::io::{Read, Write};
use std::io::Result;
use protocol::*;
use types::game::context::roleplay::GuildInformations;  use types::game::context::roleplay::AllianceInformations; use types::game::context::roleplay::BasicNamedAllianceInformations;
impl_type!(AbstractSocialGroupInfos, 416);
impl_type!(AlliancedGuildFactSheetInformations, 422, base| GuildInformations, alliance_infos| BasicNamedAllianceInformations);
impl_type!(AllianceFactSheetInformations, 421, base| AllianceInformations, creation_date| i32);
impl_type!(AllianceVersatileInformations, 432, alliance_id| VarInt, nb_guilds| VarShort, nb_members| VarShort, nb_subarea| VarShort);
impl_type!(GuildFactSheetInformations, 424, base| GuildInformations, leader_id| VarLong, nb_members| VarShort);
impl_type!(GuildInAllianceVersatileInformations, 437, base| GuildVersatileInformations, alliance_id| VarInt);
impl_type!(GuildInsiderFactSheetInformations, 423, base| GuildFactSheetInformations, leader_name| String, nb_connected_members| VarShort, nb_tax_collectors| i8, last_activity| i32, enabled| bool);
impl_type!(GuildVersatileInformations, 435, guild_id| VarInt, leader_id| VarLong, guild_level| i8, nb_members| i8);
