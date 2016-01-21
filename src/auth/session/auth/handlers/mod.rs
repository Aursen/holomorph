pub mod selection;
pub mod identification;

use protocol::{Protocol, VarShort, VarIntVec};
use protocol::messages::connection::*;
use protocol::messages::handshake::*;
use protocol::messages::queues::*;
use protocol::types::connection::GameServerInformations;
use super::{Session, QueueState};
use std::sync::atomic::Ordering;
use server::data::GameServerData;
use super::chunk::{ChunkImpl, Ref};
use std::io::{Result, Cursor};
use server::SERVER;
use protocol::enums::server_status;
use shared::{self, database};
use std::collections::HashMap;
use rand::{self, Rng};

impl shared::session::Session<ChunkImpl> for Session {
    fn new(base: shared::session::SessionBase) -> Self {
        let mut buf = ProtocolRequired {
            required_version: 1658,
            current_version: 1658,
        }.unwrap();

        let salt = rand::thread_rng().gen_ascii_chars().take(32).collect::<String>();

        HelloConnectMessage {
            salt: salt.clone(),
            key: VarIntVec(SERVER.with(|s| (*s.signed_pub_key).clone())),
        }.unwrap_with_buf(&mut buf);

        write!(SERVER, base.token, buf);

        Session {
            base: base,
            account: None,
            queue_state: QueueState::None,
            salt: salt,
            aes_key: Vec::new(),
            character_counts: HashMap::new(),
        }
    }

    fn handle<'a>(&mut self, chunk: Ref<'a>, id: i16, mut data: Cursor<Vec<u8>>) -> Result<()> {
        handle!(self, chunk, id, data)
    }

    fn close<'a>(self, _: Ref<'a>) {
        if let Some(id) = self.account.as_ref().map(|a| a.id) {
            SERVER.with(|s| database::execute(&s.db, move |conn| {
                if let Err(err) = self.base.save_logs(conn, id) {
                    error!("error while saving logs: {:?}", err);
                }
            }));
        }
    }
}

impl Session {
    pub fn update_queue(&self) {
        if let QueueState::Some(queue_size, queue_counter) = self.queue_state {
            use self::identification::{QUEUE_COUNTER, QUEUE_SIZE};

            let mut pos = queue_size - (QUEUE_COUNTER.load(Ordering::Relaxed) - queue_counter);

            if pos < 0 {
                pos = 0;
            }

            let buf = LoginQueueStatusMessage {
                position: pos as i16,
                total: QUEUE_SIZE.load(Ordering::Relaxed) as i16,
            }.unwrap();

            write!(SERVER, self.base.token, buf);
        }
    }

    fn get_server_informations(&self, server: &GameServerData, mut status: i8)
                               -> GameServerInformations {
        let data = self.account.as_ref().unwrap();

        if data.is_subscriber() && status == server_status::FULL {
            status = server_status::ONLINE;
        }

        GameServerInformations {
            id: VarShort(server.id()),
            type_: 0,
            status: status,
            completion: 0,
            is_selectable: status == server_status::ONLINE,
            characters_count: *self.character_counts.get(&server.id()).unwrap_or(&0),
            characters_slots: 0,
            date: 0.,
        }
    }

    pub fn update_server_status(&self, server_id: i16, status: i8) {
        let account = match self.account.as_ref() {
            Some(account) => account,
            None => return (),
        };

        SERVER.with(|s| {
            let server = s.game_servers.get(&server_id).unwrap();

            if server.min_level() > account.level {
                return ();
            }

            let buf = ServerStatusUpdateMessage {
                server: self.get_server_informations(server, status),
            }.unwrap();

            write!(SERVER, self.base.token, buf);
        });
    }
}
