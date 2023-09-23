use super::filter::{Filter, FilterResult};
use anyhow::Result;
use async_trait::async_trait;
use hyper::{body::Bytes, Body, Request};
use tracing::debug;

pub struct AllPassFilter(Vec<Box<dyn Filter>>);

impl AllPassFilter {
    pub fn new(filters: Vec<Box<dyn Filter>>) -> Self {
        Self(filters)
    }
}

impl std::fmt::Debug for AllPassFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AllPassFilter").field(&self.0.len()).finish()
    }
}

#[async_trait]
impl Filter for AllPassFilter {
    async fn should_allow(&self, req: &Request<Body>, bytes: &Bytes) -> Result<FilterResult> {
        for f in self.0.iter() {
            debug!("AllPassFilter trying filter {:?}", f);
            let result = f.should_allow(req, bytes).await?;

            // NotMyProblem takes next filter
            if result == FilterResult::Pass || result == FilterResult::Drop {
                debug!("AllPassFilter: filter {:?} gave result {:?}", f, result);
                return Ok(result);
            }
        }

        Ok(FilterResult::Pass)
    }
}
