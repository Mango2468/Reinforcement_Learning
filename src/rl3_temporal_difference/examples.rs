use std::fmt::Debug;
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
mod rl3_temporal_difference {
    mod td_zero;
    pub use td_zero::*;

    mod q_learning;
    pub use q_learning::*;
}

use rl1_finite_markov_decision_process::{State, Action, Reward, Policy};
use rl2_monte_carlo::{Trajectory,random_actor};
use rl3_temporal_difference::{evaluate_td};
// use crate::ml2_decision_tree::{DecisionTree,NodeLeaf}

fn main() {

   let mut wg_policy: Policy<(isize,isize),(isize,isize)> 
    = Policy { 
            state0: vec![], 
            prob1: vec![], 
            actions: vec![Action { action: (-1,0)},Action { action: (1,0)},Action { action: (0,-1)},Action { action: (0,1)}]
    };
    {
        for int_i in 0..10{
            for int_j in 0..7{
                wg_policy.state0.push(State {state: (int_i,int_j)});
                wg_policy.prob1.push(vec![]);
            }
        }
        for int_i in 0..wg_policy.state0.len(){
            for _int_j in 0..wg_policy.actions.len(){
                wg_policy.prob1[int_i].push(1.0/wg_policy.actions.len() as f64);
            }
        }
    }


    let epi_policy = evaluate_td(wg_policy.clone(),wind_moves, 10000, 0.5, 1.0,0.01,vec![State { state: (0,3)}],vec![State { state: (7,3)}]);

    for int_i in 0..epi_policy.state0.len(){
        println!("prob1[{}]{:?} :{:?}",int_i,epi_policy.state0[int_i].state,epi_policy.prob1[int_i].clone());
    }
    println!("-----------------------------------------------");

    
}

#[allow(unused)]
fn wind_moves(state: State<(isize,isize)>,action: Action<(isize,isize)>,reward: Reward<f64>) -> (State<(isize,isize)>,Reward<f64>)
    where   Trajectory<(isize,isize),(isize,isize),f64>: Clone,
            State<(isize,isize)>: PartialOrd + Clone,
            Action<(isize,isize)>: PartialOrd + Clone
{
    let wind : isize = 
    match state.state.0 {
        0isize => 0,
        1isize => 0,
        2isize => 0,
        3isize => 1,
        4isize => 1,
        5isize => 1,
        6isize => 2,
        7isize => 2,
        8isize => 1,
        _     => 0,
    };
    if state.state.0 + action.action.0 == 7 && state.state.1 + action.action.1 + wind == 3{
        (State { state: (state.state.0 + action.action.0, state.state.1 + action.action.1 + wind) }, 
        Reward { reward: 0.0}) 

    } else {
        let mut x : isize = 0;
        if state.state.0 + action.action.0 < 0 {
            x = 0;
        } else if state.state.0 + action.action.0 >= 10 {
            x = 9;
        } else {
            x = state.state.0 + action.action.0;
        }
        

        let mut y : isize = 0;

        if state.state.1 + action.action.1 + wind < 0 {
            y = 0;
        } else if state.state.1 + action.action.1 + wind >= 7 {
            y = 6;
        } else {
            y = state.state.1 + action.action.1 + wind;
        }
        println!("wind:{:?}, s: {:?}, a: {:?} => ({:?},{:?})",wind,state.state,action.action,x,y);
        (State { state: (x, y) }, 
        Reward { reward: reward.reward -1.0})
    }
}

