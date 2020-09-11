use std::io::ErrorKind;
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct ConnectionInfo {
    reader: std::io::BufReader<TcpStream>,
    writer: std::io::BufWriter<TcpStream>,
}

impl ConnectionInfo {
    pub fn from(stream: TcpStream) -> Self {
        stream.set_nonblocking(true).unwrap();
        Self {
            reader: std::io::BufReader::new(stream.try_clone().unwrap()),
            writer: std::io::BufWriter::new(stream.try_clone().unwrap()),
        }
    }

    pub fn send<T>(&mut self, data: T) -> bool
    where
        T: serde::Serialize,
    {
        let data = serde_json::to_string(&data).unwrap();
        println!("{:#?}", data);
        let res = self.send_str(data.as_bytes());
        if let Ok(()) = res {
            return true;
        }
        match res.unwrap_err().kind() {
            ErrorKind::WouldBlock => {
                return false;
            }
            e => {
                panic!("{:#?}", e);
            }
        }
    }

    pub fn send_str(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.writer.write_all(data)
    }

    pub fn try_read(&mut self) -> Option<String> {
        let mut data = String::new();
        let res = self.reader.read_to_string(&mut data);
        res.map(|_| Some(data))
            .map_err(|e| match e.kind() {
                ErrorKind::WouldBlock => None::<String>,
                e => {
                    panic!("try_read: {:#?}", e);
                    // Some(String::new())
                }
            })
            .unwrap()
    }
}
