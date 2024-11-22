use axum::{extract::Query, routing::get, Json, Router};
use ephemerides_pyo3::{get_planet_ephemerides, init};
use models::{AppError, GetAstronObjectParams, GetAstronObjectResponse};
use pyo3::prelude::*;
mod astronomy;
mod ephemerides_pyo3;
mod models;

async fn get_astron_object_data(
    Query(params): Query<GetAstronObjectParams>,
) -> Result<Json<GetAstronObjectResponse>, AppError> {
    let res: anyhow::Result<GetAstronObjectResponse> = Python::with_gil(|py| {
        let res = get_planet_ephemerides(py, &params)?;
        Ok(res)
    });
    let resp = res?;
    Ok(Json(resp))
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    init().unwrap();
    let app = Router::new().route("/get_astron_object_data", get(get_astron_object_data));
    let addr = "0.0.0.0:8081";
    println!("Binding to {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use astronomy::Planet;
    use chrono::Utc;

    use super::*;

    #[test]
    fn test_get_planet_ephemerides() {
        pyo3::prepare_freethreaded_python();
        let res: anyhow::Result<GetAstronObjectResponse> = Python::with_gil(|py| {
            let sys = py.import("sys")?;
            let path = sys.getattr("path")?;
            path.call_method1(
                "append",
                ("/Users/dean/personal/planet-tracker/.venv/lib/python3.12/site-packages",),
            )?;
            let path_vec: Vec<String> = path.extract()?;
            println!("path_vec = {:?}", path_vec);
            let params = GetAstronObjectParams {
                name: Planet::Jupiter,
                lon: 13.41,
                lat: 52.49,
                elevation: 0.0,
                when: Utc::now(),
            };
            let res = get_planet_ephemerides(py, &params)?;
            Ok(res)
        });
        println!("res = {:?}", res.unwrap());
    }
}
