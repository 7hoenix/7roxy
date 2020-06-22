use std::{error::Error, net::SocketAddrV4};
use tokio::net::{UdpSocket};
use tokio_openssl::{connect};
use openssl::ssl::{SslMethod, SslConnector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr: SocketAddrV4 = "127.0.0.1:19324".parse()?;

    let socket = UdpSocket::bind(addr).await?;

    let ssl_connector = SslConnector::builder(SslMethod::dtls())?.build();

    let configuration = ssl_connector.configure()?;

    connect(configuration, "", socket).await?;

    Ok(())
}
