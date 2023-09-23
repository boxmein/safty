use super::filter::{Filter, FilterResult};
use anyhow::Result;
use async_trait::async_trait;
use hyper::{body::Bytes, Body, Request};

#[derive(Clone, Debug)]
pub struct PassPathFilter(String);

#[async_trait]
impl Filter for PassPathFilter {
    async fn should_allow(&self, req: &Request<Body>, _: &Bytes) -> Result<FilterResult> {
        Ok(if req.uri().path() == &self.0 {
            FilterResult::Pass
        } else {
            FilterResult::Drop
        })
    }
}
