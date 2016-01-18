pub mod zaap;
pub mod meeting;
use std::io::{Read, Write};
use std::io::Result;
use protocol::*;
 use variants::InteractiveElementVariant; use types::game::interactive::InteractiveElement; use types::game::interactive::StatedElement;
impl_type!(InteractiveElementUpdatedMessage, 5708, interactive_element| InteractiveElement);
impl_type!(InteractiveMapUpdateMessage, 5002, interactive_elements| Vec<InteractiveElementVariant>);
impl_type!(InteractiveUsedMessage, 5745, entity_id| VarLong, elem_id| VarInt, skill_id| VarShort, duration| VarShort);
impl_type!(InteractiveUseEndedMessage, 6112, elem_id| VarInt, skill_id| VarShort);
impl_type!(InteractiveUseErrorMessage, 6384, elem_id| VarInt, skill_instance_uid| VarInt);
impl_type!(InteractiveUseRequestMessage, 5001, elem_id| VarInt, skill_instance_uid| VarInt);
impl_type!(StatedElementUpdatedMessage, 5709, stated_element| StatedElement);
impl_type!(StatedMapUpdateMessage, 5716, stated_elements| Vec<StatedElement>);
