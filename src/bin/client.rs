use lazy_static::lazy_static;
use myredis::FilterLayer;
use myredis::LogLayer;
use std::net::SocketAddr;
use std::sync::Arc;
use volo::FastStr;
use volo_gen::myredis::{RedisRequest, RequestType};

lazy_static! {
    static ref CLIENT: volo_gen::myredis::RedisServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::myredis::RedisServiceClientBuilder::new("redis")
            .layer_outer(LogLayer)
            .layer_outer(FilterLayer)
            .address(addr)
            .build()
    };
}
#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut args: Vec<String> = std::env::args().collect();

    let req = match args[1].to_lowercase().as_str() {
        "set" => RedisRequest {
            key: Some(FastStr::from(Arc::new(args.remove(2)))),
            value: Some(FastStr::from(Arc::new(args.remove(2)))),
            request_type: RequestType::Set,
            expire_time: None,
        },
        "get" => RedisRequest {
            key: Some(FastStr::from(Arc::new(args.remove(2)))),
            value: None,
            request_type: RequestType::Get,
            expire_time: None,
        },
        "del" => RedisRequest {
            key: Some(FastStr::from(Arc::new(args.remove(2)))),
            value: None,
            request_type: RequestType::Del,
            expire_time: None,
        },
        "ping" => RedisRequest {
            key: None,
            value: None,
            request_type: RequestType::Ping,
            expire_time: None,
        },
        "subscribe" => RedisRequest {
            key: Some(FastStr::from(Arc::new(args.remove(2)))),
            value: None,
            request_type: RequestType::Subscribe,
            expire_time: None,
        },
        "publish" => RedisRequest {
            key: Some(FastStr::from(Arc::new(args.remove(2)))),
            value: None,
            request_type: RequestType::Publish,
            expire_time: None,
        },
        _ => {
            panic!("unknown command");
        }
    };
    let resp = CLIENT.redis_command(req).await;
    match resp {
        Ok(info) => println!("{}", info.value.unwrap()),
        Err(e) => match e {
            volo_thrift::ResponseError::Application(err) => {
                println!("{}", err.message)
            }
            _ => {
                tracing::error!("{:?}", e);
            }
        },
    }
}
