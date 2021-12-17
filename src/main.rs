use comfy_table::Table;
use futures::FutureExt;
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
    let server_addr = "127.0.0.1:5000".to_string();

    // print startup info
    let logo = r#"
                          ____
    ______________ ______/ __/
   / ___/ ___/ __ `/ ___/ /_  
  (__  ) /__/ /_/ / /  / __/  
 /____/\___/\__,_/_/  /_/     
    "#;

    println!("{}\n", logo);
    println!("Listening on: {}", listen_addr);
    println!("Proxying to: {}\n", server_addr);

    // print table listing active servers
    let mut servicetable = Table::new();
    servicetable.set_header(vec!["service", "address", "status"]);

    let services = config.service.unwrap(); // unwrap used b/c it should always be loaded

    for service in services {
        servicetable.add_row(vec![service.name, service.address, "ok".to_string()]);
    }

    println!("{}", servicetable);

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
