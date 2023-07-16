#[allow(unused)]
use crate::la1_linear_algebra::*;

#[allow(unused)]
#[derive(Debug,Clone)]
pub struct Environment<S,T,U> {
    pub state : State<S>,
    pub action: Action<T>,
    pub policy : Policy<S,T>,
    pub exp_result : ExpResult<S,T,U>,
    pub time  : usize, 
}


#[allow(unused)]
#[derive(Debug,Clone,Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct State<S>{
    pub state :S,
}


#[allow(unused)]
#[derive(Debug,Clone,Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Action<S> {
    pub action : S,
}


#[allow(unused)]
#[derive(Debug,Clone,Copy)]
pub struct Reward<R> {
    pub reward : R,
}


#[allow(unused)]
#[derive(Debug,Clone )]
pub struct Policy<S,T>{
    pub state0 : Vec<State<S>> ,    //state0.len()  = 25
    pub prob1  : Vec<Vec<f64>> ,    //prob1.len() = 25, prob1[0].len() = 4
    pub actions: Vec<Action<T>> ,   //actions.len() = 4
}


#[allow(unused)]
#[derive(Debug,Clone )]
pub struct PolicyConditional<T>{
    pub conditional: Vec<bool>  ,       //state0.len()  = 25
    pub probablity1: Vec<Vec<f64>> ,    //prob1.len() = 25, prob1[0].len() = 4
    pub action     : Vec<Vec<Action<T>>> ,   //actions.len() = 4
}


#[allow(unused)]
#[derive(Debug,Clone)]
pub struct ExpResult<S,T,R>{
    pub state : State<S>,
    pub action: Action<T>,
    pub prob2 : Vec<f64>,
    pub result: Vec<State<S>>,
    pub reward: Vec<Reward<R>>,
    pub len   : usize
}

#[allow(unused)]
pub fn outcomes<S:Copy,R:Copy,F:Fn(S,S) -> Vec<(S,R,f64)> >(state: State<S>, action: Action<S>,function:F ) -> Vec<(State<S>,Reward<R>,f64) >
{
    let mut vec_result :  Vec<(State<S>,Reward<R>,f64) > = vec![];
    for int_i in 0..(function)(state.state,action.action).len(){
        vec_result.push(
            (State { state : (function)(state.state,action.action)[int_i].0}, Reward{ reward : (function)(state.state,action.action)[int_i].1},(function)(state.state,action.action)[int_i].2 )
        );
    }
    return vec_result;
    
}

impl<S:Clone + Copy,R: Clone + Copy> ExpResult<S,S,R> where State<S>: Clone, Action<S>: Clone, Reward<R>: Clone{
    #[allow(unused)]
    pub fn vec_exp_result<F:Fn(S,S)->Vec<(S,R,f64)>>(vec_st: Vec<State<S>> , vec_ac: Vec<Action<S>>,f:F) -> Vec<Vec<ExpResult<S,S,R>>>
        where State<S>: Clone + Eq, Action<S>: Clone, Reward<R>: Clone,S: Clone + Copy, R:Clone +Copy
    {
        let mut vec_result: Vec<Vec<ExpResult<S,S,R>>> = vec![];
        for int_i in 0..vec_st.len(){
            vec_result.push(vec![]);
            for int_j in 0..vec_ac.len(){
                vec_result[int_i].push(    
                    ExpResult {
                        state: vec_st[int_i].clone(),
                        action: vec_ac[int_j].clone(),
                        prob2: vec![],
                        result: vec![],
                        reward: vec![],
                        len: 0
                    }
                );
                for int_k in 0..outcomes(vec_st[int_i].clone(), vec_ac[int_j].clone(), &f).len(){
                    vec_result[int_i][int_j].result.push(outcomes(vec_st[int_i].clone(), vec_ac[int_j].clone(), &f)[int_k].0);
                    vec_result[int_i][int_j].reward.push(outcomes(vec_st[int_i].clone(), vec_ac[int_j].clone(), &f)[int_k].1);
                    vec_result[int_i][int_j].prob2.push(outcomes(vec_st[int_i].clone(), vec_ac[int_j].clone(), &f)[int_k].2);
                }
                vec_result[int_i][int_j].len = outcomes(vec_st[int_i].clone(), vec_ac[int_j].clone(), &f).len();
            }
        }
        return vec_result;
    }

}

