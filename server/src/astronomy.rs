#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{fmt, num::TryFromIntError};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[derive(Error, Debug)]
pub enum AstronomyError {
    #[error("Integer overflow")]
    Overflow(#[from] TryFromIntError),
    #[error("Calculation for {0} failed with code {1}")]
    Calculation(String, u32),
    #[error("Conversion")]
    Conversion,
}

pub struct EarthCoordinates {
    pub lat: f64,
    pub lon: f64,
    pub altitude: f64,
}

impl EarthCoordinates {
    pub fn new(lat: f64, lon: f64) -> Self {
        Self {
            lon,
            lat,
            altitude: 0.0,
        }
    }
}

impl From<&EarthCoordinates> for astro_observer_t {
    fn from(value: &EarthCoordinates) -> Self {
        astro_observer_t {
            latitude: value.lat,
            longitude: value.lon,
            height: value.altitude,
        }
    }
}

#[derive(Debug)]
pub struct AzEl {
    pub az: f64,
    pub el: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Planet {
    Mercury,
    Venus,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

impl fmt::Display for Planet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Mercury => write!(f, "Mercury"),
            Self::Venus => write!(f, "Venus"),
            Self::Mars => write!(f, "Mars"),
            Self::Jupiter => write!(f, "Jupiter"),
            Self::Saturn => write!(f, "Saturn"),
            Self::Uranus => write!(f, "Uranus"),
            Self::Neptune => write!(f, "Neptune"),
        }
    }
}

impl From<&Planet> for astro_body_t {
    fn from(value: &Planet) -> Self {
        match value {
            Planet::Mercury => astro_body_t_BODY_MERCURY,
            Planet::Venus => astro_body_t_BODY_VENUS,
            Planet::Mars => astro_body_t_BODY_MARS,
            Planet::Jupiter => astro_body_t_BODY_JUPITER,
            Planet::Saturn => astro_body_t_BODY_SATURN,
            Planet::Uranus => astro_body_t_BODY_URANUS,
            Planet::Neptune => astro_body_t_BODY_NEPTUNE,
        }
    }
}

impl TryFrom<&DateTime<Utc>> for astro_time_t {
    type Error = TryFromIntError;
    fn try_from(value: &DateTime<Utc>) -> Result<Self, Self::Error> {
        let time = unsafe {
            Astronomy_MakeTime(
                value.year(),
                i32::try_from(value.month())?,
                i32::try_from(value.day())?,
                i32::try_from(value.hour())?,
                i32::try_from(value.minute())?,
                f64::from(value.second()),
            )
        };
        Ok(time)
    }
}

fn try_convert_utc(utc: &astro_utc_t) -> Result<DateTime<Utc>, AstronomyError> {
    let second = utc.second.round() as u32;
    let hour = u32::try_from(utc.hour)?;
    let min = u32::try_from(utc.minute)?;
    let date_time =
        NaiveDate::from_ymd_opt(utc.year, u32::try_from(utc.month)?, u32::try_from(utc.day)?)
            .and_then(|d| NaiveTime::from_hms_opt(hour, min, second).map(|t| (d, t)))
            .map(|(d, t)| NaiveDateTime::new(d, t))
            .ok_or(AstronomyError::Conversion)?
            .and_utc();
    Ok(date_time)
}

impl Planet {
    pub fn get_everything(
        &self,
        date_time: &DateTime<Utc>,
        coordinates: &EarthCoordinates,
    ) -> Result<(AzEl, DateTime<Utc>, DateTime<Utc>, f64), AstronomyError> {
        let observer = astro_observer_t::from(coordinates);
        unsafe {
            let mut time = astro_time_t::try_from(date_time)?;
            let equ_ofdate = Astronomy_Equator(
                astro_body_t::from(self),
                &mut time,
                observer,
                astro_equator_date_t_EQUATOR_OF_DATE,
                astro_aberration_t_ABERRATION,
            );
            if equ_ofdate.status != astro_status_t_ASTRO_SUCCESS {
                return Err(AstronomyError::Calculation(
                    "Astronomy_Equator".to_string(),
                    equ_ofdate.status,
                ));
            }
            let hor = Astronomy_Horizon(
                &mut time,
                observer,
                equ_ofdate.ra,
                equ_ofdate.dec,
                astro_refraction_t_REFRACTION_NORMAL,
            );

            let setting_event = Astronomy_SearchRiseSetEx(
                astro_body_t::from(self),
                observer,
                astro_direction_t_DIRECTION_SET,
                time,
                300.0,
                coordinates.altitude,
            );
            let setting_time_utc = Astronomy_UtcFromTime(setting_event.time);
            let setting_time = try_convert_utc(&setting_time_utc)?;
            let rising_event = Astronomy_SearchRiseSetEx(
                astro_body_t::from(self),
                observer,
                astro_direction_t_DIRECTION_SET,
                time,
                300.0,
                coordinates.altitude,
            );
            let rising_time_utc = Astronomy_UtcFromTime(rising_event.time);
            let rising_time = try_convert_utc(&rising_time_utc)?;
            let illum = Astronomy_Illumination(astro_body_t::from(self), time);

            Ok((
                AzEl {
                    az: hor.azimuth,
                    el: hor.altitude,
                },
                setting_time,
                rising_time,
                illum.mag,
            ))
        }
    }

