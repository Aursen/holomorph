pub mod data;
pub mod social;

use session::{auth, game};
use session::game::{SocialState, SocialInformations};
use session::game::chunk::SocialUpdateType;
use std::collections::{HashSet, HashMap};
use shared::net::{Token, SessionEvent};
use shared::{self, chunk, HashBiMap};
use protocol::*;
use protocol::messages::game::approach::AlreadyConnectedMessage;
use protocol::variants::{FriendInformationsVariant, IgnoredInformationsVariant};
use eventual::Async;
use character::CharacterMinimal;
use self::data::GameServerData;
use std::sync::Mutex;

pub type Sender = chunk::Sender<Server>;

lazy_static! { pub static ref SYNC_SERVER: Mutex<Option<GameServerData>> = Mutex::new(None); }
thread_local!(pub static SERVER: GameServerData = SYNC_SERVER.lock().unwrap().clone().unwrap());

pub struct Server {
    base: shared::server::ServerBase<game::Session, game::chunk::ChunkImpl,
                                     auth::Session, auth::chunk::ChunkImpl>,

    // an in-game session can be identified by its character id
    session_characters: HashBiMap<i64, Token>,
    // and also by its account id
    session_accounts: HashBiMap<i32, Token>,
    session_socials: HashMap<i32, SocialInformations>,

    characters: HashMap<i64, CharacterMinimal>,
    character_nicknames: HashMap<String, i64>,
    character_names: HashMap<String, i64>,
    character_accounts: HashMap<i32, i64>,
    chunk_areas: HashMap<i16, usize>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            base: shared::server::ServerBase::new(),

            session_characters: HashBiMap::new(),
            session_accounts: HashBiMap::new(),
            session_socials: HashMap::new(),

            characters: HashMap::new(),
            character_nicknames: HashMap::new(),
            character_names: HashMap::new(),
            character_accounts: HashMap::new(),

            chunk_areas: HashMap::new(),
        }
    }

    pub fn auth_event(&mut self, evt: SessionEvent) {
        self.base.secondary_event(evt);
    }

    pub fn game_event(&mut self, evt: SessionEvent) {
        if let SessionEvent::Disconnect(tok) = evt {
            let id = self.session_characters.inv_remove(&tok);
            let account_id = self.session_accounts.inv_remove(&tok);
            if let Some(id) = id {
                let _ = self.session_socials.remove(&account_id.unwrap());
                self.update_social(self.characters.get(&id).unwrap(), SocialUpdateType::Default);
            }
        }

        self.base.main_event(evt);
    }
}

pub fn start_queue_timer(sender: &Sender) {
    let tx = sender.clone();
    chunk::send(sender, move |server| {
        server.base.timer.interval_ms(2000).each(move |()| {
            chunk::send(&tx, move |server| {
                for chunk in &server.base.main_chunks {
                    chunk::send(chunk, |chunk| {
                        game::chunk::update_queue(chunk);
                    });
                }
            })
        }).fire();
    });
}

pub fn add_chunk(sender: &Sender, chunk: game::chunk::Sender, areas: HashSet<i16>) {
    chunk::send(sender, move |server| {
        let len = server.base.main_chunks.len();
        for a in areas {
            let _ = server.chunk_areas.insert(a, len);
        }
        server.base.main_chunks.push(chunk)
    });
}

pub fn set_auth_chunk(sender: &Sender, chunk: auth::chunk::Sender) {
    chunk::send(sender, move |server| {
        server.base.secondary_chunk = Some(chunk);
    });
}

pub fn teleport<F>(sender: &Sender, tok: Token, area_id: i16, job: F)
                   where F: FnOnce(&mut game::chunk::Chunk) + Send + 'static {
    chunk::send(sender, move |server| {
        let chunk = server.chunk_areas.get(&area_id).unwrap();
        let _ = server.base.session_chunks.remove(&tok);
        let _ = server.base.session_chunks.insert(tok, *chunk);
        chunk::send(&server.base.main_chunks[*chunk], job);
    });
}

pub fn identification_success<F>(sender: &Sender, tok: Token, id: i32, job: F)
                                 where F: FnOnce(&mut game::Session,
                                                 HashMap<i64, CharacterMinimal>)
                                 + Send + 'static {
    chunk::send(sender, move |server| {
        if server.base.session_ids.contains_key(&id) {
            let buf = AlreadyConnectedMessage.unwrap();
            write_and_close!(SERVER, tok, buf);
            return;
        }

        let _ = server.base.session_ids.insert(id, tok);

        let characters = server.characters.iter().filter_map(|(ch_id, ch)| {
            if ch.account_id() == id {
                return Some((*ch_id, ch.clone()));
            }
            None
        }).collect();

        server.base.session_callback(tok, move |session, _| job(session, characters))
    });
}

macro_rules! load_social {
    ($state: expr, $social: ident, $server: ident, $account_id: ident) => {
        $social.get($state).iter().cloned().filter_map(|r_id| {
            $server.character_accounts.get(&r_id).map(|ch_id| {
                let ch = $server.characters.get(ch_id).unwrap();
                (
                    r_id,
                    ch.as_relation_infos(
                        $account_id,
                        $server.session_socials.get(&ch.account_id()),
                        $state
                    )
                )
            })
        })
    };
}

pub fn character_selection_success<F>(sender: &Sender, tok: Token, account_id: i32, ch_id: i64,
                                      social: SocialInformations, job: F)
                                      where F: FnOnce(&mut game::Session,
                                                      &mut game::chunk::ChunkImpl,
                                                      HashMap<i32, FriendInformationsVariant>,
                                                      HashMap<i32, IgnoredInformationsVariant>)
                                      + Send + 'static {
    chunk::send(sender, move |server| {
        let _ = server.session_characters.insert(ch_id, tok);
        let _ = server.session_accounts.insert(account_id, tok);

        let friends = load_social!(
            SocialState::Friend,
            social,
            server,
            account_id
        ).map(|(id, f)| (id, f.as_friend())).collect();

        let ignored = load_social!(
            SocialState::Ignored,
            social,
            server,
            account_id
        ).map(|(id, i)| (id, i.as_ignored())).collect();

        let _ = server.session_socials.insert(account_id, social);
        server.update_social(server.characters.get(&ch_id).unwrap(), SocialUpdateType::Online);

        server.base.session_callback(tok, move |session, mut chunk| {
            job(session, &mut *chunk, friends, ignored)
        });
    });
}

pub fn session_callback<F>(sender: &Sender, tok: Token, job: F)
                           where F: FnOnce(&mut game::Session, &mut game::chunk::ChunkImpl)
                           + Send + 'static {
    chunk::send(sender, move |server| {
        server.base.session_callback(tok, move |session, mut chunk| {
            job(session, &mut *chunk)
        })
    })
}

pub fn disconnect_player(sender: &Sender, id: i32) {
    chunk::send(sender, move |server| {
        if let Some(tok) = server.base.session_ids.get(&id) {
            close!(SERVER, *tok);
        }
    });
}

pub fn insert_character_minimal(sender: &Sender, ch: CharacterMinimal) {
    chunk::send(sender, move |server| {
        let id = ch.id();
        let name = ch.name().to_string();
        let account_nickname = ch.account_nickname().to_string();
        let account_id = ch.account_id();
        let _ = server.characters.insert(id, ch);
        let _ = server.character_names.insert(name, id);
        let _ = server.character_nicknames.insert(account_nickname, id);
        let _ = server.character_accounts.insert(account_id, id);
    })
}
