use std::{error::Error, net::SocketAddrV4};
use tokio::net::UdpSocket;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "7roxy", about = "A personal AI proxy agent.")]
struct Opt {
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
    } = Opt::from_args();

    let socket = UdpSocket::bind(local_address).await?;

    socket.connect(remote_address).await?;

    Ok(())
}
