use async_trait::async_trait;
use pingora_load_balancing::{LoadBalancer, selection::RoundRobin};
use pingora::lb::Backend;
use pingora_proxy::{ProxyHttp, Session};
use pingora::http::{ResponseHeader, RequestHeader};
use pingora_core::Result;
use pingora_core::upstreams::peer::HttpPeer;
use pingora_core::Error;
use std::sync::{Arc, RwLock};




#[derive(Clone)]
pub struct Router {
    pub load_balancer_a: Arc<RwLock<LoadBalancer<RoundRobin>>>,
    pub load_balancer_b: Arc<RwLock<LoadBalancer<RoundRobin>>>,
}

#[async_trait]
impl ProxyHttp for Router {
    
      type CTX = ();

      fn new_ctx(&self) -> () {
            ()
      }

      async fn upstream_peer(
            &self,
            session: &mut Session,
            _ctx: &mut Self::CTX,
        ) -> Result<Box<HttpPeer>> {
            let path = session.req_header().uri.path();
            let upstream = match path {
                p if p.starts_with("/apiA") => {
                    let load_balancer_a = self.load_balancer_a.read().unwrap();
                    load_balancer_a.select(b"", 256).ok_or_else(|| {
                        pingora_core::Error::new(pingora_core::Custom("No available upstreams for apiA"))
                    })?
                }
                p if p.starts_with("/apiB") => {
                    let load_balancer_b = self.load_balancer_b.read().unwrap();
                    load_balancer_b.select(b"", 256).ok_or_else(|| {
                        pingora_core::Error::new(pingora_core::Custom("No available upstreams for apiB"))
                    })?
                }
                _ => {
                    return Err(pingora_core::Error::new(pingora_core::Custom(
                        "No matching route",
                    )));
                }
            };
    
            Ok(Box::new(HttpPeer::new(upstream, false, "127.0.0.1".to_string())))
        }

      
      async fn upstream_request_filter(
            &self,
            _session: &mut Session,
            upstream_request: &mut RequestHeader,
            _ctx: &mut Self::CTX,
      ) -> Result<()> {
            println!("Request headers before modification: {:?}", upstream_request);
            upstream_request.insert_header("Host", "127.0.0.1").unwrap();
            println!("Request headers after modification: {:?}", upstream_request);
            Ok(())
      }

      fn upstream_response_filter(
            &self,
            _session: &mut Session,
            _upstream_response: &mut ResponseHeader,
            _ctx: &mut Self::CTX,
      ) {
            println!("Upstream response processed: {:?}", _upstream_response);
      }


      async fn request_filter(
            &self, 
            _session: &mut Session,
            _ctx: &mut Self::CTX) -> Result<bool> {
            println!("Request received.");
            Ok(false)
        }
    
        fn fail_to_connect(
            &self,
            _session: &mut Session,
            peer: &HttpPeer,
            _ctx: &mut Self::CTX,
            e: Box<Error>,
        ) -> Box<Error> {
            println!("Failed to connect to upstream {:?}: {:?}", peer, e);
            e
        }
    
        async fn fail_to_proxy(
            &self,
            session: &mut Session,
            e: &Error,
            _ctx: &mut Self::CTX,
        ) -> u16 {
            println!("Proxy failed with error: {:?}", e);
            let message = e.to_string();
            let error_body = match message.as_str() {
                "No available upstreams for apiA" => r#"{"error": "No available upstreams for apiA"}"#,
                "No available upstreams for apiB" => r#"{"error": "No available upstreams for apiB"}"#,
                _ => r#"{"error": "Proxy error"}"#,
            };
    
            // Send response with error body
            let _ = session.respond_error(502).await;
            println!("Error message sent to client: {}", message);
            502
        }
        
  
}


impl Router {
      pub fn log_upstream_counts(&self) {
          let lb_a_count = self.load_balancer_a.read().unwrap().backends().get_backend().len();
          let lb_b_count = self.load_balancer_b.read().unwrap().backends().get_backend().len();
  
          println!("Upstream counts:");
          println!("  API A: {}", lb_a_count);
          println!("  API B: {}", lb_b_count);
  
          let lb_a_backends = self
              .load_balancer_a
              .read()
              .unwrap()
              .backends()
              .get_backend();
  
          let lb_b_backends = self
              .load_balancer_b
              .read()
              .unwrap()
              .backends()
              .get_backend();
  
          println!("API A Backends: {:?}", lb_a_backends);
          println!("API B Backends: {:?}", lb_b_backends);
      }
  }
  
// Implement ProxyHttp for Router here as before
