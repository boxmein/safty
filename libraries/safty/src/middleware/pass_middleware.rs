use super::traits::Middleware;
use anyhow::Result;
use async_trait::async_trait;
use hyper::{body::Bytes, client::HttpConnector, Body, Request, Response, Uri};
use std::fmt::Debug;
use tracing::info;

#[derive(Debug)]
pub struct PassMiddleware {
    client: hyper::Client<HttpConnector>,
    uri: Uri,
}
impl PassMiddleware {
    pub fn new(client: hyper::Client<HttpConnector>, uri: Uri) -> Self {
        Self { client, uri }
    }

    fn get_modified_uri(&self, uri: &Uri) -> Result<Uri> {
        let mut b = Uri::builder();
        if let Some(scheme) = self.uri.scheme() {
            b = b.scheme((*scheme).clone());
        };
        if let Some(authority) = self.uri.authority() {
            b = b.authority((*authority).clone());
        }
        if let Some(path_and_query) = uri.path_and_query() {
            b = b.path_and_query((*path_and_query).clone());
        }

        let res = b.build()?;
        debug_assert!(res.authority().is_some());
        debug_assert!(res.scheme().is_some());
        debug_assert!(res.path_and_query().is_some());
        Ok(res)
    }
}
#[async_trait]
impl Middleware for PassMiddleware {
    async fn handle(&self, mut req: Request<Body>, bytes: Bytes) -> Result<Response<Body>> {
        info!("✅ {} {}", req.method(), req.uri());
        let client = self.client.clone();
        *req.uri_mut() = self.get_modified_uri(req.uri())?;
        *req.body_mut() = Body::from(bytes);
        Ok(client.request(req).await?)
    }
}
