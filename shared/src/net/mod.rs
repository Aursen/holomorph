mod connection;
mod handler;

use mio::{self, EventSet, PollOpt};
use mio::tcp::{TcpStream, TcpListener};
use mio::util::Slab;
use std::net::SocketAddr;
use std::io;
use net::connection::Connection;
use chunk;

pub use mio::Token;

pub enum Msg {
    Shutdown,
    Write(Token, Vec<u8>),
    WriteAndClose(Token, Vec<u8>),
    Close(Token),
}

pub enum SessionEvent {
    Connect(Token, String), // String is for IP address
    Packet(Token, u16, Vec<u8>),
    Disconnect(Token),
}

pub enum CallbackType {
    Listen,
    Connect,
}

struct Listener<C> {
    socket: Option<TcpListener>,
    callback: fn(&mut C, SessionEvent),
}

pub type Sender = mio::Sender<Msg>;
pub type EventLoop<C> = mio::EventLoop<Handler<C>>;

pub struct Handler<C> {
    handler: chunk::Sender<C>,
    listeners: Slab<Listener<C>>,
    connections: Slab<Connection>,
}

impl<C: 'static> Handler<C> {
    fn listen(&mut self, event_loop: &mut EventLoop<C>, address: SocketAddr,
              cb: fn(&mut C, SessionEvent)) -> io::Result<()> {
            let socket = try!(TcpListener::bind(&address));

            let tok = self.listeners.insert(Listener {
                socket: Some(socket),
                callback: cb,
            }).ok().unwrap();

            try!(
                event_loop.register(
                    self.listeners[tok].socket.as_ref().unwrap(),
                    tok,
                    EventSet::readable(),
                    PollOpt::edge()
                )
            );

            info!("ready to listen on {:?}", address);
            Ok(())
    }

    fn connect(&mut self, event_loop: &mut EventLoop<C>, address: SocketAddr,
               cb: fn(&mut C, SessionEvent)) -> io::Result<()> {
            let socket = try!(TcpStream::connect(&address));

            let tok = self.listeners.insert(Listener {
                socket: None,
                callback: cb,
            }).ok().unwrap();

            self.new_connection(event_loop, tok, socket, address)
    }

    pub fn add_callback(&mut self, event_loop: &mut EventLoop<C>, address: &str,
                        cb: fn(&mut C, SessionEvent), cb_type: CallbackType) {
        let address = match address.parse() {
            Ok(addr) => addr,
            Err(_) => panic!("failed to parse address"),
        };

        if let Err(err) = match cb_type {
            CallbackType::Listen => self.listen(event_loop, address, cb),
            CallbackType::Connect => self.connect(event_loop, address, cb),
        } {
            panic!("socket error: {}", err);
        }
    }

    pub fn new(handler: chunk::Sender<C>, listeners_size: usize) -> Self {
        Handler {
            handler: handler,
            listeners: Slab::new(listeners_size),
            connections: Slab::new_starting_at(Token(listeners_size), 65535),
        }
    }

    fn new_connection(&mut self, event_loop: &mut EventLoop<C>, tok: Token, socket: TcpStream,
                      address: SocketAddr) -> io::Result<()> {
        let ip = format!("{}", address.ip());
        let client_tok = try!(
            self.connections
                .insert(Connection::new(socket, tok))
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "too many connections"))
        );

        let cb = self.listeners[tok].callback;
        chunk::send(&self.handler, move |handler| {
            cb(handler, SessionEvent::Connect(client_tok, ip))
        });

        event_loop.register(
            self.connections[client_tok].socket(),
            client_tok,
            EventSet::readable(),
            PollOpt::level()
        )
    }

    fn handle_server_event(&mut self, event_loop: &mut EventLoop<C>, tok: Token,
                           events: EventSet) -> io::Result<()> {
        assert!(events.is_readable());

        if let Some((sock, addr)) = try!(self.listeners[tok].socket.as_ref().unwrap().accept()) {
            try!(
                self.new_connection(
                    event_loop,
                    tok,
                    sock,
                    addr
                )
            );
        }
        Ok(())
    }

    fn handle_client_event(&mut self, event_loop: &mut EventLoop<C>, tok: Token,
                           events: EventSet) -> io::Result<()> {
        if events.is_readable() {
            if let Some(packet) = try!(self.connections[tok].read()) {
                let cb = self.listeners[self.connections[tok].listener()].callback;
                chunk::send(&self.handler, move |handler| {
                    cb(handler, SessionEvent::Packet(tok, packet.id, packet.data));
                });
            }
        }

        if events.is_writable() {
            if try!(self.connections[tok].write()) {
                try!(
                    event_loop.reregister(
                        self.connections[tok].socket(),
                        tok,
                        EventSet::readable(),
                        PollOpt::level()
                    )
                );
            }
        }

        Ok(())
    }
}