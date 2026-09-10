use super::super::solution::Tour;

pub struct TravelSalesmanProblem {
    pub tour: Tour,
    pub temperature: f64,
    pub decay_factor: f64,

    accepted_solutions: Vec<Vec<usize>>,
    minimal_solution: Vec<usize>,
    minimal_cost: f64,
    batch_size: usize,
    epsilon: f64,
}

impl TravelSalesmanProblem {
    pub fn new(tour: Tour, temperature: f64, decay_factor: f64, epsilon: f64, batch_size: usize) -> TravelSalesmanProblem {
        let initial_solution = tour.current_solution.clone();
        let minimal_cost = tour.current_cost();
        TravelSalesmanProblem {
            tour,
            temperature,
            decay_factor,
            accepted_solutions: vec![initial_solution.clone()],
            minimal_solution: initial_solution,
            minimal_cost,
            batch_size,
            epsilon,
        }
    }

    pub fn accept_solutions(&mut self) {
        let mut p = 0.0;
        while self.temperature > self.epsilon {
            let mut q = f64::INFINITY;
            while p <= q {
                q = p;
                let best_before = self.minimal_cost;
                let (average, accepted) = self.calculate_batch(self.batch_size);
                if accepted == 0 {
                    return;
                }
                p = average;
                if self.minimal_cost < best_before + self.temperature {
                    self.accepted_solutions.push(self.minimal_solution.clone());
                }
                self.tour.resync_cost();
            }
            self.regulate_temp();
        }
    }

    pub fn feasibility(&self) -> bool {
        self.minimal_cost < 1.0
    }

    pub fn calculate_batch(&mut self, size_lot: usize) -> (f64, usize) {
        let max_attempts = size_lot.saturating_mul(100);
        let mut accepted = 0;
        let mut cost_sum = 0.0;
        let mut counter = 0;

        while accepted < size_lot {
            if counter >= max_attempts {
                break;
            }
            counter += 1;

            let delta = self.tour.move_solution();
            if delta > self.temperature {
                self.tour.undo_move();
                continue;
            }
            accepted += 1;
            let cost = self.tour.current_cost();
            cost_sum += cost;

            if cost < self.minimal_cost {
                self.minimal_cost = cost;
                self.minimal_solution.clone_from(&self.tour.current_solution);
            }
        }

        let average = if accepted > 0 {
            cost_sum / accepted as f64
        } else {
            0.0
        };
        (average, accepted)
    }

    pub fn regulate_temp(&mut self) {
        self.temperature = self.temperature * self.decay_factor;
    }

    pub fn best_solution(&self) -> &[usize] {
        &self.minimal_solution
    }

    pub fn best_cost(&self) -> f64 {
        self.minimal_cost
    }

    pub fn accepted_solutions(&self) -> &[Vec<usize>] {
        &self.accepted_solutions
    }

    pub fn rng_seed(&self) -> u64 { self.tour.get_rng_seed() }

    pub fn do_downhill_sweep(&mut self) -> bool {
        let Some((p, q, _)) = self.tour.best_swap() else {
            return false;
        };

        self.tour.apply_move(p, q);
        let cost = self.tour.current_cost();
        self.accepted_solutions.push(self.tour.current_solution.clone());

        if cost < self.minimal_cost {
            self.minimal_cost = cost;
            self.minimal_solution.clone_from(&self.tour.current_solution);
        }

        true
    }

    pub fn descend(&mut self) -> usize {
        let mut moves = 0;
        while self.do_downhill_sweep() {
            moves += 1;
        }
        self.tour.resync_cost();
        self.minimal_cost = self.minimal_cost.min(self.tour.current_cost());
        moves
    }
}
