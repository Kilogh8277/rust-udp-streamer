use std::net::UdpSocket;

mod UdpStream {

}

fn main() -> std::io::Result<()>{
    loop {
        let socket = UdpSocket::bind("192.168.68.52:5000").expect("Couldn't bind to the address");

        let mut buf: [u8; 2500] = [0; 2500];

        let (amt, _src) = socket.recv_from(&mut buf)?;

        print!("{}", std::str::from_utf8_mut(&mut buf[..amt]).unwrap());

        // socket.send_to(&buf, "192.168.68.50:5000").expect("Couldn't send the data");
    }
}
