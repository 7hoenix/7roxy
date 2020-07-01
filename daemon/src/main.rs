use daemon::process;
use std::{error::Error, net::SocketAddrV4};
use structopt::StructOpt;
use tokio::net::TcpListener;

#[derive(StructOpt)]
#[structopt(name = "7roxy Daemon", about = "A personal AI proxy agent daemon.")]
struct Opt {
    #[structopt(short, long)]
    local_address: SocketAddrV4,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let Opt { local_address } = Opt::from_args();

    let mut listener = TcpListener::bind(&local_address).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        println!("Connection accepted {:?}", socket);
        tokio::spawn(process(socket));
    }

    Ok(())
}
