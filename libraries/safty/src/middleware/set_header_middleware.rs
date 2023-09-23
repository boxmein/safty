use super::Middleware;
use anyhow::Result;
use async_trait::async_trait;
use hyper::{
    body::Bytes,
    http::{HeaderName, HeaderValue},
    Body, Request, Response,
};
use std::fmt::Debug;

#[derive(Debug)]
pub struct SetHeaderMiddleware<T: Middleware> {
    header_name: HeaderName,
    header_value: HeaderValue,
    next: T,
}

impl<T: Middleware> SetHeaderMiddleware<T> {
    pub fn new<HN, HV>(next: T, header_name: HN, header_value: HV) -> Self
    where
        HeaderName: TryFrom<HN>,
        <HeaderName as TryFrom<HN>>::Error: Debug,
        HeaderValue: TryFrom<HV>,
        <HeaderValue as TryFrom<HV>>::Error: Debug,
    {
        Self {
            next,
            header_name: header_name
                .try_into()
                .expect("header should parse as HeaderName"),
            header_value: header_value
                .try_into()
                .expect("header should parse as HeaderValue"),
        }
    }
}

#[async_trait]
impl<T: Middleware> Middleware for SetHeaderMiddleware<T> {
    async fn handle(&self, mut request: Request<Body>, body: Bytes) -> Result<Response<Body>> {
        let hdr = request.headers_mut();
        hdr.remove(self.header_name.clone());
        hdr.append(self.header_name.clone(), self.header_value.clone());
        self.next.handle(request, body).await
    }
}

#[cfg(test)]
mod tests {
    use super::SetHeaderMiddleware;
    use crate::middleware::Middleware;
    use anyhow::Result;
    use async_trait::async_trait;
    use hyper::{body::Bytes, Body, Request, Response};
    use tokio::sync::mpsc::{channel, Receiver, Sender};

    #[derive(Debug)]
    struct DummyMw(Sender<Request<Body>>);

    impl DummyMw {
        pub fn new() -> (DummyMw, Receiver<Request<Body>>) {
            let (tx, rx) = channel::<Request<Body>>(1);
            (Self(tx), rx)
        }
    }
    #[async_trait]
    impl Middleware for DummyMw {
        async fn handle(&self, request: Request<Body>, _: Bytes) -> Result<Response<Body>> {
            self.0.send(request).await?;
            Ok(Response::builder().body(Body::from("nice"))?)
        }
    }

    #[tokio::test]
    async fn it_sets_header() {
        let mut req = Request::builder()
            .uri("https://foo.com")
            .header("foo", "bar")
            .body(Body::from("body"))
            .expect("failed to create request");

        let (next, mut recv) = DummyMw::new();
        let mw = SetHeaderMiddleware::new(next, "beep", "beep2");

        let body = hyper::body::to_bytes(req.body_mut())
            .await
            .expect("failed to parse");

        let _ = mw.handle(req, body).await.expect("wow");

        let req = recv.try_recv().expect("http request");

        let headers: Vec<String> = req.headers().keys().map(|h| h.to_string()).collect();
        assert_eq!(headers, vec!["foo", "beep"]);
    }
}
