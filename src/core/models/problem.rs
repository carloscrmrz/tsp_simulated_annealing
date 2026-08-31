pub trait Problem: 'static {
    type Solution: Clone + 'static;
    type Move: Move<Self::Solution>;
    fn evaluate(&self, s: &Self::Solution) -> Cost;
    fn delta_evaluate(&self, s: &Self::Solution,  m: &Self::Move) -> CostDelta;
    fn feasibility(&self, s: &Self::Solution) -> Feasibility;
    fn delta(&self, s: &Self::Solution, m: &Self::Move) -> CostDelta;
}

