mod capability;

pub use crate::capability::http::{make_request, stack_exchange, Target};
use common::Message;
use std::{error::Error, net::SocketAddrV4};
use tokio::net::TcpStream;
use tokio::{net::UdpSocket, stream::StreamExt};
use tokio_util::codec::{BytesCodec, FramedRead};

pub async fn process(socket: TcpStream) {
    let mut framed = FramedRead::new(socket, BytesCodec::new());
    let bytes = framed
        .next()
        .await
        .expect("failed to read from stream")
        .expect("invalid format from client");
    let message: Message = serde_json::from_reader(bytes.as_ref()).expect("failed to read");
    match message {
        Message::FindInformationOn(search) => {
            println!("Got a directive to find information on \"{}\"", search);
            let res = make_request(
                search,
                Target::StackExchange(stack_exchange::Site::StackOverflow),
            )
            .await;
            match res {
                Ok(_) => println!("made it "),
                Err(_) => println!("Didn't make it"),
            }
        }
        Message::SchedulePairing(pairing_partner) => println!(
            "Got a directive to schedule a pairing session with {}",
            pairing_partner
        ),
    }
}

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
