use std::{error::Error, net::SocketAddrV4};
use tokio::net::TcpStream;
use tokio::net::UdpSocket;

pub async fn process(socket: TcpStream) {}

pub async fn ping(
    local_address: &SocketAddrV4,
    remote_address: &SocketAddrV4,
    should_initiate: bool,
) -> Result<(), Box<dyn Error>> {
    let mut socket = UdpSocket::bind(local_address).await?;

    socket.connect(remote_address).await?;

    let message = b"hi Zephyr";

    if should_initiate {
        let number_of_bytes_sent = socket.send(message).await?;
        println!("Number of bytes sent {}", number_of_bytes_sent);
    } else {
        let mut buffer = [0; 512];
        let number_of_bytes_received = socket.recv(&mut buffer).await?;
        println!("Number of bytes received {}", number_of_bytes_received);
        println!("Message {}", String::from_utf8(buffer.to_vec())?);
    }

    Ok(())
}
