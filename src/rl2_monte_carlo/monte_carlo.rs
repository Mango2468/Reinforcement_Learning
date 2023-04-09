use std::f32::consts::E;

use crate::rl1_finite_markov_decision_process::{ State, Action, Reward, Policy, GPI, Value};
use rand::Rng;

/// Monte Carlo's public Structure
/// 
/// Episode has two elements, one is [trajectory] and second is [prob2].
/// 
/// [trajectory] is for recording the changes of [Trajectory<S,T,U>] Structure,
/// which contains [State<S>], [Action<T>], [Reward<U>] in each period.
/// 
/// [prob2] is for presenting pobablities of each [Action<T>] will occur.
/// ```
/// use crate::rl1_finite_markov_decision_process::{ State, Action, Reward};
/// use crate::rl2_monte_carlo::{Trajectory,Episode};
/// 
/// let episode_a : Episode<f64,f64,f64> = Episode {
///     trajectory: vec![
///                     //First Action's Trajectory(shares same State)
///                     vec![Trajectory {state: State { state: 0.0 },
///                                       action: Action { action: 0.0}, 
///                                       reward: Reward { reward: 1.0}},
///                         Trajectory {state: State { state: 2.0 },
///                                       action: Action { action: 0.0}, 
///                                       reward: Reward { reward: 1.0}}],
///                     //Second Action's Trajectory(shares same State)
///                     vec![Trajectory {state: State { state: 0.0 },
///                                       action: Action { action: 1.0}, 
///                                       reward: Reward { reward: 0.0}}],
///                 ],
///     prob2 : vec![
///                 // Probablity of First Action will occur
///                 0.5,
///                 // Probablity of Second Action will occur
///                 0.5
///                 ]
/// 
/// };
/// ```
#[allow(unused)]
#[derive(Debug,Clone)]
pub struct Episode<S,T,U> {
    pub trajectory: Vec<Vec<Trajectory<S,T,U>>>,
    pub prob2     : Vec<f64> 
}

#[allow(unused)]
#[derive(Debug,Clone)]
pub struct Trajectory<S,T,U> {
    pub state : State<S>,
    pub action: Action<T>,
    pub reward: Reward<U>
}

#[allow(unused)]
pub fn random_actor(vector: Vec<f64>) -> usize{
    let mut int_vector: Vec<usize> = vec![];
    let mut sum_vector: Vec<usize> = vec![];
    let mut int_a: usize = 0;
    for int_i in 0..vector.len(){
        int_vector.push((vector[int_i]*10000.0 )as usize) ;
        int_a +=int_vector.clone()[int_i];
        if int_i == 0 {
            sum_vector.push(int_vector.clone()[int_i]);
        } else {
            sum_vector.push(int_vector.clone()[int_i] + sum_vector.clone()[int_i-1]);
        }
    }
    let random_number: usize = rand::thread_rng().gen_range(0..int_a);
    let mut results: usize = 0;
    for int_i in 1..vector.len(){
        if random_number > sum_vector[int_i-1] && random_number < sum_vector[int_i] && int_vector[int_i] != 0{
            results = int_i;
        } 
    }
    return results;
}







