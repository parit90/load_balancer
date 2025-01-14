use warp::Filter;
use std::sync::Arc;
use crate::router::router::Router;
use crate::router::api::*;
use warp::reject::Reject;


#[derive(Debug)]
pub struct CustomReject(Box<pingora::Error>);
impl Reject for CustomReject {}

pub async fn start_dynamic_api(router: Arc<Router>) {
      let add_upstream = warp::path!("add_upstream" / String / String)
          .and(warp::any().map({
              let router = router.clone();
              move || router.clone()
          }))
          .and_then(|service_name: String, server_address: String, router: Arc<Router>| async move {
              match router.add_upstream(&service_name, &server_address) {
                  Ok(_) => Ok::<_, warp::Rejection>(warp::reply::json(&"Upstream added")),
                  Err(e) => Err(warp::reject::custom(CustomReject(e))),
              }
          });
  
      let remove_upstream = warp::path!("remove_upstream" / String / String)
          .and(warp::any().map({
              let router = router.clone();
              move || router.clone()
          }))
          .and_then(|service_name: String, server_address: String, router: Arc<Router>| async move {
              match router.remove_upstream(&service_name, &server_address) {
                  Ok(_) => Ok::<_, warp::Rejection>(warp::reply::json(&"Upstream removed")),
                  Err(e) => Err(warp::reject::custom(CustomReject(e))),
              }
          });
  
      let routes = add_upstream.or(remove_upstream);
      warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
  
