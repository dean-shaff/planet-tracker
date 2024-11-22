use std::time::Instant;

use astronomy::Planet;
use chrono::Utc;
use models::GetAstronObjectParams;
use pyo3::Python;

mod astronomy;
mod ephemerides;
mod ephemerides_pyo3;
mod models;

fn main() {
    use Planet::*;
    let planets = vec![Mercury, Venus, Mars, Jupiter, Saturn, Uranus, Neptune];
    let params: Vec<GetAstronObjectParams> = planets
        .into_iter()
        .map(|p| GetAstronObjectParams {
            name: p,
            lon: 13.41,
            lat: 52.49,
            elevation: 0.0,
            when: Utc::now(),
        })
        .collect();
    let n = 1000;
    let d0 = Instant::now();
    for _ in 0..n {
        for param in &params {
            ephemerides::get_planet_ephemerides(param).unwrap();
        }
    }
    let delta0 = Instant::now() - d0;

    println!(
        "(astronomy bindings): Took {:?} to do {} iterations, or {:?} per loop",
        delta0,
        n,
        delta0 / n
    );

    ephemerides_pyo3::init().unwrap();
    let d1 = Instant::now();
    Python::with_gil(|py| {
        for _ in 0..n {
            for param in &params {
                ephemerides_pyo3::get_planet_ephemerides(py, param).unwrap();
            }
        }
    });

    let delta1 = Instant::now() - d1;

    println!(
        "(PyO3/ephem): Took {:?} to do {} iterations, or {:?} per loop",
        delta1,
        n,
        delta1 / n
    );

    if delta1 > delta0 {
        println!(
            "astronomy bindings {:?} times faster than PyO3/ephem",
            delta1.as_secs_f64() / delta0.as_secs_f64()
        );
    } else {
        println!(
            "PyO3/ephem {:?} times faster than astronomy bindings",
            delta0.as_secs_f64() / delta1.as_secs_f64()
        );
    }
}
