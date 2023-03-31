mod la1_linear_algebra{
    mod matrix;
    pub use matrix::*;

    mod vector;
    pub use vector::*;
}
mod ml2_decision_tree {
    mod decision_trees;
    pub use decision_trees::*;
}
mod rl1_finite_markov_decision_process{
    mod environment;
    pub use environment::*;

    mod value;
    pub use value::*;

    mod generalized_policy_iteration;
    pub use generalized_policy_iteration::*;

}
mod rl2_monte_carlo {
    mod monte_carlo;
    pub use monte_carlo::*;

    mod blackjack;
    pub use blackjack::*;

    mod right_turn;
    pub use right_turn::*;
}


use rl1_finite_markov_decision_process::{State, Action, Reward, Policy};
use rl2_monte_carlo::{Trajectory,Episode};
// use crate::ml2_decision_tree::{DecisionTree,NodeLeaf}

fn main() {
   
    
}

