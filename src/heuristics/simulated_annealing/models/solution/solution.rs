use std::collections::{HashMap, HashSet};
use rand::{SeedableRng, rngs::StdRng, RngExt};

use super::super::city::City;
use super::super::connection::Connection;

pub struct Tour {
    pub augmented_weight_matrix: Vec<f64>,
    pub normalizer: f64,
    pub max_distance: f64,

    rand: StdRng,
    pub rng_seed: u64,

    pub current_solution: Vec<usize>,
    current_cost: f64,
    solution_last_move: (usize, usize),
    solution_last_delta: f64,
}

fn tri_index(n: usize, i: usize, j: usize) -> usize {
    i * (2 * n - i - 1) / 2 + (j - i - 1)
}

impl Tour {
    pub fn new(
        cities: &[City],
        instance: &[usize],
        connections: &[Connection],
        rng_seed: Option<u64>
    ) -> Tour {
        let seed = rng_seed.unwrap_or_else(|| rand::random::<u64>());
        let sub_connections = Self::_build_sub_connections(cities, instance, connections);

        let max_distance: f64 = Self::_calculate_max_distance(&sub_connections);
        let normalizer: f64 = Self::_calculate_normalizer(&sub_connections, instance.len());
        let augmented_weight_matrix: Vec<f64> =
            Self::_build_weight_matrix(cities, instance, &sub_connections);

        let current_solution: Vec<usize> = (0..instance.len()).collect();

        let mut tour = Tour {
            augmented_weight_matrix,
            normalizer,
            max_distance,
            current_solution,
            rand: StdRng::seed_from_u64(seed),
            rng_seed: seed,
            solution_last_move: (0, 0),
            solution_last_delta: 0.0,
            current_cost: 0.0,
        };
        tour.current_cost = tour.calculate_current_cost();
        tour
    }

    fn _build_sub_connections(
        cities: &[City],
        instance: &[usize],
        connections: &[Connection],
    ) -> Vec<Connection> {
        let instance_ids: HashSet<i32> = instance.iter().map(|&i| cities[i].id).collect();
        connections
            .iter()
            .filter(|conn| {
                instance_ids.contains(&conn.id_city_1) && instance_ids.contains(&conn.id_city_2)
            })
            .cloned()
            .collect()
    }

    fn _calculate_normalizer(connections: &[Connection], size: usize) -> f64 {
        let mut weights: Vec<f64> = connections.iter().map(|conn| conn.distance).collect();
        weights.sort_by(|a, b| b.total_cmp(a));
        weights.iter().take(size.saturating_sub(1)).sum()
    }

    fn _calculate_max_distance(connections: &[Connection]) -> f64 {
        connections
            .iter()
            .map(|conn| conn.distance)
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(0.0)
    }

    fn _build_weight_matrix(
        cities: &[City],
        instance: &[usize],
        connections: &[Connection],
    ) -> Vec<f64> {
        let size = instance.len();
        let max_distance = Self::_calculate_max_distance(connections);

        let known: HashMap<(i32, i32), f64> = connections
            .iter()
            .map(|conn| {
                let key = (
                    conn.id_city_1.min(conn.id_city_2),
                    conn.id_city_1.max(conn.id_city_2),
                );
                (key, conn.distance)
            })
            .collect();

        let mut weight_matrix: Vec<f64> = vec![0.0; size * size.saturating_sub(1) / 2];
        for i in 0..size {
            for j in (i + 1)..size {
                let city_1 = &cities[instance[i]];
                let city_2 = &cities[instance[j]];
                let key = (city_1.id.min(city_2.id), city_1.id.max(city_2.id));
                let weight = match known.get(&key) {
                    Some(&distance) => distance,
                    None => city_1.distance_to_city(city_2) * max_distance,
                };
                weight_matrix[tri_index(size, i, j)] = weight;
            }
        }

        weight_matrix
    }

    pub fn calculate_current_cost(&self) -> f64 {
        let n = self.current_solution.len();
        let mut sum = 0.0;
        for i in 1..n {
            let a = self.current_solution[i - 1];
            let b = self.current_solution[i];
            let (lo, hi) = if a < b { (a, b) } else { (b, a) };
            sum += self.augmented_weight_matrix[tri_index(n, lo, hi)];
        }
        sum / self.normalizer
    }

    pub fn calculate_cost(&self, possible_solution: &[usize]) -> f64 {
        let n = possible_solution.len();
        let mut sum = 0.0;
        for i in 1..n {
            let a = possible_solution[i - 1];
            let b = possible_solution[i];
            let (lo, hi) = if a < b { (a, b) } else { (b, a) };
            sum += self.augmented_weight_matrix[tri_index(n, lo, hi)];
        }
        sum / self.normalizer
    }

    pub fn move_solution(&mut self) -> f64 {
        let n = self.current_solution.len();
        if n < 2 {
            return 0.0;
        }
        let a = self.rand.random_range(0..n);
        let mut b = self.rand.random_range(0..n - 1);
        if b >= a {
            b += 1;
        }
        let (p, q) = if a < b { (a, b) } else { (b, a) };

        let before = self.incident_cost(p, q);
        self.current_solution.swap(p, q);
        let after = self.incident_cost(p, q);

        let delta = (after - before) / self.normalizer;
        self.current_cost += delta;
        self.solution_last_move = (p, q);
        self.solution_last_delta = delta;
        delta
    }

    pub fn undo_move(&mut self) {
        let (p, q) = self.solution_last_move;
        self.current_solution.swap(p, q);
        self.current_cost -= self.solution_last_delta;
    }

    fn edge_cost(&self, a: usize, b: usize) -> f64 {
        let n = self.current_solution.len();
        let (city_a, city_b) = (self.current_solution[a], self.current_solution[b]);
        let (lo, hi) = if city_a < city_b { (city_a, city_b) } else { (city_b, city_a) };
        self.augmented_weight_matrix[tri_index(n, lo, hi)]
    }

    fn incident_cost(&self, p: usize, q: usize) -> f64 {
        let n = self.current_solution.len();
        let mut sum = 0.0;
        if p > 0 {
            sum += self.edge_cost(p - 1, p);
        }
        sum += self.edge_cost(p, p + 1);
        if q > p + 1 {
            sum += self.edge_cost(q - 1, q);
        }
        if q + 1 < n {
            sum += self.edge_cost(q, q + 1);
        }
        sum
    }

    pub fn current_cost(&self) -> f64 {
        self.current_cost
    }

    pub fn resync_cost(&mut self) {
        self.current_cost = self.calculate_current_cost();
    }

    pub fn get_rng_seed(&self) -> u64 { self.rng_seed }
}
