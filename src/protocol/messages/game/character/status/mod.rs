use std::io::{Read, Write};
use std::io::Result;
use protocol::*;
 use variants::PlayerStatusVariant;
impl_type!(PlayerStatusUpdateErrorMessage, 6385);
impl_type!(PlayerStatusUpdateMessage, 6386, account_id| i32, player_id| VarInt, status| PlayerStatusVariant);
impl_type!(PlayerStatusUpdateRequestMessage, 6387, status| PlayerStatusVariant);
