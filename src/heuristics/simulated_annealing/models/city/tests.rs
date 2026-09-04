
    use super::super::city::City;
    #[test]
    fn test_trivial_distance() {
        let city_1 = City {
            id: 1,
            country: Some(String::from("Some Country")),
            name:Some(String::from("Some City")),
            population: Some(0),
            lat: 0.0,
            long: 0.0,
        };
        assert_eq!(
            city_1.distance_to_city(&city_1), 0.0
        )
    }

    #[test]
    fn test_two_cities() {
        let city_1 = City {
            id: 1,
            country: Some(String::from("Japan")),
            name:Some(String::from("Tokyo")),
            population: Some(0),
            lat: 35.68500000000000227,
            long: 139.7510000000000047,
        };
        let city_2 = City {
            id: 7,
            country: Some(String::from("Philippines")),
            name:Some(String::from("Manila")),
            population: Some(0),
            lat: 14.60420000000000051,
            long: 120.9819999999999994,
        };

        let result = city_2.distance_to_city(&city_1);
        let expected = 2999396.231968969;
        let abs_difference = (result - expected).abs();
        assert!(abs_difference < 1e-7);
    }