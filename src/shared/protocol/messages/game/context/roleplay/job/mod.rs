use std::io::{Read, Write};
use io::Result;
use protocol::*;
use protocol::types::game::context::roleplay::job::JobCrafterDirectoryEntryJobInfo;  use protocol::types::game::context::roleplay::job::JobCrafterDirectorySettings; use protocol::types::game::look::EntityLook; use protocol::types::game::context::roleplay::job::JobExperience; use protocol::types::game::context::roleplay::job::JobDescription; use protocol::types::game::context::roleplay::job::JobCrafterDirectoryListEntry; use protocol::types::game::context::roleplay::job::JobCrafterDirectoryEntryPlayerInfo;
impl_type!(JobAllowMultiCraftRequestMessage, 5748, enabled| bool);
impl_type!(JobBookSubscriptionMessage, 6593, added_or_deleted| bool, job_id| i8);
impl_type!(JobCrafterDirectoryAddMessage, 5651, list_entry| JobCrafterDirectoryListEntry);
impl_type!(JobCrafterDirectoryDefineSettingsMessage, 5649, settings| JobCrafterDirectorySettings);
impl_type!(JobCrafterDirectoryEntryMessage, 6044, player_info| JobCrafterDirectoryEntryPlayerInfo, job_info_list| Vec<JobCrafterDirectoryEntryJobInfo>, player_look| EntityLook);
impl_type!(JobCrafterDirectoryEntryRequestMessage, 6043, player_id| VarInt);
impl_type!(JobCrafterDirectoryListMessage, 6046, list_entries| Vec<JobCrafterDirectoryListEntry>);
impl_type!(JobCrafterDirectoryListRequestMessage, 6047, job_id| i8);
impl_type!(JobCrafterDirectoryRemoveMessage, 5653, job_id| i8, player_id| VarInt);
impl_type!(JobCrafterDirectorySettingsMessage, 5652, crafters_settings| Vec<JobCrafterDirectorySettings>);
impl_type!(JobDescriptionMessage, 5655, jobs_description| Vec<JobDescription>);
impl_type!(JobExperienceMultiUpdateMessage, 5809, experiences_update| Vec<JobExperience>);
impl_type!(JobExperienceOtherPlayerUpdateMessage, 6599, base| JobExperienceUpdateMessage, player_id| VarInt);
impl_type!(JobExperienceUpdateMessage, 5654, experiences_update| JobExperience);
impl_type!(JobLevelUpMessage, 5656, new_level| i8, jobs_description| JobDescription);
impl_type!(JobMultiCraftAvailableSkillsMessage, 5747, base| JobAllowMultiCraftRequestMessage, player_id| VarInt, skills| Vec<VarShort>);
