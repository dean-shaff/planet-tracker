use axum::{extract::Query, routing::get, Json, Router};

mod astronomy;
mod ephemerides;
mod models;
use ephemerides::get_planet_ephemerides;
use models::{AppError, GetAstronObjectParams, GetAstronObjectResponse};

async fn get_astron_object_data(
    Query(params): Query<GetAstronObjectParams>,
) -> Result<Json<GetAstronObjectResponse>, AppError> {
    let resp = get_planet_ephemerides(&params)?;
    Ok(Json(resp))
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = Router::new().route("/get_astron_object_data", get(get_astron_object_data));
    let addr = "0.0.0.0:8081";
    println!("Binding to {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
