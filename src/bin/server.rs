#![feature(impl_trait_in_assoc_type)]

use myredis::FilterLayer;
use myredis::LogLayer;
use myredis::S;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Mutex;

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);
    volo_gen::myredis::RedisServiceServer::new(S {
        map: Mutex::new(HashMap::<String, String>::new()),
    })
    .layer_front(LogLayer)
    .layer_front(FilterLayer)
    .run(addr)
    .await
    .unwrap();
}
