pub struct Cost(pub f64);
pub struct CostDelta(pub f64);
pub struct Iteration(pub u64);
pub struct Seed(pub u64);
pub struct RunnerId(pub u32);

pub enum Feasibility {
    Feasible,
    Infeasible { violation: f64 },
}
trait Move<S> {
    fn apply(&self, s: &mut S);
    fn undo(&self, s: &mut S);
}
pub trait SolutionStore<P: Problem>: Send {
    fn init(&mut self, initial: &P::Solution);
    fn on_move(&mut self, m: &P::Move);
    fn mark_best(&mut self, current: &P::Solution, at: Iteration);
    fn on_reset(&mut self, new_current: &P::Solution);
    fn best(&self, current: &P::Solution) -> Option<(P::Solution, Cost)>;
}

pub trait Solution {
    fn weight(&self, s: Self::Solution) -> f64;
    fn iteration(&self, s: Self::Solution) -> Iteration;
}