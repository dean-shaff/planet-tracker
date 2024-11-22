use crate::{
    astronomy::EarthCoordinates,
    models::{AppError, GetAstronObjectParams, GetAstronObjectResponse},
};

pub fn get_planet_ephemerides(
    params: &GetAstronObjectParams,
) -> Result<GetAstronObjectResponse, AppError> {
    let coords = EarthCoordinates::new(params.lat, params.lon);
    let az_el = params.name.get_ephemerides(&params.when, &coords)?;
    let setting_time = params.name.get_setting_time(&params.when, &coords)?;
    let rising_time = params.name.get_rising_time(&params.when, &coords)?;
    let apparent_magnitude = params.name.get_apparent_magnitude(&params.when)?;

    // let (az_el, setting_time, rising_time, apparent_magnitude) =
    //     params.name.get_everything(&params.when, &coords)?;
    Ok(GetAstronObjectResponse {
        name: params.name.clone(),
        magnitude: apparent_magnitude,
        size: 0.0,
        az: az_el.az,
        el: az_el.el,
        ra: 0.0,
        dec: 0.0,
        setting_time,
        rising_time,
        when: params.when.clone(),
    })
}
