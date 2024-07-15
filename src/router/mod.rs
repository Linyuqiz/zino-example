use axum::{
    routing::{get, post},
    Router,
};
use zino::DefaultController;
use zino_example::model::user::User;

pub fn routes() -> Vec<Router> {
    let mut routes = Vec::new();

    let router = Router::new()
        .route("/user/list", get(User::list))
        .route("/user/get", get(User::view))
        .route("/user/create", post(User::new))
        .route("/user/update", post(User::update))
        .route("/user/delete", post(User::soft_delete));
    routes.push(router);

    routes
}
