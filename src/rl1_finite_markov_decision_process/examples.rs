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

}
mod rl2_monte_carlo {
    mod monte_carlo;
    pub use monte_carlo::*;

    mod blackjack;
    pub use blackjack::*;

    mod generalized_policy_iteration;
    pub use generalized_policy_iteration::*;
}


use rl1_finite_markov_decision_process::{State, Action, Reward, Policy, ExpResult};
use rl2_monte_carlo::{Card, BlackJackAction,Trajectory,Episode,GPI,open_state,random_card};
// use crate::ml2_decision_tree::{DecisionTree,NodeLeaf}

use rl1_finite_markov_decision_process::{Value, policy_value};

fn main() {
    //Especially, this example has extracted from the e-book "Reinforcement Learning: An Introduction", Chapter 3 "Finite Markov Decision Process", problem number 3-5.
    
    //First, construct the 'State' struct. => [State<S>] where S: (isize, isize)
    //Join the every 'State' struct into 'Vec<State>'.
    let mut vec_st : Vec<State<(isize,isize)>> = vec![];
    for int_i in 0..5{
        for int_j in 0..5{
            vec_st.push(
                State {
                     state: (int_i, int_j), 
                }
            );
        }
    }
    
    //Second, construct the 'Action' struct. => [Action<S>] where S: (isize, isize)
    //Join the every 'Action' struct into 'Vec<Action>'.
    let vec_ac : Vec<Action<(isize,isize)>> = vec![
        Action {action : ( 1, 0)},  //Down  (↓)  
        Action {action : (-1, 0)},  //Up    (↑)
        Action {action : ( 0,-1)},  //Left  (←)
        Action {action : ( 0, 1)}]; //Right (→)
    
    //Third, construct the 'Policy' struct. => [Policy<S>] where S: (isize, isize)
    //'Policy' struct has 4 elements.
    //'state0' has information about every 'State' struct. So it's type is 'Vec<State>'.
    //'prob1' has information about the 'conditional probability'. In this case, conditional probability means 'the probability of Action is occur when the State is given'.
    //So, 'prob1' is presented by matrix form. 
    // // In this analasys,'conditional probability' has significant realtions with 'state-transition probability'.
    //'prob1'::  Vec<   Vec<f64>    > : Outer Vector means "when the certain State is given", Inner Vector means "the probability of certain Action is occur". 
    //Therefore, sum of Inner Vector's elements have to be 1.0_f64.
    //'actions' means the set of actions which has occur when the certain State is given.
    //So, row size of 'prob1' and 'actions' is same, also column size of 'prob1' and 'actions' is same.
    let mut policy : Policy<(isize,isize),(isize,isize)> = 
        Policy { 
            state0: vec_st.clone(),
            prob1:  vec![],
            actions: vec_ac.clone(), 
        };

    for _int_i in 0..vec_st.len(){
        policy.prob1.push(vec![0.25,0.25,0.25,0.25]);
    }
    //Fourth, construct the 'ExpResult' struct. => [ExpResult<S,R>] where S: (isize, isize), R: f64.
    //'ExpResult' struct has informations about probabilities, expected rewards and expected result states when the State and Action are given.
    //Therefore, these informations have to be presented in Vector form.
    //But 'State' and 'Action' is also presented in Vector, so 'ExpResult' has to be joined with Vec<Vec<ExpResult>>.
    //'ExpResult'::  Vec<   Vec<ExpResult>    > : Outer Vector means "when the certain State is given", Inner Vector means "when the certain Action is also given", and 'ExpResult' means the expected results of environment.

    let vec_exp_res: Vec<Vec<ExpResult<(isize,isize),(isize,isize),f64>>>  = ExpResult::vec_exp_result(vec_st.clone(), vec_ac.clone(), f);

    // for int_i in 0..vec_st.len(){
    //     for int_j in 0..vec_ac.len(){
    //         for int_k in 0..vec_exp_res[int_i][int_j].prob2.len(){
    //             println!("[(state: {:?} + action: {:?}] X prob2: {:?}) => (result: {:?} , reward: {:?})",
    //             vec_exp_res[int_i][int_j].state.state,
    //             vec_exp_res[int_i][int_j].action.action,
    //             vec_exp_res[int_i][int_j].prob2[int_k],
    //             vec_exp_res[int_i][int_j].result[int_k].state,
    //             vec_exp_res[int_i][int_j].reward[int_k].reward);
    //         }
    //     }
    // }
    
    //Fifth, set up the beginning value of every 'State'.
    let mut ex_val: Vec<Value<f64>> = vec![];
    for _int_i in 0..vec_st.len(){
        ex_val.push(Value { value: 0.0 });
    }

    //Finnally, renew every 'State' value, untill the value is stabilized.
    //search 'optimal policy' through updating data. 
    //Update sequances are located in "Dynamic Programming"("Reinforcement Learning: An Introduction"- Chapter 4). 
    #[allow(unused)]
    let mut int_dispersion : f64 = 100.0;
    let mut int_i : usize = 0;
    //Repeat updating Policy and Value untill the policy and value data is stabilized.
    while int_dispersion > 0.01 {
        //Set Basic GPI
        let gpi: GPI<(isize,isize),(isize,isize),f64> = GPI { policy: policy.clone(), value: ex_val.clone() };
        //Using General Policy Iteration with Dynamic Programming Algorithm
        let set = gpi.clone().evaluated_greedy(vec_exp_res.clone(), 0.9, 0.00);

        // calculate the dispersion between Ex_Val and Policy_Expected Value
        int_dispersion = set.clone().1;

        ex_val = set.clone().0.value;
        policy = set.clone().0.policy;

        //print the 'policy'
        for int_j in 0..vec_st.len(){
            println!("period: {}  State: {}  Value: {:?}",int_i, int_j, set.clone().0.value[int_j]);
        }
        println!("------------------------Dispersion : {}--------------------------------",int_dispersion);
        int_i +=1;
    }
    for int_i in 0..policy.prob1.len(){
        println!("State: ({},{}) 's Prob1 => [D(↓): {:?}, U(↑): {:?}, L(←): {:?}, R(→): {:?}]",policy.clone().state0[int_i].state.0,policy.clone().state0[int_i].state.1,policy.clone().prob1[int_i][0],policy.clone().prob1[int_i][1],policy.clone().prob1[int_i][2],policy.clone().prob1[int_i][3]);
    }

}



///It's used for 'example 3-5'.
/// 
///This function is for calculate the informations about expected result states
///& reward & probability when State and Action is given.
/// ```
///Vec<(S,R,f64)>
///      | |  └―――――――> probability of reward and result is occur
///      | └――――――――――> Reaward::reward
///      └―――――――――――――> result State::state  
/// ```
fn f(st: (isize,isize), ac: (isize,isize)) -> Vec<((isize,isize),f64,f64)>  
    where f64: Copy{
    let i_result : (isize,isize) = (st.0+ac.0,st.1+ac.1);
    if st.0 == 0 && st.1 == 1{
        return vec![((4,1),10.0,1.0)] ;
    } else if st.0 == 0 && st.1 == 3{
        return vec![((2,3),5.0,1.0)];
    } else if i_result.0 >=0 && i_result.1 >=0 && i_result.0 <5 && i_result.1 <5{
        return vec![(i_result,0.0,1.0)];
    } else {
        return vec![(st.clone(),-1.0,1.0)];
    }
}

