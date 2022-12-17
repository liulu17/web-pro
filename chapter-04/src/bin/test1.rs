use axum::{Json, Router};
use axum::extract::Path;
use axum::routing::{get,post};
// use warp::post;
use serde::Deserialize;
use std::collections::HashMap;


#[tokio::main]
async fn main() {

    async fn params_map(
        Path(params): Path<HashMap<String, String>>,
    ) {
        println!("{:?}",params)
    }

    async fn params_vec(
        Path(params): Path<Vec<(String, String)>>,
    ) {
        println!("{:?}",params)
    }

    let app = Router::new()
        .route("/users/:user_id/team/:team_id", get(params_map).post(params_vec));

    axum::Server::bind(&"0.0.0.0:9100".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}