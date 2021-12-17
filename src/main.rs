use env_logger::Env;
use futures::FutureExt;
use log::{error, info};
use std::error::Error;
use tokio::net::TcpListener;

mod balancer;
mod config;
mod handler;

// main loop
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup logging
    let env = Env::default().filter_or("MY_LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    // load config (replace this logic later)
    let config = config::load("./scarf.toml".to_string());

    // set listen addr
    let listen_addr = format!("{}:{}", config.ip, config.port);

    let mut addrlist: Vec<String> = Vec::new();

    // print startup info
    let logo = r#"
                          ____
    ______________ ______/ __/
   / ___/ ___/ __ `/ ___/ /_  
  (__  ) /__/ /_/ / /  / __/  
 /____/\___/\__,_/_/  /_/     
    "#;

    println!("{}\n", logo);
    info!("Listening on: {}", listen_addr);

    let services = config.service.unwrap(); // unwrap used b/c it should always be loaded

    for service in services {
        addrlist.push(service.address);
    }

    // open tcp socket
    let listener = TcpListener::bind(listen_addr).await?;

    // handle connections
    while let Ok((inbound, _)) = listener.accept().await {
        // random select the
        let server_addr = balancer::roundrobin(addrlist.to_owned());

        info!("sending to {}", server_addr);
        let transfer = handler::transfer(inbound, server_addr.clone()).map(|r| {
            if let Err(e) = r {
                // error handling
                error!("Failed to transfer; error={}", e);
            }
        });
        // create thread
        tokio::spawn(transfer);
    }

    Ok(())
}
