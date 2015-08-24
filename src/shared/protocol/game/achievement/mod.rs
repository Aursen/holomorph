use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(AchievementDetailedListMessage, 6358, started_achievements| Vec<Achievement>, finished_achievements| Vec<Achievement>);
impl_type!(AchievementDetailedListRequestMessage, 6357, category_id| VarShort);
impl_type!(AchievementDetailsMessage, 6378, achievement| Achievement);
impl_type!(AchievementDetailsRequestMessage, 6380, achievement_id| VarShort);
impl_type!(AchievementFinishedInformationMessage, 6381, base| AchievementFinishedMessage, name| String, player_id| VarInt);
impl_type!(AchievementFinishedMessage, 6208, id| VarShort, finishedlevel| i8);
impl_type!(AchievementListMessage, 6205, finished_achievements_ids| Vec<VarShort>, rewardable_achievements| Vec<AchievementRewardable>);
impl_type!(AchievementRewardErrorMessage, 6375, achievement_id| i16);
impl_type!(AchievementRewardRequestMessage, 6377, achievement_id| i16);
impl_type!(AchievementRewardSuccessMessage, 6376, achievement_id| i16);
impl_type!(FriendGuildSetWarnOnAchievementCompleteMessage, 6382, enable| bool);
impl_type!(FriendGuildWarnOnAchievementCompleteStateMessage, 6383, enable| bool);

impl_type!(Achievement, 363, id| VarShort, finished_objective| Vec<AchievementObjective>, started_objectives| Vec<AchievementStartedObjective>);
impl_type!(AchievementObjective, 404, id| VarInt, max_value| VarShort);
impl_type!(AchievementRewardable, 412, id| VarShort, finishedlevel| i8);
impl_type!(AchievementStartedObjective, 402, base| AchievementObjective, value| VarShort);
