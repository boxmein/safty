use anyhow::Result;
use async_trait::async_trait;
use hyper::{body::Bytes, Body, Request, Response};
use std::fmt::Debug;

#[async_trait]
pub trait Middleware: Send + Sync + Debug {
    async fn handle(&self, request: Request<Body>, body: Bytes) -> Result<Response<Body>>;
}
