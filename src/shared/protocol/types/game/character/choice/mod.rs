use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use protocol::types::game::character::CharacterMinimalPlusLookInformations; use protocol::types::game::character::AbstractCharacterInformation;
impl_type!(AbstractCharacterToRefurbishInformation, 475, base| AbstractCharacterInformation, colors| Vec<i32>, cosmetic_id| VarInt);
impl_type!(CharacterBaseInformations, 45, base| CharacterMinimalPlusLookInformations, breed| i8, sex| bool);
impl_type!(CharacterHardcoreOrEpicInformations, 474, base| CharacterBaseInformations, death_state| i8, death_count| VarShort, death_max_level| i8);
impl_type!(CharacterRemodelingInformation, 479, base| AbstractCharacterInformation, name| String, breed| i8, sex| bool, cosmetic_id| VarShort, colors| Vec<i32>);
impl_type!(CharacterToRecolorInformation, 212, base| AbstractCharacterToRefurbishInformation);
impl_type!(CharacterToRelookInformation, 399, base| AbstractCharacterToRefurbishInformation);
impl_type!(CharacterToRemodelInformations, 477, base| CharacterRemodelingInformation, possible_change_mask| i8, mandatory_change_mask| i8);
impl_type!(RemodelingInformation, 480, name| String, breed| i8, sex| bool, cosmetic_id| VarShort, colors| Vec<i32>);
