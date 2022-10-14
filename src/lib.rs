use axum::{routing::{get}, Router};
use axum::extract::{Path, Query};
use sync_wrapper::SyncWrapper;
use serde::Deserialize;

mod spell_it;

async fn spell(Path(phrase): Path<String>) -> String {
    spell_it::spell_it(&phrase)
}

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    let router = Router::new().route("/:phrase", get(spell));
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
