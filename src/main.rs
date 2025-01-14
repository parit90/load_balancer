mod router;
mod handlers;
mod utils;

use router::router::Router;
use handlers::dynamic_api::start_dynamic_api;
use pingora::server::Server;
use pingora_proxy::http_proxy_service;
use pingora_load_balancing::{LoadBalancer, selection::RoundRobin};
use std::sync::{Arc, RwLock};

fn main() {
    let mut server = Server::new(None).unwrap();
    server.bootstrap();

    let upstream_a = LoadBalancer::try_from_iter(["127.0.0.1:3001"]).unwrap();
    let upstream_b = LoadBalancer::try_from_iter(["127.0.0.1:3002"]).unwrap();

    let router = Router {
        load_balancer_a: Arc::new(RwLock::new(upstream_a)),
        load_balancer_b: Arc::new(RwLock::new(upstream_b)),
    };

    // Log the upstream counts and addresses at startup
    router.log_upstream_counts();

    let router_clone = Arc::new(router.clone());
    std::thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                start_dynamic_api(router_clone).await;
            });
    });

    let mut service = http_proxy_service(&server.configuration, router.clone());
    service.add_tcp("0.0.0.0:6188");
    server.add_service(service);

    server.run_forever();
}
