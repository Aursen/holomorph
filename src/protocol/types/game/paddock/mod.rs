use std::io::{Read, Write};
use std::io::Result;
use protocol::*;
use types::game::context::roleplay::GuildInformations;  use types::game::mount::ItemDurability; use types::game::context::roleplay::ObjectItemInRolePlay;
impl_type!(MountInformationsForPaddock, 184, model_id| i8, name| String, owner_name| String);
impl_type!(PaddockAbandonnedInformations, 133, base| PaddockBuyableInformations, guild_id| i32);
impl_type!(PaddockBuyableInformations, 130, base| PaddockInformations, price| VarInt, locked| bool);
impl_type!(PaddockContentInformations, 183, base| PaddockInformations, paddock_id| i32, world_x| i16, world_y| i16, map_id| i32, sub_area_id| VarShort, abandonned| bool, mounts_informations| Vec<MountInformationsForPaddock>);
impl_type!(PaddockInformations, 132, max_outdoor_mount| VarShort, max_items| VarShort);
impl_type!(PaddockInformationsForSell, 222, guild_owner| String, world_x| i16, world_y| i16, sub_area_id| VarShort, nb_mount| i8, nb_object| i8, price| VarInt);
impl_type!(PaddockItem, 185, base| ObjectItemInRolePlay, durability| ItemDurability);
impl_type!(PaddockPrivateInformations, 131, base| PaddockAbandonnedInformations, guild_info| GuildInformations);
