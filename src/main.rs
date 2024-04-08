use std::io::Read;

use crate::udp_stream::UdpStream;
use crate::tcp_stream::TcpServer;

mod tcp_stream;
mod udp_stream;

fn main() -> std::io::Result<()>{
    let udp_socket = UdpStream::new("0.0.0.0", 5000).expect("Failed to create a UdpStream implementation");
    let tcp_socket = TcpServer::bind("0.0.0.0", 5001).expect("Failed to create a TcpServer implementation");
    let mut buf: [u8; 2500] = [0; 2500];
    loop {
        println!("Send a UDP message");
        let (amt, src) = udp_socket.receive(&mut buf).expect("Failed to receive a valid UDP message");

        print!("Received UDP message from {1}: {0}", std::str::from_utf8_mut(&mut buf[..amt]).unwrap(), src);

        buf = [0; 2500];

        println!("Send a TCP message");

        let (mut client_stream, client_addr) = tcp_socket.accept().expect("Failed to receive TCP data");

        client_stream.read(&mut buf).expect("Failed to read TCP stream");

        print!("Received TCP message from {1}: {0}", std::str::from_utf8_mut(&mut buf[..amt]).unwrap(), client_addr);

        // socket.send_to(&buf, "192.168.68.50:5000").expect("Couldn't send the data");
    }
}
