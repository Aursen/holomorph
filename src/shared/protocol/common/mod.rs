use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(NetworkDataContainerMessage, 2, content| Bytes);