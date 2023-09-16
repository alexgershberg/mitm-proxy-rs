use anyhow::Error;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle_request(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    // let mut response = Response::new(Body::empty());

    // match (request.method(), request.uri().path()) {
    //     (&Method::GET, "/") => {
    //         *response.body_mut() = Body::from("Try posting data to /echo");
    //     }
    //
    //     (&Method::POST, "/echo") => *response.body_mut() = request.into_body(),
    //
    //     _ => {
    //         *response.status_mut() = StatusCode::NOT_FOUND;
    //     }
    // }

    // println!("{request:?}");
    // if let Some(host) = request.headers().get("host") {
    //     let client = Client::new();
    //     client.request(request).await.unwrap();
    // } else {
    //     eprintln!("Host not found?");
    // }

    println!("{request:?}");
    let client = Client::new();
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
