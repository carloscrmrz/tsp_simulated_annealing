use rusqlite::{Error, Row};

use super::super::db::FromRow;

const EARTH_RADIUS: f64 = 6373000.0_f64;

#[derive(Debug, Clone, PartialEq)]
pub struct City {
    pub id: i32,
    pub name: Option<String>,
    pub country: Option<String>,
    pub population: Option<i32>,
    pub lat: f64,
    pub long: f64,
}

fn calculate_haversine(first: &City, second: &City) -> f64 {
    let (first_long_rad, first_lat_rad) = (first.long.to_radians(), first.lat.to_radians());
    let (second_long_rad, second_lat_rad) = (second.long.to_radians(), second.lat.to_radians());
    let delta_long = second_long_rad - first_long_rad;
    let delta_lat = second_lat_rad - first_lat_rad;

    (delta_lat / 2.0).sin().powi(2) +
        first_lat_rad.cos() * second_lat_rad.cos() * (delta_long / 2.0).sin().powi(2)
}

impl City {
    pub fn distance_to_city(&self, to: &City) -> f64 {
        if self == to { return 0.0; }
        return EARTH_RADIUS * self._calculate_central_angle(&to)
    }

    fn _calculate_central_angle(&self, to : &City) -> f64 {
        let haversine: f64 = calculate_haversine(self, &to);
        let sqrt_haversine: f64 = haversine.sqrt();
        let sqrt_one_minus_haversine: f64 = (1.0 - haversine).sqrt();

        2.0 * sqrt_haversine.atan2(sqrt_one_minus_haversine)
    }
}

impl FromRow for City {
    const QUERY: &'static str =
        "SELECT id, name, country, population, latitude, longitude FROM cities ORDER BY id";

    fn from_row(row: &Row) -> Result<City, Error> {
        Ok(City {
            id: row.get("id")?,
            name: row.get("name")?,
            country: row.get("country")?,
            population: row.get("population")?,
            lat: row.get("latitude")?,
            long: row.get("longitude")?,
        })
    }
}
