use postgres::{Connection, SslMode};
use std::sync::{Arc, Mutex, mpsc};
use std::boxed::FnBox;
use std::thread;

// same as Connection::connect but panics on fail
pub fn connect(uri: &str) -> Connection {
    match Connection::connect(uri, &SslMode::None) {
        Ok(conn) => conn,
        Err(err) => panic!("database connection failed: {}", err),
    }
}

type Thunk = Box<FnBox(&mut Connection) + Send + 'static>;
pub type Sender = mpsc::Sender<Thunk>;

// starts a thread pool
pub fn spawn_threads(threads: usize, uri: &str) -> Sender {
    assert!(threads >= 1);

    let (tx, rx) = mpsc::channel::<Thunk>();
    let rx = Arc::new(Mutex::new(rx));

    for _ in 0..threads {
        let rx = rx.clone();
        let uri = uri.to_string();
        thread::spawn(move || {
            let mut conn = connect(&uri);

            loop {
                // we acquire the lock only for receiving, not for executing a job
                let msg = {
                    let lock = rx.lock().unwrap();
                    lock.recv()
                };

                match msg {
                    Ok(job) => job.call_box((&mut conn,)),
                    Err(..) => return(),
                }
            }
        });
    }

    tx
}

// helper function to convert an FnOnce into an FnBox and send it to the pool
pub fn execute<F>(sender: &Sender, job: F)
    where F : FnOnce(&mut Connection) + Send + 'static {

    let boxed_job: Thunk = Box::new(move |conn: &mut Connection| job(conn));
    let _ = sender.send(boxed_job);
}
