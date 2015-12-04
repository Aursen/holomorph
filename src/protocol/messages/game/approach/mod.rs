use std::io::{Read, Write};
use std::io::Result;
use protocol::*;
 use variants::ServerSessionConstantVariant;
impl_type!(AccountCapabilitiesMessage, 6216, tutorial_available| Flag, can_create_new_character| Flag, account_id| i32, breeds_visible| i16, breeds_available| i16, status| i8);
impl_type!(AccountLoggingKickedMessage, 6029, days| VarShort, hours| i8, minutes| i8);
impl_type!(AlreadyConnectedMessage, 109);
impl_type!(AuthenticationTicketAcceptedMessage, 111);
impl_type!(AuthenticationTicketMessage, 110, lang| String, ticket| String);
impl_type!(AuthenticationTicketRefusedMessage, 112);
impl_type!(HelloGameMessage, 101);
impl_type!(ReloginTokenRequestMessage, 6540);
impl_type!(ReloginTokenStatusMessage, 6539, valid_token| bool, ticket| VarIntVec<u8>);
impl_type!(ServerOptionalFeaturesMessage, 6305, features| Vec<u8>);
impl_type!(ServerSessionConstantsMessage, 6434, variables| Vec<ServerSessionConstantVariant>);
impl_type!(ServerSettingsMessage, 6340, lang| String, community| i8, game_type| i8);
