use crate::{
    db::model::oauth::application, error::Result, http::extractor::FormOrJson, state::Zustand,
    util::generate_secret,
};
use axum::{extract::State, routing, Json, Router};
use chrono::Utc;
use kitsune_type::mastodon::App;
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AppForm {
    client_name: String,
    redirect_uris: String,
}

async fn post(
    State(db_conn): State<DatabaseConnection>,
    FormOrJson(form): FormOrJson<AppForm>,
) -> Result<Json<App>> {
    let application = application::Model {
        id: Uuid::now_v7(),
        name: form.client_name,
        secret: generate_secret(),
        redirect_uri: form.redirect_uris,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
    .into_active_model()
    .insert(&db_conn)
    .await?;

    Ok(Json(App {
        id: application.id,
        name: application.name,
        redirect_uri: application.redirect_uri,
        client_id: application.id,
        client_secret: application.secret,
    }))
}

pub fn routes() -> Router<Zustand> {
    Router::new().route("/", routing::post(post))
}
