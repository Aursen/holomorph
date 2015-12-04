macro_rules! get_mut_character {
    ($ch: ident, $chunk: ident) => {
        $chunk.maps
              .get_mut(&$ch.map_id).unwrap()
              .get_mut_actor($ch.id).unwrap()
              .as_mut_character()
    };
}

macro_rules! get_character {
    ($ch: ident, $chunk: ident) => {
        $chunk.maps
              .get(&$ch.map_id).unwrap()
              .get_actor($ch.id).unwrap()
              .as_character()
    };
}

pub mod chunk;
mod handlers;

use std::collections::{HashMap, HashSet};
use shared;
use time;
use character::{Character, CharacterMinimal};
use protocol::variants::{FriendInformationsVariant, IgnoredInformationsVariant,
    PlayerStatusVariant};

pub struct CharacterRef {
    id: i32,
    map_id: i32,
    movements: Option<Vec<i16>>,
}

pub enum SocialState {
    Friend,
    Ignored,
}

#[derive(Clone)]
pub struct SocialInformations {
    pub friends: HashSet<i32>,
    pub ignored: HashSet<i32>,
    warn_on_connection: bool,
    warn_on_level_gain: bool,
    pub status: PlayerStatusVariant,
}

impl SocialInformations {
    pub fn is_friend_with(&self, account_id: i32) -> bool {
        self.friends.contains(&account_id)
    }

    pub fn ignores(&self, account_id: i32) -> bool {
        self.ignored.contains(&account_id)
    }
}

/*#[derive(Clone)]
pub struct SocialInformations {
    pub friends: HashMap<i32, SocialState>,
    warn_on_connection: bool,
    warn_on_level_gain: bool,
    pub status: PlayerStatusVariant,
}

impl SocialInformations {
    fn get_social_state(&self, account_id: i32) -> SocialState {

    }

    pub fn is_friend_with(&self, account_id: i32) -> bool {
        self.friends.get(&account_id).map(|state| )
    }

    pub fn ignores(&self, account_id: i32) -> bool {
        self.ignored.contains(&account_id)
    }
}*/

struct AccountData {
    id: i32,
    nickname: String,
    secret_answer: String,
    level: i8,
    subscription_end: i64,
    last_connection: i64,
    last_ip: String,
    social: SocialInformations,
}

impl AccountData {
    fn is_subscriber(&self) -> bool {
        self.subscription_end > time::get_time().sec
    }
}

enum GameState {
    None,
    TicketQueue(isize, isize),
    CharacterSelection(HashMap<i32, CharacterMinimal>), // at this point, self.account is some
    GameQueue(isize, isize),
    SwitchingContext(i32, Character),
    InContext(CharacterRef),
}

impl GameState {
    fn is_none(&self) -> bool {
        match *self {
            GameState::None => true,
            _ => false,
        }
    }
}

pub struct Session {
    base: shared::session::SessionBase,
    account: Option<AccountData>,
    state: GameState,

    last_sales_chat_request: i64,
    last_seek_chat_request: i64,

    friends: HashMap<i32, FriendInformationsVariant>,
    ignored: HashMap<i32, IgnoredInformationsVariant>,
}
