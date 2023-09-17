use anyhow::{Error};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, StatusCode};
use hyper_rustls::HttpsConnectorBuilder;
use rustls::{Certificate, ClientConfig, PrivateKey, RootCertStore};
use std::convert::Infallible;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;

// https://docs.rs/rustls/latest/rustls/struct.Certificate.html
fn load_certificates_from_file(path: &str) -> std::io::Result<Vec<Certificate>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let certs = rustls_pemfile::certs(&mut reader)?;

    Ok(certs.into_iter().map(Certificate).collect())
}

// https://docs.rs/rustls/latest/rustls/struct.PrivateKey.html
fn load_private_key_from_file(path: &str) -> Result<PrivateKey, Box<dyn std::error::Error>> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut keys = rustls_pemfile::pkcs8_private_keys(&mut reader)?;

    match keys.len() {
        0 => Err(format!("No PKCS8-encoded private key found in {path}").into()),
        1 => Ok(PrivateKey(keys.remove(0))),
        _ => Err(format!("More than one PKCS8-encoded private key found in {path}").into()),
    }
}
async fn handle_request(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("{request:?}");

    let cer_file = fs::File::open("certs/mitm-proxy-rs.cer").unwrap();
    let mut cer_rd = BufReader::new(cer_file);
    let certs = rustls_pemfile::certs(&mut cer_rd).map_err(|e| eprintln!("Failed to read certificate: {e}")).unwrap();



    let cer = load_certificates_from_file("certs/mitm-proxy-rs.cer").unwrap();
    let pkey = load_private_key_from_file("certs/mitm-proxy-rs.key").unwrap();


    // heavy "inspiration" taken from here: https://github.com/rustls/hyper-rustls/blob/main/examples/client.rs
    let mut roots = RootCertStore::empty();
    roots.add_parsable_certificates(&certs);

    let tls = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(roots)
        .with_client_auth_cert(cer, pkey)
        .unwrap();

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
