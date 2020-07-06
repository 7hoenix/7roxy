use bytes::Bytes;
use common::Message;
use futures::sink::SinkExt;
use std::error::Error;
use std::net::SocketAddrV4;
use structopt::StructOpt;
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, FramedWrite};

#[derive(StructOpt)]
#[structopt(
    name = "7roxy Client",
    about = "A personal AI proxy agent terminal client."
)]
struct Opt {
    #[structopt(short, long)]
    daemon_address: SocketAddrV4,

    #[structopt(short, long)]
    set_directive: String,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let Opt {
        daemon_address,
        set_directive,
    } = Opt::from_args();

    let socket = TcpStream::connect(daemon_address).await?;
    println!("Got a connection {:#?}", socket);

    let mut bytes_delimited: FramedWrite<TcpStream, _> =
        FramedWrite::new(socket, BytesCodec::new());

    println!(
        "Received new directive \"{}\", will get right on it.",
        set_directive
    );

    let msg = serde_json::to_string(&Message::FindInformationOn(set_directive))
        .expect("Failed to serialize to JSON");
    bytes_delimited
        .send(Bytes::from(msg))
        .await
        .expect("failed to send");
    Ok(())
}
