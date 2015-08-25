pub mod effects;
use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use protocol::variants::ObjectEffectVariant;
impl_type!(BidExchangerObjectInfo, 122, object_uid| VarInt, effects| Vec<ObjectEffectVariant>, prices| Vec<i32>);
impl_type!(GoldItem, 123, base| Item, sum| VarInt);
impl_type!(Item, 7);
impl_type!(ObjectItem, 37, base| Item, position| i8, object_gid| VarShort, effects| Vec<ObjectEffectVariant>, object_uid| VarInt, quantity| VarInt);
impl_type!(ObjectItemGenericQuantity, 483, base| Item, object_gid| VarShort, quantity| VarInt);
impl_type!(ObjectItemInformationWithQuantity, 387, base| ObjectItemMinimalInformation, quantity| VarInt);
impl_type!(ObjectItemMinimalInformation, 124, base| Item, object_gid| VarShort, effects| Vec<ObjectEffectVariant>);
impl_type!(ObjectItemNotInContainer, 134, base| Item, object_gid| VarShort, effects| Vec<ObjectEffectVariant>, object_uid| VarInt, quantity| VarInt);
impl_type!(ObjectItemQuantity, 119, base| Item, object_uid| VarInt, quantity| VarInt);
impl_type!(ObjectItemToSell, 120, base| Item, object_gid| VarShort, effects| Vec<ObjectEffectVariant>, object_uid| VarInt, quantity| VarInt, object_price| VarInt);
impl_type!(ObjectItemToSellInBid, 164, base| ObjectItemToSell, unsold_delay| i32);
impl_type!(ObjectItemToSellInHumanVendorShop, 359, base| Item, object_gid| VarShort, effects| Vec<ObjectEffectVariant>, object_uid| VarInt, quantity| VarInt, object_price| VarInt, public_price| VarInt);
impl_type!(ObjectItemToSellInNpcShop, 352, base| ObjectItemMinimalInformation, object_price| VarInt, buy_criterion| String);
impl_type!(SellerBuyerDescriptor, 121, quantities| Vec<VarInt>, types| Vec<VarInt>, tax_percentage| f32, tax_modification_percentage| f32, max_item_level| i8, max_item_per_account| VarInt, npc_contextual_id| i32, unsold_delay| VarShort);
impl_type!(SpellItem, 49, base| Item, position| i8, spell_id| i32, spell_level| i8);
