mod heuristics;

use clap::Parser;

use heuristics::simulated_annealing::io::{
    args::Args,
    instance_reader::InstanceReader,
};
use heuristics::simulated_annealing::models::{
    city::City,
    connection::Connection,
    db,
    problem::TravelSalesmanProblem,
    solution::Tour,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let instance_reader = InstanceReader::new(&args);

    let connection = db::connect(args.db_path)?;
    let cities: Vec<City> = db::load_all(&connection)?;
    let connections: Vec<Connection> = db::load_all(&connection)?;
    let instance: Vec<usize> = instance_reader.get_parsed_instance(&cities);
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
    println!("seed {:?}", tsp.rng_seed());

    Ok(())
}
