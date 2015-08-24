use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(AccessoryPreviewErrorMessage, 6521, error| i8);
impl_type!(AccessoryPreviewMessage, 6517, look| EntityLook);
impl_type!(AccessoryPreviewRequestMessage, 6518, generic_id| Vec<VarShort>);

impl_type!(EntityLook, 55, bones_id| VarShort, skins| Vec<VarShort>, indexed_colors| Vec<i32>, scales| Vec<VarShort>, subentities| Vec<SubEntity>);
impl_type!(IndexedEntityLook, 405, look| EntityLook, index| i8);
impl_type!(SubEntity, 54, binding_point_category| i8, binding_point_index| i8, sub_entity_look| EntityLook);
