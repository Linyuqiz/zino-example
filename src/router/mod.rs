use ntex::web::{get, post, ServiceConfig};
use zino::{DefaultController, RouterConfigure};
use zino_model::Tag;

pub fn routes() -> Vec<RouterConfigure> {
    vec![tag_router as RouterConfigure]
}

fn tag_router(cfg: &mut ServiceConfig) {
    cfg.route("/tag/new", post().to(Tag::new))
        .route("/tag/{id}/delete", post().to(Tag::soft_delete))
        .route("/tag/{id}/update", post().to(Tag::update))
        .route("/tag/{id}/view", get().to(Tag::view))
        .route("/tag/list", get().to(Tag::list))
        .route("/tag/tree", get().to(Tag::tree));
}
