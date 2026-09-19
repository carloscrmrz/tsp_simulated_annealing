use super::InstanceReader;
use crate::heuristics::simulated_annealing::io::args::Args;
use crate::heuristics::simulated_annealing::models::city::City;
use std::fs;
use std::path::PathBuf;

fn city(id: i32) -> City {
    City { id, name: None, country: None, population: None, lat: 0.0, long: 0.0 }
}

fn args_with_raw_instance(raw: &str) -> Args {
    Args {
        instance: Some(raw.to_string()),
        instance_path: None,
        temperature: 50000.0,
        decay_factor: 0.98,
        seed: None,
        lot_size: 5000,
        epsilon: 1e-9,
        max_lots: 10000,
        db_path: None,
        threads: None,
        concurrency: None,
        activate_sweep: false,
    }
}

fn args_with_path(path: &str) -> Args {
    let mut args = args_with_raw_instance("");
    args.instance = None;
    args.instance_path = Some(path.to_string());
    args
}

#[test]
fn parses_a_raw_comma_separated_instance_into_city_indices() {
    let cities = vec![city(10), city(20), city(30)];
    let reader = InstanceReader::new(&args_with_raw_instance("30,10,20"));

    assert_eq!(reader.get_parsed_instance(&cities), vec![2, 0, 1]);
}

#[test]
fn trims_whitespace_and_ignores_empty_tokens() {
    let cities = vec![city(1), city(2)];
    let reader = InstanceReader::new(&args_with_raw_instance(" 1 , , 2 "));

    assert_eq!(reader.get_parsed_instance(&cities), vec![0, 1]);
}

#[test]
fn returns_empty_vec_for_unknown_city_id() {
    let cities = vec![city(1), city(2)];
    let reader = InstanceReader::new(&args_with_raw_instance("1,99"));

    assert!(reader.get_parsed_instance(&cities).is_empty());
}

#[test]
fn returns_empty_vec_for_non_numeric_token() {
    let cities = vec![city(1)];
    let reader = InstanceReader::new(&args_with_raw_instance("1,abc"));

    assert!(reader.get_parsed_instance(&cities).is_empty());
}

#[test]
fn reads_instance_from_file_path() {
    let cities = vec![city(5), city(6)];
    let mut path: PathBuf = std::env::temp_dir();
    path.push(format!("tsp-instance-reader-test-{}.txt", std::process::id()));
    fs::write(&path, "6,5").unwrap();

    let reader = InstanceReader::new(&args_with_path(path.to_str().unwrap()));
    let result = reader.get_parsed_instance(&cities);

    fs::remove_file(&path).unwrap();
    assert_eq!(result, vec![1, 0]);
}

#[test]
fn returns_empty_vec_when_instance_file_is_missing() {
    let cities = vec![city(1)];
    let reader = InstanceReader::new(&args_with_path("/no/such/path/for/tsp-tests.txt"));

    assert!(reader.get_parsed_instance(&cities).is_empty());
}

#[test]
#[should_panic(expected = "No instance specified")]
fn new_panics_when_neither_instance_nor_path_is_given() {
    let mut args = args_with_raw_instance("");
    args.instance = None;
    InstanceReader::new(&args);
}
