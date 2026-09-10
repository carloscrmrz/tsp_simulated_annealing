use super::Tour;
use crate::heuristics::simulated_annealing::models::{city::City, connection::Connection};

fn city(id: i32, lat: f64, long: f64) -> City {
    City {
        id,
        name: None,
        country: None,
        population: None,
        lat,
        long,
    }
}

fn conn(id_city_1: i32, id_city_2: i32, distance: f64) -> Connection {
    Connection {
        id_city_1,
        id_city_2,
        distance,
    }
}

fn sample_tour(seed: u64) -> Tour {
    let cities = vec![
        city(1, 0.0, 0.0),
        city(2, 10.0, 10.0),
        city(3, 20.0, 5.0),
        city(4, -5.0, 15.0),
        city(5, 30.0, -10.0),
        city(6, 12.0, 25.0),
    ];
    let connections = vec![
        conn(1, 2, 1_000_000.0),
        conn(2, 3, 1_500_000.0),
        conn(3, 4, 900_000.0),
        conn(4, 5, 2_000_000.0),
    ];
    let instance: Vec<usize> = (0..cities.len()).collect();
    Tour::new(&cities, &instance, &connections, Some(seed))
}

#[test]
fn delta_matches_full_recompute() {
    let mut tour = sample_tour(42);

    for _ in 0..10_000 {
        let before = tour.calculate_current_cost();
        let delta = tour.move_solution();
        let after = tour.calculate_current_cost();
        assert!(
            (delta - (after - before)).abs() < 1e-7,
            "delta {delta} vs scratch diff {}",
            after - before
        );
        assert!((tour.current_cost() - after).abs() < 1e-7);
    }
}

#[test]
fn undo_restores_solution_and_cost() {
    let mut tour = sample_tour(7);
    let cost_before = tour.current_cost();
    let solution_before = tour.current_solution.clone();

    tour.move_solution();
    tour.undo_move();

    assert_eq!(tour.current_solution, solution_before);
    assert!((tour.current_cost() - cost_before).abs() < 1e-7);
}

#[test]
fn swap_delta_matches_full_recompute_and_is_side_effect_free() {
    let mut tour = sample_tour(7);
    let n = tour.current_solution.len();

    for p in 0..n {
        for q in (p + 1)..n {
            let before_solution = tour.current_solution.clone();
            let before_cost = tour.current_cost();

            let delta = tour.swap_delta(p, q);

            assert_eq!(tour.current_solution, before_solution);
            assert!((tour.current_cost() - before_cost).abs() < 1e-12);

            let mut swapped = before_solution.clone();
            swapped.swap(p, q);
            let expected = tour.calculate_cost(&swapped) - before_cost;
            assert!(
                (delta - expected).abs() < 1e-9,
                "swap({p},{q}): delta {delta} vs expected {expected}"
            );
        }
    }
}

