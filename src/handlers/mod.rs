use axum::Router;
use inventory;
use tower_http::services::ServeDir;

pub mod home;
pub mod about;
pub mod hotreload;

pub struct RouteInfo {
    pub path: &'static str,
    pub method: &'static str,
    pub route_builder: fn() -> (&'static str, axum::routing::MethodRouter),
}

inventory::collect!(RouteInfo);

pub fn build_router() -> Router {
    let mut router = Router::new();
    
    for route_info in inventory::iter::<RouteInfo> {
        let (path, method_router) = (route_info.route_builder)();
        router = router.route(path, method_router);
    }
    
    // Add static file serving
    router = router.nest_service("/static", ServeDir::new("static"));
    
    router
}
