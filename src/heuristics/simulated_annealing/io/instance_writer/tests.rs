use super::InstanceWriter;
use crate::heuristics::simulated_annealing::models::{
    city::City, connection::Connection, problem::TravelSalesmanProblem, solution::Tour,
};
use chrono::{Local, TimeZone, Timelike};

fn city(id: i32, lat: f64, long: f64) -> City {
    City { id, name: None, country: None, population: None, lat, long }
}

fn conn(id_city_1: i32, id_city_2: i32) -> Connection {
    Connection { id_city_1, id_city_2, distance: 0.0 }
}

fn sample_problem(order: Vec<usize>) -> (Vec<City>, Vec<usize>, TravelSalesmanProblem) {
    let cities = vec![
        city(1, 0.0, 0.0),
        city(2, 0.0, 1.0),
        city(3, 0.0, 2.0),
        city(4, 0.0, 3.0),
    ];
    let connections = vec![
        conn(1, 2), conn(1, 3), conn(1, 4),
        conn(2, 3), conn(2, 4), conn(3, 4),
    ];
    let instance: Vec<usize> = (0..cities.len()).collect();
    let mut tour = Tour::new(&cities, &instance, &connections, Some(0));
    tour.current_solution = order;
    tour.resync_cost();
    let problem = TravelSalesmanProblem::new(tour, 50.0, 0.98, 1e-9, 10);
    (cities, instance, problem)
}

#[test]
fn format_instance_joins_city_ids_in_instance_order() {
    let (cities, instance, problem) = sample_problem(vec![0, 1, 2, 3]);
    let writer = InstanceWriter::new(&cities, &instance, None, &problem);

    assert_eq!(writer.format_instance().unwrap(), "1,2,3,4");
}

#[test]
fn format_instance_errors_on_out_of_range_city_index() {
    let (cities, _instance, problem) = sample_problem(vec![0, 1, 2, 3]);
    let bad_instance = vec![0usize, 99];
    let writer = InstanceWriter::new(&cities, &bad_instance, None, &problem);

    assert!(writer.format_instance().is_err());
}

#[test]
fn format_path_maps_solution_positions_to_city_ids() {
    let (cities, instance, problem) = sample_problem(vec![0, 1, 2, 3]);
    let writer = InstanceWriter::new(&cities, &instance, None, &problem);

    assert_eq!(writer.format_path(&[3usize, 0, 2, 1]).unwrap(), "4,1,3,2");
}

#[test]
fn format_path_errors_on_out_of_range_position() {
    let (cities, instance, problem) = sample_problem(vec![0, 1, 2, 3]);
    let writer = InstanceWriter::new(&cities, &instance, None, &problem);

    assert!(writer.format_path(&[7usize]).is_err());
}

#[test]
fn format_solutions_reports_the_normalized_cost_of_each_accepted_solution() {
    let (cities, instance, problem) = sample_problem(vec![0, 1, 2, 3]);
    let writer = InstanceWriter::new(&cities, &instance, None, &problem);

    let expected_cost = problem.tour.calculate_cost(&problem.accepted_solutions()[0]);
    assert_eq!(writer.format_solutions(), format!("E:{expected_cost:.15}\n"));
}

#[test]
fn format_report_includes_paths_seed_and_feasibility() {
    let (cities, instance, mut problem) = sample_problem(vec![0, 3, 1, 2]);
    problem.descend();
    let writer = InstanceWriter::new(&cities, &instance, None, &problem);

    let report = writer.format_report(&problem).unwrap();

    assert!(report.contains(&format!("Initial Path: {}", writer.format_instance().unwrap())));
    assert!(report.contains(&format!(
        "Solution: {}",
        writer.format_path(problem.best_solution()).unwrap()
    )));
    assert!(report.contains(&format!("Seed: {}", problem.rng_seed())));
    assert!(report.contains(if problem.feasibility() { "Feasible: YES" } else { "Feasible: NO" }));
}

#[test]
fn output_file_name_embeds_the_padded_index_and_the_formatted_timestamp() {
    let at = Local.with_ymd_and_hms(2024, 3, 7, 9, 5, 2).unwrap();
    let name = InstanceWriter::output_file_name(3, &at);

    assert!(name.starts_with("./results/output-0003-"));
    assert!(name.ends_with(".out"));
    assert!(name.contains(&at.format("%d%m-%Y-%H%M-%S").to_string()));
}

#[test]
fn output_file_name_includes_centiseconds_from_subsecond_millis() {
    let at = Local
        .with_ymd_and_hms(2024, 1, 1, 0, 0, 0)
        .unwrap()
        .with_nanosecond(456_000_000)
        .unwrap();

    let name = InstanceWriter::output_file_name(0, &at);

    assert!(name.ends_with("45.out"));
}

#[test]
fn solutions_file_name_replaces_extension_with_sol() {
    assert_eq!(
        InstanceWriter::solutions_file_name("./results/output-0001.out"),
        "./results/output-0001.sol"
    );
}

#[test]
fn solutions_file_name_appends_extension_when_missing() {
    assert_eq!(InstanceWriter::solutions_file_name("output"), "output.sol");
}
