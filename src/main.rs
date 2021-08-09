use std::sync::Arc;

use prometheus::Encoder;
use rustyocean::{api, register_metrics, update_metrics};

use lazy_static::lazy_static;

lazy_static! {
    static ref REGISTRY: prometheus::Registry =
        prometheus::Registry::new_custom(Some("digitalocean".to_owned()), None).unwrap();
}

async fn handle(_req: hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, String> {
    let mut buffer = Vec::new();
    let encoder = prometheus::TextEncoder::new();

    let metrics = REGISTRY.gather();

    encoder.encode(&metrics, &mut buffer).unwrap();
    Ok(hyper::Response::new(hyper::Body::from(buffer)))
}

async fn run_server() {
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 9100));

    let make_service = hyper::service::make_service_fn(|_conn| async {
        Ok::<_, String>(hyper::service::service_fn(handle))
    });

    let server = hyper::Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        tracing::error!("Running Webserver: {:?}", e);
    }
}

fn main() {
    // Setting up the logging/tracing stuff
    let log_level = std::env::var("LOG").unwrap_or("info".to_string());
    let tracing_directive_str = format!("rustyocean={}", log_level);
    let tracing_sub = tracing_subscriber::FmtSubscriber::builder()
        .json()
        .with_level(true)
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing_directive_str.parse().unwrap()),
        )
        .finish();
    tracing::subscriber::set_global_default(tracing_sub)
        .expect("Setting initial Tracing-Subscriber");

    tracing::info!("Starting...");

    let token = std::env::var("DIGITALOCEAN_TOKEN").unwrap();
    let client = Arc::new(api::API::new(token));

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    register_metrics(&REGISTRY);

    rt.spawn(update_metrics(client.clone()));

    rt.block_on(run_server());
}
