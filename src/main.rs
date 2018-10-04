extern crate redis;
// extern crate tokio_threadpool;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

use redis::{Client};
// use tokio_threadpool::ThreadPool;

fn main() {
    pretty_env_logger::init();
    info!("Service started");

    let conn = Client::open("redis://localhos")
        .map(|c| RedisConn { client: c })
        .and_then(|rc| rc.client.get_connection());

    match conn {
        Ok(_) => info!("OK"),
        _ => error!("Cannot connect to redis")
    };
}

struct RedisConn {
    client: Client
}