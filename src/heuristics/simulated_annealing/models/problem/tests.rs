use super::TravelSalesmanProblem;
use crate::heuristics::simulated_annealing::models::{city::City, connection::Connection, solution::Tour};

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

fn conn(id_city_1: i32, id_city_2: i32) -> Connection {
    Connection { id_city_1, id_city_2, distance: 0.0 }
}

fn line_cities() -> Vec<City> {
    vec![
        city(1, 0.0, 0.0),
        city(2, 0.0, 1.0),
        city(3, 0.0, 2.0),
        city(4, 0.0, 3.0),
    ]
}

fn complete_connections() -> Vec<Connection> {
    vec![
        conn(1, 2), conn(1, 3), conn(1, 4),
        conn(2, 3), conn(2, 4), conn(3, 4),
    ]
}

fn line_tour(order: Vec<usize>) -> Tour {
    let cities = line_cities();
    let connections = complete_connections();
    let instance: Vec<usize> = (0..cities.len()).collect();
    let mut tour = Tour::new(&cities, &instance, &connections, Some(0));
    tour.current_solution = order;
    tour.resync_cost();
    tour
}

#[test]
fn new_initializes_state_from_the_tour() {
    let tour = line_tour(vec![0, 1, 2, 3]);
    let initial_cost = tour.current_cost();
    let initial_solution = tour.current_solution.clone();
    let seed = tour.get_rng_seed();

    let problem = TravelSalesmanProblem::new(tour, 50.0, 0.98, 1e-9, 10);

    assert_eq!(problem.best_cost(), initial_cost);
    assert_eq!(problem.best_solution().to_vec(), initial_solution);
    assert_eq!(problem.accepted_solutions().to_vec(), vec![initial_solution]);
    assert_eq!(problem.rng_seed(), seed);
    assert_eq!(problem.temperature, 50.0);
    assert_eq!(problem.decay_factor, 0.98);
}

#[test]
fn feasibility_is_true_for_an_optimal_tour_on_a_complete_graph() {
    let tour = line_tour(vec![0, 1, 2, 3]);
    let problem = TravelSalesmanProblem::new(tour, 50.0, 0.98, 1e-9, 10);

    assert!(problem.feasibility());
}

#[test]
fn feasibility_is_false_when_unknown_edges_are_penalized() {
    let cities = line_cities();
    let connections = vec![conn(1, 2), conn(2, 3), conn(3, 4)];
    let instance: Vec<usize> = (0..cities.len()).collect();
    let mut tour = Tour::new(&cities, &instance, &connections, Some(0));
    tour.current_solution = vec![0, 3, 1, 2];
    tour.resync_cost();

    let problem = TravelSalesmanProblem::new(tour, 50.0, 0.98, 1e-9, 10);

    assert!(!problem.feasibility());
}

#[test]
fn regulate_temp_multiplies_temperature_by_decay_factor() {
    let tour = line_tour(vec![0, 1, 2, 3]);
    let mut problem = TravelSalesmanProblem::new(tour, 100.0, 0.9, 1e-9, 10);

    problem.regulate_temp();
    assert!((problem.temperature - 90.0).abs() < 1e-9);

    problem.regulate_temp();
    assert!((problem.temperature - 81.0).abs() < 1e-9);
}

#[test]
fn calculate_batch_rejects_every_move_when_temperature_is_too_low() {
    let tour = line_tour(vec![0, 1, 2, 3]);
    let mut problem = TravelSalesmanProblem::new(tour, f64::NEG_INFINITY, 0.98, 1e-9, 5);

    let (average, accepted) = problem.calculate_batch(5);

    assert_eq!(accepted, 0);
    assert_eq!(average, 0.0);
}

#[test]
fn calculate_batch_accepts_every_move_when_temperature_is_very_high() {
    let tour = line_tour(vec![0, 1, 2, 3]);
    let mut problem = TravelSalesmanProblem::new(tour, f64::INFINITY, 0.98, 1e-9, 5);

    let (average, accepted) = problem.calculate_batch(5);

    assert_eq!(accepted, 5);
    assert!(average >= 0.0);
}

#[test]
fn do_downhill_sweep_improves_an_unsorted_tour() {
    let tour = line_tour(vec![0, 3, 1, 2]);
    let cost_before = tour.current_cost();
    let mut problem = TravelSalesmanProblem::new(tour, 1.0, 0.98, 1e-9, 10);

    let improved = problem.do_downhill_sweep();

    assert!(improved);
    assert!(problem.best_cost() < cost_before);
    assert_eq!(problem.accepted_solutions().len(), 2);
}

#[test]
fn do_downhill_sweep_reports_no_move_at_a_local_optimum() {
    let tour = line_tour(vec![0, 1, 2, 3]);
    let mut problem = TravelSalesmanProblem::new(tour, 1.0, 0.98, 1e-9, 10);

    let improved = problem.do_downhill_sweep();

    assert!(!improved);
    assert_eq!(problem.accepted_solutions().len(), 1);
}

#[test]
fn descend_stops_exactly_when_no_improving_swap_remains() {
    let tour = line_tour(vec![0, 3, 1, 2]);
    let initial_cost = tour.current_cost();
    let mut problem = TravelSalesmanProblem::new(tour, 1.0, 0.98, 1e-9, 10);

    let moves = problem.descend();

    assert!(moves >= 1);
    assert!(problem.best_cost() <= initial_cost);
    assert!(problem.tour.best_swap().is_none());
}
