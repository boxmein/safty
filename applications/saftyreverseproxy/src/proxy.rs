use anyhow::Result;
use hyper::{body::HttpBody, Body, Request, Response};
use safty::middleware::Middleware;
use tracing::debug;
use tracing::warn;

#[derive(Debug)]
pub struct Proxy<T: Middleware> {
    middleware: T,
}

impl<T: Middleware> Proxy<T> {
    pub fn new(middleware: T) -> Self {
        Self { middleware }
    }

    pub async fn handle(&self, mut req: Request<Body>) -> Result<Response<Body>> {
        let size = req.body().size_hint().upper().unwrap_or(0);
        debug!(
            "evaluating request: {} {:?} (approx. {} bytes)",
            req.method(),
            req.uri().path(),
            size,
        );
        if size > 10_000_000 {
            warn!("size is over 10 MB");
        }
        let bytes = hyper::body::to_bytes(req.body_mut()).await?;

        self.middleware.handle(req, bytes).await
    }
}
