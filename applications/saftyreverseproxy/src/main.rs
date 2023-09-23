mod proxy;

use anyhow::Result;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Client, Server, StatusCode};
use once_cell::sync::OnceCell;
use safty::middleware::{
    BoxMiddleware, ConstantResponseMiddleware, PassMiddleware, SetHeaderMiddleware,
};
use safty_waf::filters::{AllPassFilter, DropFilter, Filter, PassFilter, ScopeRegexPathFilter};
use safty_waf::FilterMiddleware;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

use crate::proxy::Proxy;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let host = [0, 0, 0, 0];
    let port = 8080;

    println!("more logs: set RUST_LOG=trace");
    let addr = SocketAddr::from((host, port));

    setup_proxy();

    let service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|req| get_proxy().handle(req)))
    });

    let serv = Server::bind(&addr).serve(service);

    println!("safty by @boxmein running on {}", addr);
    if let Err(e) = serv.await {
        eprintln!("server error: {}", e);
    };

    Ok(())
}

static PROXY: OnceCell<Proxy<BoxMiddleware>> = OnceCell::new();

fn setup_proxy() {
    let uri = env::var("DOWNSTREAM_URI")
        .expect("Set env DOWNSTREAM_URI")
        .parse()
        .expect("Invalid URI");

    let filt = get_filter();

    let fail = ConstantResponseMiddleware::new(
        StatusCode::FORBIDDEN,
        "no https://www.youtube.com/watch?v=6n3pFFPSlW4".to_owned(),
    );
    let pass = PassMiddleware::new(Client::new(), uri);
    let set_header = SetHeaderMiddleware::new(pass, "foo", "bar");
    let mw = FilterMiddleware::new(fail, set_header, filt);
    let finalmw = BoxMiddleware::new(Box::new(mw));
    let proxy: Proxy<BoxMiddleware> = Proxy::new(finalmw);
    PROXY.set(proxy).expect("failed to set");
}

fn get_filter() -> impl Filter {
    AllPassFilter::new(vec![
        // This entire region of the app needs to be set on fire...
        Box::new(ScopeRegexPathFilter::new("^/admin", DropFilter::new())),
        Box::new(PassFilter::new()),
    ])
}

fn get_proxy() -> &'static Proxy<BoxMiddleware> {
    PROXY.get().expect("Failed to get proxy")
}
