use crate::filters::Filter;
use anyhow::Result;
use async_trait::async_trait;
use hyper::{body::Bytes, Body, Request};
use tracing::debug;

use super::FilterResult;

#[derive(Debug)]
pub struct SingleQuotesFilter;

impl SingleQuotesFilter {
    #[allow(unused)]
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Filter for SingleQuotesFilter {
    async fn should_allow(&self, _: &Request<Body>, bytes: &Bytes) -> Result<FilterResult> {
        // 0x27 = '
        if bytes.contains(&0x27u8) {
            debug!("SingleQuotesFilter: payload contains a single quote");
            Ok(FilterResult::Drop)
        } else {
            debug!("SingleQuotesFilter: payload seems fine");
            Ok(FilterResult::NotMyProblem)
        }
    }
}
