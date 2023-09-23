use anyhow::Result;
use async_trait::async_trait;
use hyper::{body::Bytes, Body, Request};
use std::fmt::Debug;

#[derive(PartialEq, Eq, Debug)]
pub enum FilterResult {
    Pass,
    Drop,
    NotMyProblem,
}

#[async_trait]
pub trait Filter: Send + Sync + Debug {
    async fn should_allow(&self, req: &Request<Body>, body: &Bytes) -> Result<FilterResult>;
}
