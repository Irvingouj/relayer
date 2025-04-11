use std::time::Duration;
use std::{env, io, net::SocketAddr};
use tokio::io::copy_bidirectional;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;
use tokio_graceful_shutdown::{SubsystemBuilder, SubsystemHandle, Toplevel};
use url::Url;

async fn relay_subsystem(
    subsys: SubsystemHandle,
    listen_addr: SocketAddr,
    target_addr: String,
) -> io::Result<()> {
    let listener = TcpListener::bind(listen_addr).await?;
    println!("Listening on {}", listen_addr);
    println!("Relaying to {}", target_addr);

    loop {
        tokio::select! {
            accept_result = listener.accept() => {
                match accept_result {
                    Ok((inbound, _)) => {
                        let target = target_addr.clone();
                        tokio::spawn(async move {
                            match timeout(Duration::from_secs(5), TcpStream::connect(&target)).await {
                                Ok(Ok(outbound)) => {
                                    if let Err(e) = transfer(inbound, outbound).await {
                                        eprintln!("relay error: {}", e);
                                    }
                                }
                                Ok(Err(e)) => eprintln!("connect failed: {}", e),
                                Err(_) => eprintln!("connect timeout to {}", target),
                            }
                        });
                    }
                    Err(e) => {
                        eprintln!("accept error: {}", e);
                        break;
                    }
                }
            }
            _ = subsys.on_shutdown_requested() => {
                println!("Shutdown requested, stopping relay server");
                break;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let endpoint_url = env::var("RELAYER_ENDPOINT").expect("RELAYER_ENDPOINT must be set");
    let url = Url::parse(&endpoint_url).expect("Invalid RELAYER_ENDPOINT URL");

    let host = url.host_str().expect("Missing host in RELAYER_ENDPOINT");
    let port = url.port().unwrap_or(80);
    let target_addr = format!("{}:{}", host, port);

    let listen_port = env::var("RELAYER_LISTENING_PORT").unwrap_or_else(|_| "3000".into());
    let listen_addr: SocketAddr = format!("0.0.0.0:{}", listen_port)
        .parse()
        .expect("Invalid RELAYER_LISTENING_PORT");

    Toplevel::new(move |s| {
        let target_addr = target_addr.clone();
        let listen_addr = listen_addr;
        async move {
            s.start(SubsystemBuilder::new("Relayer", move |handle| {
                relay_subsystem(handle, listen_addr, target_addr)
            }));
        }
    })
    .catch_signals()
    .handle_shutdown_requests(Duration::from_secs(10))
    .await?;

    Ok(())
}

async fn transfer(mut inbound: TcpStream, mut outbound: TcpStream) -> io::Result<()> {
    copy_bidirectional(&mut inbound, &mut outbound).await?;
    Ok(())
}
