use std::{error::Error, net::SocketAddrV4};
use structopt::StructOpt;
use tokio::net::UdpSocket;

#[derive(StructOpt)]
#[structopt(name = "7roxy Client", about = "A personal AI proxy agent client.")]
struct Opt {
    #[structopt(short = "i", long)]
    should_initiate: bool,

    #[structopt(short, long)]
    local_address: SocketAddrV4,

    #[structopt(short, long)]
    remote_address: SocketAddrV4,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let Opt {
        local_address,
        remote_address,
        should_initiate,
    } = Opt::from_args();

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
