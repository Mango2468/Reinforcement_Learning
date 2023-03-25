use crate::rl1_finite_markov_decision_process::{ State, Action, Reward};

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







