use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::collections::HashMap;
use log::{info, error, warn, debug};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use websocket::OwnedMessage;
use websocket::sync::Client;
use websocket::{
    sync::Server, server::NoTlsAcceptor
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub radius: f32,
    pub zone: Option<super::Zone>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    Frame(Vec<(f32, f32)>),
    Clear,
    Spawn(Vec<(Uuid, Vec<f32>)>),
    Kill(Vec<usize>),
    Settings(Settings),
}

pub fn spawn(agents: &Vec<super::Agent>) -> Event {
    Event::Spawn(agents.into_iter().map(|agent| (agent.uuid, Vec::from(agent.genome.to_vec()))).collect())
}

pub fn frame(agents: &Vec<super::Agent>) -> Event {
    Event::Frame(agents.into_iter().map(|agent| agent.position).collect())
}

type ViewerServer = Server<NoTlsAcceptor>;
type ViewerClient = Client<TcpStream>;

struct Viewer {
    server: ViewerServer,
    clients: HashMap<SocketAddr, ViewerClient>,
    agents: Vec<(Uuid, Vec<f32>)>,
    settings: Option<Settings>,
}

impl Viewer {
    fn new(addr: &str) -> Viewer {
        let server = Server::bind(addr).unwrap();
        server.set_nonblocking(true).unwrap();

        Viewer {
            server,
            clients: HashMap::new(),
            agents: vec![],
            settings: None,
        }
    }

    fn send_message(&self, to: &mut ViewerClient, event: &Event) {
        let encoded = serde_json::to_string(event).unwrap();
        to.send_message(&websocket::Message::text(encoded)).ok();
    }

    fn accept_incoming(&mut self) {
        while let Ok(upgrade) = self.server.accept() {
            match upgrade.accept() {
                Err(e) => warn!("websocket accept error: {:?}", e),
                Ok(mut client) => {
                    if let Ok(addr) = client.peer_addr() {
                        client.set_nodelay(true).unwrap();
                        client.set_nonblocking(true).unwrap();

                        if let Some(settings) = &self.settings {
                            self.send_message(&mut client, &Event::Settings(settings.clone()));
                        }
                        self.send_message(&mut client, &Event::Spawn(self.agents.clone()));

                        if let Some(old_client) = self.clients.insert(addr, client) {
                            old_client.shutdown().unwrap();
                            debug!("disconnected: {}", addr);
                        }
                        debug!("connected: {}", addr);
                    }
                    else {
                        error!("couldn't get peer address");
                    }
                }
            }
        }
    }

    fn publish_events(&mut self, receiver: &Receiver<Event>) {
        use std::time::Duration;
        const TIMEOUT: Duration = Duration::from_millis(1);

        while let Ok(event) = receiver.recv_timeout(TIMEOUT) {
            //debug!("Publishing to {} clients: {:?}", self.clients.len(), event);

            match &event {
                Event::Spawn(agents) => {
                    let mut agents = agents.clone();
                    self.agents.append(&mut agents);
                },
                Event::Clear => {
                    self.agents.clear();
                }
                Event::Settings(settings) => {
                    self.settings = Some(settings.clone())
                }
                _ => ()
            }

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
        }
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
