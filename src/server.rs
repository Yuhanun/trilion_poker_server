use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use crate::connectioninfo;
use crate::game;

pub struct Server {
    listener: Arc<TcpListener>,
    connections: Arc<Mutex<HashMap<u16, connectioninfo::ConnectionInfo>>>,
    games: Arc<Mutex<HashMap<usize, game::Game>>>,
}

impl Server {
    pub fn new() -> std::io::Result<Self> {
        Ok(Self {
            listener: Arc::from(TcpListener::bind("127.0.0.1:6969")?),
            connections: Arc::from(Mutex::from(HashMap::new())),
            games: Arc::from(Mutex::from(HashMap::new()))
        })
    }

    fn tick(&mut self) {
        self.connections
            .lock()
            .unwrap()
            .iter()
            .for_each(|e| println!("{:#?}", e));

        println!("Iterated");
    }

    pub fn run(mut self) {
        let thread_listener = self.listener.clone();
        let thread_connections = self.connections.clone();
        std::thread::spawn(move || {
            for stream in thread_listener.incoming() {
                let stream = stream.unwrap();
                thread_connections.lock().unwrap().insert(
                    stream.local_addr().unwrap().port(),
                    connectioninfo::ConnectionInfo::from(stream),
                );
            }
        });

        loop {
            self.tick();
        }
    }
}
