struct City {
    id: i32,
    lat: f64,
    long: f64,
}
struct Tour(Vec<City>);

const EARTH_RADIUS: i32 = 6373000;

impl Solution for Tour {
    fn distance(&self, s: Self::Solution, first: City, second: City) -> f64 {
        return EARTH_RADIUS * s._calculate_c()
    }

    fn _convert_degrees_to_radians(degrees: f64) -> f64 {
        (degrees * PI) / 180.0
    }

    fn _calculate_c(&self, _s: &Self::Solution, first: City, second: City) -> f64 {
        let haversine: f64 = self._calculate_haversine(first, second);
        let sqrt_haversine: f64 = haversine.sqrt();
        let sqrt_haversine_minus_one: f64 = (haversine - 1).sqrt() ;
        2 * sqrt_haversine.atan2(sqrt_haversine_minus_one)
    }

    fn _calculate_haversine(first: City, second: City) -> f64 {
        delta_long = second.long - first.long;
        delta_lat = first.lat - second.lat;

        (delta_long / 2).sin().powi(2) + (
            first.lat.cos() *
            second.lat.cos() *
            (delta_lat / 2).sin().powi(2)
        )
    }
}