    pub fn get_apparent_magnitude(&self, date_time: &DateTime<Utc>) -> Result<f64, AstronomyError> {
        let mag = unsafe {
            let time = astro_time_t::try_from(date_time)?;
            let illum = Astronomy_Illumination(astro_body_t::from(self), time);
            illum.mag
        };
        Ok(mag)
    }

    pub fn get_setting_time(
        &self,
        date_time: &DateTime<Utc>,
        coordinates: &EarthCoordinates,
    ) -> Result<DateTime<Utc>, AstronomyError> {
        let observer = astro_observer_t::from(coordinates);
        unsafe {
            let time = astro_time_t::try_from(date_time)?;
            let setting_event = Astronomy_SearchRiseSetEx(
                astro_body_t::from(self),
                observer,
                astro_direction_t_DIRECTION_SET,
                time,
                300.0,
                coordinates.altitude,
            );
            let setting_time_utc = Astronomy_UtcFromTime(setting_event.time);
            try_convert_utc(&setting_time_utc)
        }
    }

    pub fn get_rising_time(
        &self,
        date_time: &DateTime<Utc>,
        coordinates: &EarthCoordinates,
    ) -> Result<DateTime<Utc>, AstronomyError> {
        let observer = astro_observer_t::from(coordinates);
        unsafe {
            let time = astro_time_t::try_from(date_time)?;
            let setting_event = Astronomy_SearchRiseSetEx(
                astro_body_t::from(self),
                observer,
                astro_direction_t_DIRECTION_RISE,
                time,
                300.0,
                coordinates.altitude,
            );
            let setting_time_utc = Astronomy_UtcFromTime(setting_event.time);
            try_convert_utc(&setting_time_utc)
        }
    }

    pub fn get_ephemerides(
        &self,
        date_time: &DateTime<Utc>,
        coordinates: &EarthCoordinates,
    ) -> Result<AzEl, AstronomyError> {
        let observer = astro_observer_t::from(coordinates);
        let az_el = unsafe {
            let mut time = astro_time_t::try_from(date_time)?;
            let equ_ofdate = Astronomy_Equator(
                astro_body_t::from(self),
                &mut time,
                observer,
                astro_equator_date_t_EQUATOR_OF_DATE,
                astro_aberration_t_ABERRATION,
            );
            if equ_ofdate.status != astro_status_t_ASTRO_SUCCESS {
                return Err(AstronomyError::Calculation(
                    "Astronomy_Equator".to_string(),
                    equ_ofdate.status,
                ));
            }
            let hor = Astronomy_Horizon(
                &mut time,
                observer,
                equ_ofdate.ra,
                equ_ofdate.dec,
                astro_refraction_t_REFRACTION_NORMAL,
            );
            AzEl {
                az: hor.azimuth,
                el: hor.altitude,
            }
        };
        Ok(az_el)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_ephemerides() {
        let planet = Planet::Jupiter;
        let now = Utc::now();
        let coords = EarthCoordinates::new(52.49, 13.41);
        let res = planet.get_ephemerides(&now, &coords);
        let ephemerides = res.unwrap();
        println!("ephemerides = {:?}", ephemerides);
    }

    #[test]
    fn test_get_setting_time() {
        let planet = Planet::Jupiter;
        let now = Utc::now();
        let coords = EarthCoordinates::new(52.49, 13.41);
        let res = planet.get_setting_time(&now, &coords);
        let setting_time = res.unwrap();
        println!("setting_time = {:?}", setting_time);
    }

    #[test]
    fn test_get_rising_time() {
        let planet = Planet::Jupiter;
        let now = Utc::now();
        let coords = EarthCoordinates::new(52.49, 13.41);
        let res = planet.get_rising_time(&now, &coords);
        let rising_time = res.unwrap();
        println!("rising_time = {:?}", rising_time);
    }

    #[test]
    fn test_get_apparent_magnitude() {
        let planet = Planet::Jupiter;
        let now = Utc::now();
        let res = planet.get_apparent_magnitude(&now);
        let apparent_magnitude = res.unwrap();
        println!("apparent_magnitude = {:?}", apparent_magnitude);
    }
}
