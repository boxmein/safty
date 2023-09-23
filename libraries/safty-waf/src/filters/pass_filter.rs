use super::filter::{Filter, FilterResult};
use anyhow::Result;
use async_trait::async_trait;
use hyper::{body::Bytes, Body, Request};

#[derive(Clone, Debug)]
pub struct PassFilter;

impl PassFilter {
    #[allow(unused)]
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Filter for PassFilter {
    async fn should_allow(&self, _: &Request<Body>, _: &Bytes) -> Result<FilterResult> {
        Ok(FilterResult::Pass)
    }
}
