use mio::{TryRead, TryWrite};
use mio::tcp::*;
use std::io;
use std::io::{Read, Cursor};
use ::io::ReadExt;
use mio;
use mio::Sender;
use std::collections::VecDeque;
use super::server::Msg;
use ::session::Session;

type Buffer = (Vec<u8>, usize);

fn make_buffer(len: usize) -> Buffer {
    (vec![0; len], 0)
}

pub struct Connection<S: Session> {
    session: S,
    pub socket: TcpStream,
    pub token: mio::Token,
    read_buffer: Option<Buffer>,
    write_buffer: VecDeque<Buffer>,
    state: State,
}

#[derive(Debug)]
enum State {
    WaitingForHeader,
    WaitingForLen(u16),
    WaitingForData(u16),
}

impl<S: Session> Connection<S> {
    pub fn new(socket: TcpStream, token: mio::Token, sender: Sender<Msg>) -> Connection<S> {
        Connection {
            session: S::new(token, sender),
            socket: socket,
            token: token,
            read_buffer: Some(make_buffer(2)),
            write_buffer: VecDeque::new(),
            state: State::WaitingForHeader,
        }
    }

    pub fn readable(&mut self) -> io::Result<()> {
        let (mut buf, pos) = self.read_buffer.take().unwrap();
        let s = match try!(self.socket.try_read(&mut buf[pos..])) {
            None | Some(0) => return Err(io::Error::new(io::ErrorKind::Other, "EOF")),
            Some(s) => s,
        };

        if pos + s != buf.len() {
            self.read_buffer = Some((buf, pos + s));
            return Ok(());
        }

        let mut buf = Cursor::new(buf);
        match self.state {
            State::WaitingForHeader => {
                let header = try!(buf.read_u16());
                let id = header >> 2;
                let nbytes = header & 3;
                self.state = State::WaitingForLen(id);
                self.read_buffer = Some(make_buffer(nbytes as usize));
            }

            State::WaitingForLen(id) => {
                let mut len = 0u32;
                for _ in (0..buf.get_ref().len()) {
                    len = (len << 8) + (try!(buf.read_u8()) as u32);
                }
                self.state = State::WaitingForData(id);
                self.read_buffer = Some(make_buffer(len as usize));
            }

            State::WaitingForData(id) => {
                self.state = State::WaitingForHeader;
                self.read_buffer = Some(make_buffer(2));

                try!(self.session.handle_packet(id, buf))
            }
        }

        Ok(())
    }

    pub fn writable(&mut self) -> io::Result<()> {
        if self.write_buffer.is_empty() {
            return Ok(())
        }

        {
            let mut buf = self.write_buffer.back_mut().unwrap();
            let s = match try!(self.socket.try_write(&buf.0[buf.1..])) {
                None => return Err(io::Error::new(io::ErrorKind::Other, "cannot write")),
                Some(s) => s,
            };

            if buf.1 + s != buf.0.len() {
                buf.1 += s;
                return Ok(());
            }
        }

        let _ = self.write_buffer.pop_back();
        Ok(())
    }

    pub fn write(&mut self, buffer: Vec<u8>) {
        self.write_buffer.push_front((buffer, 0));
    }
}
