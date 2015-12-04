use session::auth::Session;
use session::auth::chunk::{ChunkImpl, Ref};
use protocol::*;
use protocol::holomorph::*;
use shared::{self, crypt};
use std::io::Result;
use server::{self, SERVER};

#[register_handlers]
impl Session {
    pub fn handle_hello<'a>(&mut self, _: Ref<'a>, msg: HelloMessage) -> Result<()> {
        let md5_key = SERVER.with(|s| crypt::md5(&s.cnf.server_key));

        let buf = IdentificationMessage {
            id: SERVER.with(|s| s.cnf.server_id),
            key: crypt::md5(&(md5_key + &msg.salt)),
            state: 3,
            ip: SERVER.with(|s| s.cnf.bind_ip.clone()),
            port: SERVER.with(|s| s.cnf.bind_port),
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);
        Ok(())
    }

    pub fn handle_disconnect_player<'a>(&mut self, _: Ref<'a>, msg: DisconnectPlayerMessage)
                                        -> Result<()> {
        SERVER.with(|s| server::disconnect_player(&s.server, msg.id));

        Ok(())
    }
}
