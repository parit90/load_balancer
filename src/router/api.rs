use pingora_load_balancing::LoadBalancer;
use pingora::lb::Backend;
use pingora_core::Result;
use std::sync::{Arc, RwLock};
use async_trait::async_trait;
use crate::router::router::Router;




impl Router {
      pub fn add_upstream(&self, service_name: &str, server_address: &str) -> Result<()> {
          let addr = server_address
              .parse::<std::net::SocketAddr>()
              .map_err(|_| pingora_core::Error::new(pingora_core::Custom("Invalid address")))?;
  
          let pingora_addr = pingora_core::protocols::l4::socket::SocketAddr::Inet(addr);
  
          match service_name {
              "apiA" => {
                  let mut lb = self.load_balancer_a.write().unwrap();
                  let current_backends = lb.backends().get_backend();
                  let mut updated_backends = current_backends.as_ref().clone();
                  updated_backends.insert(Backend::new(server_address)?);
                  *lb = LoadBalancer::try_from_iter(updated_backends.into_iter()).unwrap();
              }
              "apiB" => {
                  let mut lb = self.load_balancer_b.write().unwrap();
                  let current_backends = lb.backends().get_backend();
                  let mut updated_backends = current_backends.as_ref().clone();
                  updated_backends.insert(Backend::new(server_address)?);
                  *lb = LoadBalancer::try_from_iter(updated_backends.into_iter()).unwrap();
              }
              _ => return Err(pingora_core::Error::new(pingora_core::Custom("Invalid service name"))),
          }
  
          Ok(())
      }
  
      pub fn remove_upstream(&self, service_name: &str, server_address: &str) -> Result<()> {
          let addr = server_address
              .parse::<std::net::SocketAddr>()
              .map_err(|_| pingora_core::Error::new(pingora_core::Custom("Invalid address")))?;
  
          let pingora_addr = pingora_core::protocols::l4::socket::SocketAddr::Inet(addr);
  
          match service_name {
              "apiA" => {
                  let mut lb = self.load_balancer_a.write().unwrap();
                  let current_backends = lb.backends().get_backend();
                  let mut updated_backends = current_backends.as_ref().clone();
                  updated_backends.retain(|backend| backend.addr != pingora_addr);
                  *lb = LoadBalancer::try_from_iter(updated_backends.into_iter()).unwrap();
              }
              "apiB" => {
                  let mut lb = self.load_balancer_b.write().unwrap();
                  let current_backends = lb.backends().get_backend();
                  let mut updated_backends = current_backends.as_ref().clone();
                  updated_backends.retain(|backend| backend.addr != pingora_addr);
                  *lb = LoadBalancer::try_from_iter(updated_backends.into_iter()).unwrap();
              }
              _ => return Err(pingora_core::Error::new(pingora_core::Custom("Invalid service name"))),
          }
  
          Ok(())
      }
  }