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
}


use rl1_finite_markov_decision_process::{State, Action, Reward, Policy};
use rl2_monte_carlo::{Card, BlackJackAction,Trajectory,Episode,open_state,random_card,blackjack_simulator,virtual_machine};
// use crate::ml2_decision_tree::{DecisionTree,NodeLeaf}

fn main() {

    //Processing Policy of BlackJack
    let mut bj_policy: Policy<(usize,usize,bool),BlackJackAction> = Policy { state0: vec![], prob1: vec![], actions: vec![] };

    //Initialize Policy which fits BlackJack 
    bj_policy.actions=vec![Action { action : BlackJackAction::Stick},Action { action : BlackJackAction::Hit}];
    for int_i in 0..2 {
        for int_j in 0..12{
            for int_k in 12..22{
                if int_k < 20 {
                    if int_i == 0 {
                        bj_policy.state0.push(State { state: (int_k,int_j,true) });
                        bj_policy.prob1.push(vec![1.0,0.0]);
                    } else {
                        bj_policy.state0.push(State { state: (int_k,int_j,false) });
                        bj_policy.prob1.push(vec![1.0,0.0]);
                    }
                } else {
                    if int_i == 0 {
                        bj_policy.state0.push(State { state: (int_k,int_j,true) });
                        bj_policy.prob1.push(vec![0.0,1.0]);
                    } else {
                        bj_policy.state0.push(State { state: (int_k,int_j,false) });
                        bj_policy.prob1.push(vec![0.0,1.0]);
                    }
                }
            }
        }
    }
    for __int_z in 0..5{
        bj_policy =  virtual_machine(bj_policy.clone(), 2000, 0.0);
    }
    
}
