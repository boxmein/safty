use super::filter::{Filter, FilterResult};
use anyhow::Result;
use async_trait::async_trait;
use hyper::{body::Bytes, Body, Method, Request};
use regex::Regex;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct ScopeToPathFilter<T: Filter>(String, T);

#[async_trait]
impl<T: Filter> Filter for ScopeToPathFilter<T> {
    async fn should_allow(&self, req: &Request<Body>, bytes: &Bytes) -> Result<FilterResult> {
        if req.uri().path() == &self.0 {
            self.1.should_allow(req, bytes).await
        } else {
            Ok(FilterResult::NotMyProblem)
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScopeRegexPathFilter<T: Filter> {
    pat: Regex,
    filter: T,
}

impl<T: Filter> ScopeRegexPathFilter<T> {
    pub fn new(pattern: &str, filter: T) -> Self {
        let pat = Regex::new(pattern).unwrap();
        Self { pat, filter }
    }
}

#[async_trait]
impl<T: Filter> Filter for ScopeRegexPathFilter<T> {
    async fn should_allow(&self, req: &Request<Body>, bytes: &Bytes) -> Result<FilterResult> {
        if self.pat.is_match(req.uri().path()) {
            debug!("ScopeRegexPathFilter: testing {} matched", req.uri().path());
            Ok(self.filter.should_allow(req, bytes).await?)
        } else {
            debug!("ScopeRegexPathFilter: no match: {}", req.uri().path());
            Ok(FilterResult::NotMyProblem)
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScopeRegexMethodPathFilter<T: Filter> {
    method: Method,
    pat: Regex,
    filter: T,
}

impl<T: Filter> ScopeRegexMethodPathFilter<T> {
    #[allow(unused)]
    pub fn new(method: Method, pattern: &str, filter: T) -> Self {
        let pat = Regex::new(pattern).unwrap();
        Self {
            method,
            pat,
            filter,
        }
    }
}

#[async_trait]
impl<T: Filter> Filter for ScopeRegexMethodPathFilter<T> {
    async fn should_allow(&self, req: &Request<Body>, bytes: &Bytes) -> Result<FilterResult> {
        if &self.method == req.method() && self.pat.is_match(req.uri().path()) {
            debug!(
                "ScopeRegexMethodPathFilter: testing {} matched",
                req.uri().path()
            );
            Ok(self.filter.should_allow(req, bytes).await?)
        } else {
            debug!("ScopeRegexMethodPathFilter: no match: {}", req.uri().path());
            Ok(FilterResult::NotMyProblem)
        }
    }
}
