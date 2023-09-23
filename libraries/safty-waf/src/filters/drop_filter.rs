use super::filter::{Filter, FilterResult};
use anyhow::Result;
use async_trait::async_trait;
use hyper::{body::Bytes, Body, Request};

#[derive(Clone, Debug)]
pub struct DropFilter;

impl DropFilter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Filter for DropFilter {
    async fn should_allow(&self, _: &Request<Body>, _: &Bytes) -> Result<FilterResult> {
        Ok(FilterResult::Drop)
    }
}
