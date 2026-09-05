mod heuristics;

use std::thread;
use chrono::Local;
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

    let n_threads = args.threads.unwrap_or_else(|| 1);
    let started_at = Local::now();
    let (cities, instance, connections) = (&cities, &instance, &connections);

    let results: Vec<Result<(), String>> = thread::scope(|scope| {
        let handles: Vec<_> = (0..n_threads)
            .map(|i| {
                scope.spawn(move || {
                    let tour = Tour::new(cities, instance, connections, args.seed);
                    let file_output_name = InstanceWriter::output_file_name(i, &started_at);
                    let instance_writer =
                        InstanceWriter::new(cities, instance, Some(file_output_name));
                    let mut tsp =
                        TravelSalesmanProblem::new(tour, args.temperature, args.decay_factor);
                    tsp.accept_solutions();
                    instance_writer.write_instance(&tsp)
                })
            })
            .collect();

        handles
            .into_iter()
            .map(|handle| handle.join().expect("solver thread panicked"))
            .collect()
    });

    for result in results {
        result?;
    }
    Ok(())
}
