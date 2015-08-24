use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(PortalUseRequestMessage, 6492, portal_id| VarInt);
impl_type!(TreasureHuntAvailableRetryCountUpdateMessage, 6491, quest_type| i8, available_retry_count| i32);
impl_type!(TreasureHuntDigRequestAnswerFailedMessage, 6509, base| TreasureHuntDigRequestAnswerMessage, wrong_flag_count| i8);
impl_type!(TreasureHuntDigRequestAnswerMessage, 6484, quest_type| i8, result| i8);
impl_type!(TreasureHuntDigRequestMessage, 6485, quest_type| i8);
impl_type!(TreasureHuntFinishedMessage, 6483, quest_type| i8);
impl_type!(TreasureHuntFlagRemoveRequestMessage, 6510, quest_type| i8, index| i8);
impl_type!(TreasureHuntFlagRequestAnswerMessage, 6507, quest_type| i8, result| i8, index| i8);
impl_type!(TreasureHuntFlagRequestMessage, 6508, quest_type| i8, index| i8);
impl_type!(TreasureHuntGiveUpRequestMessage, 6487, quest_type| i8);
impl_type!(TreasureHuntLegendaryRequestMessage, 6499, legendary_id| VarShort);
impl_type!(TreasureHuntMessage, 6486, quest_type| i8, start_map_id| i32, known_steps_list| Vec<TreasureHuntStepVariant>, total_step_count| i8, check_point_current| VarInt, check_point_total| VarInt, available_retry_count| i32, flags| Vec<TreasureHuntFlag>);
impl_type!(TreasureHuntRequestAnswerMessage, 6489, quest_type| i8, result| i8);
impl_type!(TreasureHuntRequestMessage, 6488, quest_level| i8, quest_type| i8);
impl_type!(TreasureHuntShowLegendaryUIMessage, 6498, available_legendary_ids| Vec<VarShort>);

impl_type!(PortalInformation, 466, portal_id| VarShort, area_id| i16);
impl_type!(TreasureHuntFlag, 473, map_id| i32, state| i8);
impl_type!(TreasureHuntStep, 463);
impl_type!(TreasureHuntStepDig, 465);
impl_type!(TreasureHuntStepFight, 462);
impl_type!(TreasureHuntStepFollowDirection, 468, base| TreasureHuntStep, direction| i8, map_count| VarShort);
impl_type!(TreasureHuntStepFollowDirectionToHint, 472, base| TreasureHuntStep, direction| i8, npc_id| VarShort);
impl_type!(TreasureHuntStepFollowDirectionToPOI, 461, base| TreasureHuntStep, direction| i8, poi_label_id| VarShort);
