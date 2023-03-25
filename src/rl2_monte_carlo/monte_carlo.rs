use crate::rl1_finite_markov_decision_process::{ State, Action, Reward};

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
#[derive(Debug,Clone)]
pub struct Policy<S,T> {
    pub state0 : Vec<State<S>>,
    pub prob1  : Vec<Vec<f64>>,
    pub actions: Vec<Vec<Action<T>>>
}






