use actix_web::web::{get, post, scope, ServiceConfig};

pub mod create;
pub mod get;
mod edit;

pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/item")
            .route("create/{title}", post().to(create::create))
            .route("get", get().to(get::get))
            .route("edit", post().to(edit::edit))
    );
}
