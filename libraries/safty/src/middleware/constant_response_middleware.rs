use super::traits::Middleware;
use anyhow::Result;
use async_trait::async_trait;
use hyper::{body::Bytes, Body, Request, Response, StatusCode};
use std::fmt::Debug;
use tracing::info;

#[derive(Debug)]
pub struct ConstantResponseMiddleware {
    status_code: StatusCode,
    body: String,
}

impl ConstantResponseMiddleware {
    pub fn new(status_code: StatusCode, body: String) -> Self {
        Self { status_code, body }
    }
}

#[async_trait]
impl Middleware for ConstantResponseMiddleware {
    async fn handle(&self, req: Request<Body>, _: Bytes) -> Result<Response<Body>> {
        info!("❌ {} {}", req.method(), req.uri());

        Ok(Response::builder()
            .status(self.status_code)
            .body(Body::from(self.body.clone()))?)
    }
}
