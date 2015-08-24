use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(GuidedModeQuitRequestMessage, 6092);
impl_type!(GuidedModeReturnRequestMessage, 6088);
impl_type!(QuestListMessage, 5626, finished_quests_ids| Vec<VarShort>, finished_quests_counts| Vec<VarShort>, active_quests| Vec<QuestActiveInformationsVariant>, reinit_done_quests_ids| Vec<VarShort>);
impl_type!(QuestListRequestMessage, 5623);
impl_type!(QuestObjectiveValidatedMessage, 6098, quest_id| VarShort, objective_id| VarShort);
impl_type!(QuestObjectiveValidationMessage, 6085, quest_id| VarShort, objective_id| VarShort);
impl_type!(QuestStartedMessage, 6091, quest_id| VarShort);
impl_type!(QuestStartRequestMessage, 5643, quest_id| VarShort);
impl_type!(QuestStepInfoMessage, 5625, infos| QuestActiveInformationsVariant);
impl_type!(QuestStepInfoRequestMessage, 5622, quest_id| VarShort);
impl_type!(QuestStepStartedMessage, 6096, quest_id| VarShort, step_id| VarShort);
impl_type!(QuestStepValidatedMessage, 6099, quest_id| VarShort, step_id| VarShort);
impl_type!(QuestValidatedMessage, 6097, quest_id| VarShort);

impl_type!(GameRolePlayNpcQuestFlag, 384, quests_to_valid_id| Vec<VarShort>, quests_to_start_id| Vec<VarShort>);
impl_type!(QuestActiveDetailedInformations, 382, base| QuestActiveInformations, step_id| VarShort, objectives| Vec<QuestObjectiveInformationsVariant>);
impl_type!(QuestActiveInformations, 381, quest_id| VarShort);
impl_type!(QuestObjectiveInformations, 385, objective_id| VarShort, objective_status| bool, dialog_params| Vec<String>);
impl_type!(QuestObjectiveInformationsWithCompletion, 386, base| QuestObjectiveInformations, cur_completion| VarShort, max_completion| VarShort);
