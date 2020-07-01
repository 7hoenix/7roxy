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
}

fn main() -> Result<(), Box<dyn Error>> {
    let Opt { daemon_address } = Opt::from_args();

    let stream = TcpStream::connect(daemon_address)?;
    println!("Got a connection {:#?}", stream);
    Ok(())
}
