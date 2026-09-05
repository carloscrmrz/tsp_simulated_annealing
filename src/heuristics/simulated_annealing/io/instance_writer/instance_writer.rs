use std::fs;
use std::path::Path;

use crate::heuristics::simulated_annealing::models::city::City;
use crate::heuristics::simulated_annealing::models::problem::TravelSalesmanProblem;

pub struct InstanceWriter {
    cities: Vec<City>,
    instance: Vec<usize>,
    path_to_output: Option<String>,
}

impl InstanceWriter {
    pub fn new(
        cities: Vec<City>,
        instance: Vec<usize>,
        path_to_output: Option<String>,
    ) -> InstanceWriter {
        InstanceWriter {
            cities,
            instance,
            path_to_output,
        }
    }

    /// The instance as it was read in, rendered as city ids in their original
    /// order.
    pub fn format_instance(&self) -> Result<String, String> {
        let ids: Result<Vec<String>, String> = self
            .instance
            .iter()
            .map(|&city_index| {
                self.cities
                    .get(city_index)
                    .map(|city| city.id.to_string())
                    .ok_or_else(|| format!("city index {city_index} out of range"))
            })
            .collect();

        Ok(ids?.join(","))
    }

    pub fn format_path(&self, solution: &[usize]) -> Result<String, String> {
        let ids: Result<Vec<String>, String> = solution
            .iter()
            .map(|&position| {
                let city_index = self
                    .instance
                    .get(position)
                    .ok_or_else(|| format!("solution position {position} out of range"))?;
                self.cities
                    .get(*city_index)
                    .map(|city| city.id.to_string())
                    .ok_or_else(|| format!("city index {city_index} out of range"))
            })
            .collect();

        Ok(ids?.join(","))
    }

    pub fn format_report(&self, tsp: &TravelSalesmanProblem) -> Result<String, String> {
        Ok(format!(
            "{:>10}: {}\n{:>10}: {}\n\n{:>10}: {:.9}\n{:>10}: {:.9}\n{:>10}: {:.9}\n{:>10}: {}\n{:>10}: {}\n",
            "Initial Path",
            self.format_instance()?,
            "Solution",
            self.format_path(tsp.best_solution())?,
            "Maximum",
            tsp.tour.max_distance,
            "Normalizer",
            tsp.tour.normalizer,
            "Evaluation",
            tsp.best_cost(),
            "Seed",
            tsp.rng_seed(),
            "Feasible",
            if tsp.feasibility() { "YES" } else { "NO" },
        ))
    }

    pub fn write_instance(&self, tsp: &TravelSalesmanProblem) -> Result<(), String> {
        let report = self.format_report(tsp)?;
        print!("{report}");

        match self.path_to_output.as_ref() {
            Some(path_to_output) => {
                if let Some(parent) = Path::new(path_to_output).parent() {
                    fs::create_dir_all(parent).map_err(|err| err.to_string())?;
                }
                fs::write(path_to_output, report).map_err(|err| err.to_string())
            }
            None => Ok(()),
        }
    }
}
