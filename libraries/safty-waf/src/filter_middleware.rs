use anyhow::Result;
use async_trait::async_trait;
use hyper::{body::Bytes, Body, Request, Response};
use safty::middleware::Middleware;
use std::fmt::Debug;

use crate::filters::{Filter, FilterResult};

#[derive(Debug)]
pub struct FilterMiddleware<T: Filter, U: Middleware, V: Middleware> {
    fail: U,
    pass: V,
    filter: T,
}

impl<T: Filter, U: Middleware, V: Middleware> FilterMiddleware<T, U, V> {
    pub fn new(fail: U, pass: V, filter: T) -> Self {
        Self { fail, pass, filter }
    }
}

#[async_trait]
impl<T: Filter, U: Middleware, V: Middleware> Middleware for FilterMiddleware<T, U, V> {
    async fn handle(&self, req: Request<Body>, bytes: Bytes) -> Result<Response<Body>> {
        if self.filter.should_allow(&req, &bytes).await? == FilterResult::Drop {
            return self.fail.handle(req, bytes).await;
        }
        self.pass.handle(req, bytes).await
    }
}
