use std::io::{Read, Write};
use std::io::Result;
use protocol::*;
 use messages::debug::DebugInClientMessage;
impl_type!(ClientYouAreDrunkMessage, 6594, base| DebugInClientMessage);
