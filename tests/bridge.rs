// use std::sync::Arc;
// use tokio::prelude::*;
// use reqwest::StatusCode;

use huezinga::bridge::Bridge;
// use huezinga::json::Nupnp;

#[tokio::test]
async fn discover() {
    let mut bridge = Bridge::new();
    let res = bridge.discover().await.expect("Failed to get response");
    println!("IP: {:#?}", res);
    assert!(bridge.ip.is_some())
}
