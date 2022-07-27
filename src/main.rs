#![allow(unused)]

extern crate core;

use std::error::Error;

use hyper::{body::HttpBody as _, Client, Uri};

mod another;
mod event;
mod semver2;
mod thread_pool;
mod mines;

async fn foo() {
    println!("Hello from foo!");
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    let res = client
        .get(Uri::from_static("http://essrv10.richard-wolf.com/kimai/api/"))
        .await.unwrap();

    println!("status: {}", res.status());

    let buf = hyper::body::to_bytes(res).await.unwrap();

    foo().await;

    println!("body: {:?}", buf);
}
