use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(CheckFileMessage, 6156, filename_hash| String, type_| i8, value| String);
impl_type!(CheckFileRequestMessage, 6154, filename| String, type_| i8);
impl_type!(CheckIntegrityMessage, 6372, data| VarIntVec<u8>);
impl_type!(ClientKeyMessage, 5607, key| String);
impl_type!(RawDataMessage, 6253, content| VarIntVec<u8>);
