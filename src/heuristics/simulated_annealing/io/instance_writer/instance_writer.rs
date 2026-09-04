use std::fs;

use crate::heuristics::simulated_annealing::models::city::City;

pub struct InstanceWriter {
    path_to_instance: Option<String>,
}

impl InstanceWriter {
    pub fn new(path_to_instance: Option<String>) -> InstanceWriter {
        InstanceWriter { path_to_instance }
    }

    /// Renders an instance (indices into `cities`) as the comma separated list
    /// of city ids that `InstanceReader` expects.
    pub fn format_instance(cities: &[City], instance: &[usize]) -> Result<String, String> {
        let ids: Result<Vec<String>, String> = instance
            .iter()
            .map(|&idx| {
                cities
                    .get(idx)
                    .map(|city| city.id.to_string())
                    .ok_or_else(|| format!("city index {idx} out of range"))
            })
            .collect();

        Ok(ids?.join(","))
    }

    pub fn write_instance(&self, cities: &[City], instance: &[usize]) -> Result<(), String> {
        let rendered = InstanceWriter::format_instance(cities, instance)?;
        match self.path_to_instance.as_ref() {
            Some(path_to_instance) => {
                fs::write(path_to_instance, rendered).map_err(|err| err.to_string())
            }
            None => {
                println!("{rendered}");
                Ok(())
            }
        }
    }
}
