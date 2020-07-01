use std::{error::Error, net::SocketAddrV4};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "7roxy Daemon", about = "A personal AI proxy agent daemon.")]
struct Opt {
    #[structopt(short, long)]
    local_address: SocketAddrV4,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let Opt {
        local_address,
    } = Opt::from_args();

    Ok(())
}
