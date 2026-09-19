use super::Args;
use clap::Parser;

#[test]
fn defaults_match_expected_values_when_no_flags_are_given() {
    let args = Args::try_parse_from(["simulated-annealing"]).unwrap();

    assert_eq!(args.instance, None);
    assert_eq!(args.instance_path, None);
    assert_eq!(args.temperature, 50000.0);
    assert_eq!(args.decay_factor, 0.98);
    assert_eq!(args.seed, None);
    assert_eq!(args.lot_size, 5000);
    assert_eq!(args.epsilon, 0.000000001_f64);
    assert_eq!(args.max_lots, 10000);
    assert_eq!(args.db_path, None);
    assert_eq!(args.threads, None);
    assert_eq!(args.concurrency, None);
    assert!(!args.activate_sweep);
}

#[test]
fn parses_long_flags_over_defaults() {
    let args = Args::try_parse_from([
        "simulated-annealing",
        "--instance", "1,2,3",
        "--temperature", "10.5",
        "--decay-factor", "0.5",
        "--seed", "42",
        "--lot-size", "100",
        "--epsilon", "0.01",
        "--max-lots", "5",
        "--db-path", "custom.db",
        "--threads", "4",
        "--concurrency", "2",
        "--activate-sweep",
    ])
    .unwrap();

    assert_eq!(args.instance, Some("1,2,3".to_string()));
    assert_eq!(args.temperature, 10.5);
    assert_eq!(args.decay_factor, 0.5);
    assert_eq!(args.seed, Some(42));
    assert_eq!(args.lot_size, 100);
    assert_eq!(args.epsilon, 0.01);
    assert_eq!(args.max_lots, 5);
    assert_eq!(args.db_path, Some("custom.db".to_string()));
    assert_eq!(args.threads, Some(4));
    assert_eq!(args.concurrency, Some(2));
    assert!(args.activate_sweep);
}

#[test]
fn instance_path_accepts_its_short_flag() {
    let args = Args::try_parse_from(["simulated-annealing", "-p", "input.tsp"]).unwrap();

    assert_eq!(args.instance_path, Some("input.tsp".to_string()));
}

#[test]
fn rejects_a_non_numeric_temperature() {
    let result = Args::try_parse_from(["simulated-annealing", "--temperature", "not-a-number"]);

    assert!(result.is_err());
}
