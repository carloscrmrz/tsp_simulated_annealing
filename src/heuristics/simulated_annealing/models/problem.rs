struct TwoCitySwap { i, j }
struct Tour(Vec<CityId>);
struct TravelSalesmanProblem { }

impl Problem for TravelSalesmanProblem {
    type Solution = Tour;
    type Move = TwoCitySwap;
    fn evaluate(&self, s: &Self::Solution) -> Cost {

    }
    fn delta_evaluate(&self, s: &Self::Solution,  m: &Self::Move) -> CostDelta {

    }
    fn feasibility(&self, s: &Self::Solution) -> Feasibility {

    }
    fn delta(&self, s: &Self::Solution, m: &Self::Move) -> CostDelta {

    }
}