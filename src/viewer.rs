use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::collections::HashMap;
use log::{info, warn, debug};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use websocket::OwnedMessage;
use websocket::sync::Client;
use websocket::{
    sync::Server, server::NoTlsAcceptor
};

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    Frame(Vec<(f32, f32)>),
    Clear,
    Spawn(Vec<(Uuid, String)>),
    Kill(Vec<usize>),
}

struct Viewer {
    server: Server<NoTlsAcceptor>,
    clients: HashMap<SocketAddr, Client<TcpStream>>,
}

impl Viewer {
    fn new(addr: &str) -> Viewer {
        let server = Server::bind(addr).unwrap();
        server.set_nonblocking(true).unwrap();

        Viewer {
            server,
            clients: HashMap::new(),
        }
    }

    fn accept_incoming(&mut self) {
        while let Ok(upgrade) = self.server.accept() {
            match upgrade.accept() {
                Err(e) => warn!("websocket accept error: {:?}", e),
                Ok(mut client) => {
                    client.set_nodelay(true).unwrap();
                    client.set_nonblocking(true).unwrap();
                    let addr = client.peer_addr().unwrap();
                    if let Some(old_client) = self.clients.insert(addr, client) {
                        old_client.shutdown().unwrap();
                        debug!("disconnected: {}", addr);
                    }
                    debug!("connected: {}", addr);
                }
            }
        }
    }

    fn publish_events(&mut self, receiver: &Receiver<Event>) {
        use std::time::Duration;
        const TIMEOUT: Duration = Duration::from_millis(1);

        while let Ok(event) = receiver.recv_timeout(TIMEOUT) {
            debug!("Publishing to {} clients: {:?}", self.clients.len(), event);

            let encoded = serde_json::to_string(&event).unwrap();
            let message = websocket::Message::text(encoded);

            let mut disconnected_clients = vec![];
            for (addr, client) in self.clients.iter_mut() {
                match client.send_message(&message) {
                    Err(_) => {
                        // If sending fails, assume the connection is lost.
                        // Shut it down for good measure.
                        client.shutdown().ok();
                        debug!("disconnected: {}", addr);
                        disconnected_clients.push(addr.clone());
                    }
                    Ok(_) => {}
                }
            }

            // Clean up disconnected clients.
            for addr in disconnected_clients {
                self.clients.remove(&addr);
            }
        }
    }

    fn receive_requests(&mut self) {
        //let mut disconnected_clients = vec![];
        for (addr, client) in self.clients.iter_mut() {
            if let Ok(message) = client.recv_message() {
                debug!("message from {}: {:?}", addr, message);
                match message {
                    OwnedMessage::Close(_) => { client.shutdown().ok(); }
                    _ => (),
                };
            }
            //match client.recv_message() {
            //    Err(e) => {
            //        // If sending fails, assume the connection is lost.
            //        // Shut it down for good measure.
            //        client.shutdown().ok();
            //        debug!("disconnected: {}: {}", addr, e);
            //        disconnected_clients.push(addr.clone());
            //    }
            //    Ok(message) => {
            //        debug!("message from {}: {:?}", addr, message);
            //    }
            //}
        }

        // Clean up disconnected clients.
        //for addr in disconnected_clients {
        //    self.clients.remove(&addr);
        //}
    }

    fn run(&mut self, receiver: Receiver<Event>) {
        info!("viewer api started");

        loop {
            self.accept_incoming();
            self.receive_requests();
            self.publish_events(&receiver);
        }
    }
}

pub fn start_viewer() -> Sender<Event> {
    let (sender, receiver) = channel();

    std::thread::spawn(|| {
        let mut viewer = Viewer::new("0.0.0.0:29999");
        viewer.run(receiver);
    });

    sender
}
