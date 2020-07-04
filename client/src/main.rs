use common::Message;
use std::error::Error;
use std::net::{SocketAddrV4, TcpStream};
use structopt::StructOpt;

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

fn main() -> Result<(), Box<dyn Error>> {
    let Opt {
        daemon_address,
        set_directive,
    } = Opt::from_args();

    let stream = TcpStream::connect(daemon_address)?;
    println!("Got a connection {:#?}", stream);
    println!(
        "Received new directive \"{}\", will get right on it.",
        set_directive
    );
    serde_json::to_writer(stream, &Message::FindInformationOn(set_directive))
        .expect("Failed to write");

    Ok(())
}
