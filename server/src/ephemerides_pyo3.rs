use std::env;

use chrono::NaiveDateTime;
use pyo3::prelude::*;

use crate::models::{GetAstronObjectParams, GetAstronObjectResponse};

pub fn get_planet_ephemerides<'py>(
    py: Python<'py>,
    params: &GetAstronObjectParams,
) -> anyhow::Result<GetAstronObjectResponse> {
    let ephem = PyModule::import(py, "ephem")?;
    let observer = ephem.getattr("Observer")?.call0()?;
    let py_date = params.when.into_pyobject(py)?;
    observer.setattr("pressure", 0)?;
    observer.setattr("epoch", ephem.getattr("J2000")?)?;

    observer.setattr("lon", params.lon.to_string())?;
    observer.setattr("lat", params.lat.to_string())?;
    observer.setattr("elevation", params.elevation)?;
    observer.setattr("date", py_date)?;

    let planet = ephem.getattr(params.name.to_string())?.call0()?;
    planet.getattr("compute")?.call1((observer.clone(),))?;

    let setting_time_str: String = observer
        .getattr("next_setting")?
        .call1((planet.clone(),))?
        .getattr("__str__")?
        .call0()?
        .extract()?;
    let setting_time = NaiveDateTime::parse_from_str(&setting_time_str, "%Y/%m/%d %H:%M:%S")
        .unwrap()
        .and_utc();

    let rising_time_str: String = observer
        .getattr("next_rising")?
        .call1((planet.clone(),))?
        .getattr("__str__")?
        .call0()?
        .extract()?;
    let rising_time = NaiveDateTime::parse_from_str(&rising_time_str, "%Y/%m/%d %H:%M:%S")
        .unwrap()
        .and_utc();

    let resp = GetAstronObjectResponse {
        name: params.name.clone(),
        magnitude: planet.getattr("mag")?.extract()?,
        size: planet.getattr("size")?.extract()?,
        az: planet.getattr("az")?.extract()?,
        el: planet.getattr("alt")?.extract()?,
        ra: planet.getattr("ra")?.extract()?,
        dec: planet.getattr("dec")?.extract()?,
        setting_time,
        rising_time,
        when: params.when.clone(),
    };

    Ok(resp)
}

pub fn init() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    let path_addition = env::var("PATH_ADDITION").unwrap();
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let path = sys.getattr("path")?;
        path.call_method1("append", (path_addition,))?;
        Ok(())
    })
}
