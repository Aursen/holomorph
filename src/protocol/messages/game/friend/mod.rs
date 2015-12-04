use std::io::{Read, Write};
use std::io::Result;
use protocol::*;
 use variants::FriendInformationsVariant; use variants::IgnoredInformationsVariant; use variants::FriendSpouseInformationsVariant;
impl_type!(FriendAddedMessage, 5599, friend_added| FriendInformationsVariant);
impl_type!(FriendAddFailureMessage, 5600, reason| i8);
impl_type!(FriendAddRequestMessage, 4004, name| String);
impl_type!(FriendDeleteRequestMessage, 5603, account_id| i32);
impl_type!(FriendDeleteResultMessage, 5601, success| bool, name| String);
impl_type!(FriendJoinRequestMessage, 5605, name| String);
impl_type!(FriendSetWarnOnConnectionMessage, 5602, enable| bool);
impl_type!(FriendSetWarnOnLevelGainMessage, 6077, enable| bool);
impl_type!(FriendsGetListMessage, 4001);
impl_type!(FriendsListMessage, 4002, friends_list| Vec<FriendInformationsVariant>);
impl_type!(FriendSpouseFollowWithCompassRequestMessage, 5606, enable| bool);
impl_type!(FriendSpouseJoinRequestMessage, 5604);
impl_type!(FriendUpdateMessage, 5924, friend_updated| FriendInformationsVariant);
impl_type!(FriendWarnOnConnectionStateMessage, 5630, enable| bool);
impl_type!(FriendWarnOnLevelGainStateMessage, 6078, enable| bool);
impl_type!(GuildMemberSetWarnOnConnectionMessage, 6159, enable| bool);
impl_type!(GuildMemberWarnOnConnectionStateMessage, 6160, enable| bool);
impl_type!(IgnoredAddedMessage, 5678, ignore_added| IgnoredInformationsVariant, session| bool);
impl_type!(IgnoredAddFailureMessage, 5679, reason| i8);
impl_type!(IgnoredAddRequestMessage, 5673, name| String, session| bool);
impl_type!(IgnoredDeleteRequestMessage, 5680, account_id| i32, session| bool);
impl_type!(IgnoredDeleteResultMessage, 5677, success| Flag, session| Flag, name| String);
impl_type!(IgnoredGetListMessage, 5676);
impl_type!(IgnoredListMessage, 5674, ignored_list| Vec<IgnoredInformationsVariant>);
impl_type!(SpouseGetInformationsMessage, 6355);
impl_type!(SpouseInformationsMessage, 6356, spouse| FriendSpouseInformationsVariant);
impl_type!(SpouseStatusMessage, 6265, has_spouse| bool);
impl_type!(WarnOnPermaDeathStateMessage, 6513, enable| bool);
