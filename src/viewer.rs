use std::sync::mpsc::{Sender, Receiver, channel};
use log::{info, error, debug};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use websocket::{
    OwnedMessage,
    sync::Server, server::NoTlsAcceptor
};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Frame(Vec<(f32, f32)>),
    Clear,
    Spawn(Vec<(Uuid, String)>),
    Kill(Vec<usize>),
}

struct Viewer {
    server: Server<NoTlsAcceptor>,
}

impl Viewer {
    fn new(addr: &str) -> Viewer {
        let server = Server::bind(addr).unwrap();
        server.set_nonblocking(true).unwrap();

        Viewer {
            server
        }
    }

    fn accept_incoming(&mut self) {
        while let Ok(upgrade) = self.server.accept() {
            match upgrade.accept() {
                Err(e) => error!("websocket error: {:?}", e),
                Ok(client) => {
                    debug!("new client: {:?}", client.peer_addr());
                    client.shutdown();
                }
            }
        }
    }

    fn publish_messages(&self, receiver: &Receiver<Message>) {
        use std::time::Duration;
        const TIMEOUT: Duration = Duration::from_millis(1);

        while let Ok(message) = receiver.recv_timeout(TIMEOUT) {
            debug!("Publishing: {:?}", message);
        }
    }

    fn run(&mut self, receiver: Receiver<Message>) {
        info!("viewer api started");

        loop {
            self.accept_incoming();
            self.publish_messages(&receiver);
        }
    }
}

pub fn start_viewer() -> Sender<Message> {
    let (sender, receiver) = channel();

    std::thread::spawn(|| {
        let mut viewer = Viewer::new("0.0.0.0:29999");
        viewer.run(receiver);
    });

    sender
}
