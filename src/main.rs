mod heuristics;

use std::collections::HashMap;
use std::fs;

use clap::Parser;

use heuristics::simulated_annealing::models::{
    city::City,
    connection::Connection,
    db,
    problem::TravelSalesmanProblem,
    solution::Tour,
};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    instance: Option<String>,

    #[arg(short, long, default_value_t = 50000.0)]
    temperature: f64,

    #[arg(short, long, default_value_t = 0.98)]
    decay_factor: f64,

    #[arg(short, long)]
    seed: Option<u64>,

    #[arg(short, long, default_value_t = 5000)]
    lot_size: usize,

    #[arg(short, long, default_value_t = 10000)]
    max_lots: usize,

    #[arg(long)]
    db_path: Option<String>,
}

fn parse_instance(arg: &str, cities: &[City]) -> Result<Vec<usize>, String> {
    let index_by_id: HashMap<i32, usize> = cities
        .iter()
        .enumerate()
        .map(|(idx, city)| (city.id, idx))
        .collect();

    arg.split(',')
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(|token| {
            let id: i32 = token.parse().map_err(|_| format!("invalid city id: {token:?}"))?;
            index_by_id
                .get(&id)
                .copied()
                .ok_or_else(|| format!("city id {id} not found"))
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let connection = db::connect(args.db_path)?;
    let cities: Vec<City> = db::load_all(&connection)?;
    let connections: Vec<Connection> = db::load_all(&connection)?;
    let instance: Vec<usize> = match &args.instance {
        Some(arg) => parse_instance(arg, &cities)?,
        None => (0..cities.len()).collect(),
    };
    let tour = Tour::new(cities, instance, connections, args.seed);
    println!("Maximum {:.9}", tour.max_distance);
    println!("Normalizer {:.9}", tour.normalizer);
    println!("Initial evaluation {:.12}", tour.calculate_current_cost());

    let mut tsp = TravelSalesmanProblem::new(tour, args.temperature, args.decay_factor);

    tsp.accept_solutions();

    println!(
        "best cost {:.12}, feasible? {}",
        tsp.best_cost(),
        tsp.best_cost() < 1.0
    );
    println!("best solution {:?}", tsp.best_solution());

    Ok(())
}
