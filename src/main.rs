use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Level, Verbosity};
use futures_util::StreamExt;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::rustls::ClientConfig;
use tokio_rustls::TlsConnector;
use tokio_tungstenite::client_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tracing::info;
use tracing::level_filters::LevelFilter;

/// CLI arguments
#[derive(Parser, Debug)]
#[command(about, version)]
struct Args {
    /// Tunnel host
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    host: String,

    /// Tunnel port
    #[arg(long, default_value_t = 9091)]
    port: u16,

    /// hack.chat real IP
    #[arg(long, default_value_t = String::from("104.131.138.176"))]
    ip: String,

    /// Dummy domain to bypass SNI
    #[arg(long, default_value_t = String::from("bake.lyka.pro"))]
    domain: String,

    /// hack.chat WS url
    #[arg(long, default_value_t = String::from("wss://hack.chat/chat-ws"))]
    ws: String,

    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse CLI args
    let Args {
        host,
        port,
        ip,
        domain,
        ws,
        verbose,
    } = Args::parse();

    // Setup tracing filter
    let filter = match verbose.log_level() {
        None => LevelFilter::OFF,
        Some(Level::Error) => LevelFilter::ERROR,
        Some(Level::Warn) => LevelFilter::WARN,
        Some(Level::Info) => LevelFilter::INFO,
        Some(Level::Debug) => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    };

    // Setup tracing subscriber
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_max_level(filter)
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    // Create rustls config
    let config = {
        // Add native certs
        let mut root_cert_store = tokio_rustls::rustls::RootCertStore::empty();
        let certs = rustls_native_certs::load_native_certs()?;
        for cert in certs {
            root_cert_store.add(cert)?;
        }

        let config = tokio_rustls::rustls::ClientConfig::builder()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth();

        Arc::new(config)
    };

    let addr = format!("{host}:{port}");
    let listener = TcpListener::bind(&addr).await?;

    info!("server listening on {addr}");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(
            stream,
            config.clone(),
            ip.clone(),
            domain.clone(),
            ws.clone(),
        ));
    }

    Ok(())
}

async fn accept_connection(
    client_stream: TcpStream,
    config: Arc<ClientConfig>,
    ip: String,
    domain: String,
    ws: String,
) -> anyhow::Result<()> {
    info!("recieved client connection");

    // Connect to hack.chat
    let (server_write, server_read) = {
        // TLS connect
        let connector = TlsConnector::from(config);
        let addr = format!("{ip}:443",).parse::<SocketAddr>()?;
        let tcp_stream = TcpStream::connect(addr).await?;
        let tls_stream = connector.connect(domain.try_into()?, tcp_stream).await?;

        // WS handshake
        let request = ws.into_client_request()?;
        let (ws_stream, _) = client_async(request, tls_stream).await?;
        ws_stream.split()
    };

    info!("connected to hack.chat");

    // Accept connection from browser
    let client_stream = tokio_tungstenite::accept_async(client_stream).await?;
    let (client_write, client_read) = client_stream.split();

    info!("start forwarding");

    tokio::select! {
        res = server_read.forward(client_write) => res,
        res = client_read.forward(server_write) => res
    }?;

    info!("terminated forwarding");

    Ok(())
}
