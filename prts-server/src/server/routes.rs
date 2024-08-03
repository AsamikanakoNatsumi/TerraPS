use axum::{
    routing::{get, post},
    Json, Router,
};

use tower_http::trace::{DefaultMakeSpan, DefaultOnEos, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::models::PlayerDataDeltaStatic;

use super::core::{asset, prod_cfg};

pub fn app() -> Router {
    let trace = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::default().level(Level::INFO))
        .on_request(DefaultOnRequest::default().level(Level::INFO))
        .on_eos(DefaultOnEos::default().level(Level::INFO))
        .on_failure(DefaultOnFailure::default().level(Level::ERROR))
        .on_response(DefaultOnResponse::default().level(Level::DEBUG));

    Router::new().nest("/config/prod", prod_config_routes()).merge(misc_routes()).layer(trace).fallback(fallback)
}

fn prod_config_routes() -> Router {
    Router::new()
        .route("/official/Android/version", get(prod_cfg::prod_version))
        .route("/official/network_config", get(prod_cfg::prod_network))
        .route("/official/remote_config", get(prod_cfg::prod_remote))
        .route("/official/refresh_config", get(prod_cfg::prod_refresh))
        .route("/announce_meta/Android/announcement.meta.jsons", get(prod_cfg::prod_announce))
        .route("/announce_meta/Android/preannouncement.meta.json", get(prod_cfg::prod_preannounce))
}

fn misc_routes() -> Router {
    Router::new().route("/assetbundle/official/Android/assets/:hash/:name", get(asset::get_file))
}

async fn fallback() -> Json<PlayerDataDeltaStatic> {
    Json(PlayerDataDeltaStatic::default())
}
