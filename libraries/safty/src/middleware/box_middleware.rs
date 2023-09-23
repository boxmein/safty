use super::traits::Middleware;
use anyhow::Result;
use async_trait::async_trait;
use hyper::{body::Bytes, Body, Request, Response};
use std::fmt::Debug;

#[derive(Debug)]
pub struct BoxMiddleware(Box<dyn Middleware>);
impl BoxMiddleware {
    pub fn new(middleware: Box<dyn Middleware>) -> Self {
        Self(middleware)
    }
}
#[async_trait]
impl Middleware for BoxMiddleware {
    async fn handle(&self, request: Request<Body>, body: Bytes) -> Result<Response<Body>> {
        self.0.handle(request, body).await
    }
}
