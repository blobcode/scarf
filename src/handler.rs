use std::error::Error;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

// relays tcp connection
pub async fn transfer(mut inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    // create streams
    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    // methods
    let client_to_server = async {
        io::copy(&mut ri, &mut wo).await?;
        wo.shutdown().await
    };

    let server_to_client = async {
        io::copy(&mut ro, &mut wi).await?;
        wi.shutdown().await
    };

    // merge streams
    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}
