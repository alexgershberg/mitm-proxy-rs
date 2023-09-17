use anyhow::Error;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, StatusCode};
use hyper_rustls::HttpsConnectorBuilder;
use rustls::{ClientConfig, RootCertStore};
use std::convert::Infallible;
use std::fs;
use std::io::BufReader;
use std::net::SocketAddr;

async fn handle_request(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("{request:?}");

    let f = fs::File::open("certs/sample.pem").unwrap();
    let mut rd = BufReader::new(f);
    let certs = rustls_pemfile::certs(&mut rd).map_err(|e| eprintln!("Failed to read certificate: {e}")).unwrap();

    // heavy "inspiration" taken from here: https://github.com/rustls/hyper-rustls/blob/main/examples/client.rs
    let mut roots = RootCertStore::empty();
    roots.add_parsable_certificates(&certs);

    let tls = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(roots)
        .with_no_client_auth();

    let https = HttpsConnectorBuilder::new()
        .with_tls_config(tls)
        .https_or_http()
        .enable_http1()
        .build();

    let client = Client::builder().build(https);
    let response = match client.request(request).await {
        Ok(resp) => resp,
        Err(err) => {
            eprintln!("Can't handle this one (YET!) | Error: {err}");
            Response::builder()
                .status(StatusCode::NOT_IMPLEMENTED)
                .body(Body::empty())
                .unwrap()
        }
    };

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 10001));
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

    let server = Server::bind(&addr).serve(make_svc);

    server.await?;

    Ok(())
}
