mod heuristics;

use clap::Parser;

use heuristics::simulated_annealing::io::{
    args::Args,
    instance_reader::InstanceReader,
    instance_writer::InstanceWriter,
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
    let tour = Tour::new(&cities, &instance, &connections, args.seed);

    let instance_writer = InstanceWriter::new(cities, instance, Some(String::from("./results/output.txt")));
    let mut tsp = TravelSalesmanProblem::new(tour, args.temperature, args.decay_factor);
    tsp.accept_solutions();

    instance_writer.write_instance(&tsp)?;
    Ok(())
}
