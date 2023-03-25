#[allow(unused)]
use crate::la1_linear_algebra::*;
use crate::rl1_finite_markov_decision_process::{ Policy, ExpResult};

use super::{State, Action, Reward};

#[allow(unused)]
#[derive(Debug,Clone,Copy)]
pub struct Value<R>{
    pub value: R
}

#[allow(unused)]
pub fn policy_value<S>(policy: Policy<S,S>,vec_exp_res: Vec<Vec<ExpResult<S,S,f64>>>,ex_value: Vec<Value<f64>>,discount: f64) -> Vec<Value<f64>>
    where Policy<S,S>: Clone ,
          ExpResult<S,S,f64>: Clone ,
          State<S>: Clone + Copy + Eq,
          Action<S>: Clone + Copy + Eq,
          Reward<f64>: Clone + Copy,
          Value<f64>: Clone + Copy,
          S: Clone + Copy,

{
    let mut vec_result: Vec<Value<f64>> = vec![];
    for int_i in 0..policy.state0.len() {
        let mut net_value : f64 = 0.0;
        for int_j in 0..policy.actions.len(){
            for int_k in 0..vec_exp_res[int_i][int_j].len{
                for int_l in 0..policy.state0.len() {
                    if policy.clone().state0[int_l] 
                    == vec_exp_res.clone()[int_i][int_j].result[int_k]{
                        net_value = 
                        net_value + 
                            policy.prob1[int_i][int_j]
                            *vec_exp_res.clone()[int_i][int_j].prob2[int_k]
                            *(vec_exp_res.clone()[int_i][int_j].reward[int_k].reward
                                + discount * ex_value[int_l].value);
                        break;
                    }
                }
            }
        }
        vec_result.push(Value { value: net_value });
    }
    return vec_result;
}

