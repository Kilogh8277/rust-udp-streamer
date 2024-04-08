use std::{io::{Read, Write}, net::{SocketAddr, TcpListener, TcpStream}};

pub struct TcpClient {
    stream: TcpStream,
}

impl TcpClient {
    pub fn connect(addr: &str, port: u16) -> Result<Self, std::io::Error> {
        let this_addr = format!("{}:{}", addr, port);
        let stream = TcpStream::connect(this_addr)?;
        Ok(TcpClient { stream })
    }

    pub fn send(&mut self, data: &[u8]) -> Result<usize, std::io::Error> {
        self.stream.write(data)
    }

    pub fn receive(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.stream.read(buf)
    }
}

pub struct TcpServer {
    listener: TcpListener,
}

impl TcpServer {
    pub fn bind(addr: &str, port: u16) -> Result<Self, std::io::Error> {
        let this_addr = format!("{}:{}", addr, port);
        let listener = TcpListener::bind(this_addr)?;
        Ok(TcpServer { listener })
    }

    pub fn accept(&self) -> Result<(TcpStream, SocketAddr), std::io::Error> {
        self.listener.accept()
    }
}