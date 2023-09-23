use super::filter::{Filter, FilterResult};
use anyhow::Result;
use async_trait::async_trait;
use hyper::{body::Bytes, Body, Request};

#[derive(Clone, Debug)]
pub struct InvertFilter<T: Filter>(Box<T>);

#[async_trait]
impl<T: Filter> Filter for InvertFilter<T> {
    async fn should_allow(&self, req: &Request<Body>, bytes: &Bytes) -> Result<FilterResult> {
        let res = self.0.should_allow(req, &bytes).await?;
        Ok(if res == FilterResult::Pass {
            FilterResult::Drop
        } else if res == FilterResult::Drop {
            FilterResult::Pass
        } else {
            FilterResult::NotMyProblem
        })
    }
}
