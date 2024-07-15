use ntex::web::{get, post, ServiceConfig};
use zino::{DefaultController, RouterConfigure};
use zino_example::model::user::User;

pub fn routes() -> Vec<RouterConfigure> {
    vec![user_router as RouterConfigure]
}

fn user_router(cfg: &mut ServiceConfig) {
    cfg.route("/user/list", get().to(User::list))
        .route("/user/detail", get().to(User::view))
        .route("/user/create", post().to(User::new))
        .route("/user/update", post().to(User::update))
        .route("/user/delete", post().to(User::delete));
}
