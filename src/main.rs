use std::error::Error;
use tokio::net::TcpListener;

mod config;
mod handler;

// main loop
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load config (replace this logic later)
    let config = config::load("./scarf.toml".to_string());

    // set listen addr
    let listen_addr = format!("{}:{}", config.ip, config.port);

    // ! replace later
    let server_addr = "127.0.0.1:5000".to_string();

    // print startup info
    println!("Listening on: {}", listen_addr);
    println!("Proxying to: {}", server_addr);

    // open tcp socket
    let listener = TcpListener::bind(listen_addr).await?;

    // handle connections
    while let Ok((inbound, _)) = listener.accept().await {
        let transfer = handler::transfer(inbound, server_addr.clone()).map(|r| {
            if let Err(e) = r {
                // error handling
                println!("Failed to transfer; error={}", e);
            }
        });
        // create thread
        tokio::spawn(transfer);
    }

    Ok(())
}
