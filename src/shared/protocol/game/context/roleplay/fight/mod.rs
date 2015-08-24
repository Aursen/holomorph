use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(GameRolePlayAggressionMessage, 6073, attacker_id| VarInt, defender_id| VarInt);
impl_type!(GameRolePlayAttackMonsterRequestMessage, 6191, monster_group_id| i32);
impl_type!(GameRolePlayFightRequestCanceledMessage, 5822, fight_id| i32, source_id| VarInt, target_id| i32);
impl_type!(GameRolePlayPlayerFightFriendlyAnsweredMessage, 5733, fight_id| i32, source_id| VarInt, target_id| VarInt, accept| bool);
impl_type!(GameRolePlayPlayerFightFriendlyAnswerMessage, 5732, fight_id| i32, accept| bool);
impl_type!(GameRolePlayPlayerFightFriendlyRequestedMessage, 5937, fight_id| i32, source_id| VarInt, target_id| VarInt);
impl_type!(GameRolePlayPlayerFightRequestMessage, 5731, target_id| VarInt, target_cell_id| i16, friendly| bool);
impl_type!(GameRolePlayRemoveChallengeMessage, 300, fight_id| i32);
impl_type!(GameRolePlayShowChallengeMessage, 301, commons_infos| FightCommonInformations);
