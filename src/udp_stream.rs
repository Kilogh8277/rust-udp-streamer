use std::net::{UdpSocket, SocketAddr};

pub struct UdpStream {
    socket: UdpSocket,
}

impl UdpStream {
    pub fn new(ip: &str, port: u16) -> Result<Self, std::io::Error> {
        let addr = format!("{}:{}", ip, port);
        let socket = UdpSocket::bind(addr)?;
        Ok(UdpStream { socket })
    }

    pub fn send(&self, data: &[u8], dest: SocketAddr) -> Result<usize, std::io::Error> {
        self.socket.send_to(data, dest)
    }

    pub fn receive(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr), std::io::Error> {
        self.socket.recv_from(buf)
    }
}