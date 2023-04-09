
use std::fmt::Debug;

use crate::rl1_finite_markov_decision_process::{ State, Action, Reward, Policy, GPI, Value};
use crate::rl2_monte_carlo::{Trajectory,Episode,random_actor};
use rand::Rng;

//Epsilon Soft Temporal Difference Value Expector
#[allow(unused)]
pub fn evaluate_td<S,A,F: Fn(State<S>,Action<A>,Reward<f64>) -> (State<S>,Reward<f64>) >(td_policy: Policy<S,A>,function: F,repeat: usize, alpha: f64, gamma: f64,epsilon_soft: f64,innit: Vec<State<S>>,fin: Vec<State<S>>) -> Policy<S,A>
    where F: Clone, S:Clone + PartialEq + Ord + Debug, A: Clone + PartialEq + Ord + Debug
{   
    //State Initialize(Vector State0 Innitialize & Finnish)
    let mut vec_s0_innit: Vec<State<S>> = innit.clone();
    let mut vec_s0_fin  : Vec<State<S>> = fin.clone();
    let mut vec_traj: Vec<Vec<Trajectory<S,A,f64>>> = vec![];
    let mut s: Vec<State<S>> = vec![];
    let mut a: Vec<Action<A>> = vec![];
    let mut r: Vec<Reward<f64>> = vec![];
    let mut policy : Policy<S,A> = td_policy.clone();

    let mut randnum: usize = 0;
    let mut epi_num: usize = 0;

    //Initialize Value Vector: ∀s∈S, ∀a∈A(s) st. Q(s,a) = 0.0
    let mut vec_value : Vec<Vec<f64>> = vec![];
    for int_i in 0..policy.state0.len(){
        vec_value.push(vec![]);
        for int_j in 0..policy.actions.len(){
            vec_value[int_i].push(0.0);
        }
    }
    
    {
        vec_traj.push(vec![]);
        s.push(vec_s0_innit[0].clone());
        s.push(vec_s0_innit[0].clone());
        randnum = random_actor(policy.prob1[policy.state0.binary_search(& s[0].clone()).unwrap()].clone());
        a.push(policy.actions[randnum].clone());
        a.push(policy.actions[randnum].clone());
        r.push(Reward { reward: -1.0 });
        r.push(Reward { reward: -1.0 });
    }
    
    for int_i in 0..repeat{

        //Each Stages Movement(Trajectories)
        let mut s1r1: (State<S>,Reward<f64>) = function(s[0].clone(),a[0].clone(),r[0].clone());
       
        //After a0 is executed, Observing r1 & s1
        s[1] = s1r1.0.clone();
        r[1] = s1r1.1.clone();

        //Renewing Policy according to Q(s,a)
        let mut compare: f64 = -100000.0;
        let mut vec_compare: Vec<usize> = vec![];
        for int_j in 0..policy.actions.len(){
            if vec_value[policy.state0.binary_search(&s[0].clone()).unwrap()][int_j] > compare{
                compare = vec_value[policy.state0.binary_search(&s[0].clone()).unwrap()][int_j];
            }
        }
        for int_j in 0..policy.actions.len(){
            if (compare - vec_value[policy.state0.binary_search(&s[0].clone()).unwrap()][int_j]).abs() <= 0.01{
                vec_compare.push(int_j);
            } 
        }
        for int_j in 0..policy.actions.len(){
            policy.prob1[policy.state0.clone().binary_search(&s[0].clone()).unwrap()][int_j] =
                match vec_compare.binary_search(&int_j){
                    Ok(_) => (1.0 - epsilon_soft)/vec_compare.len() as f64 + epsilon_soft/policy.actions.len() as f64,
                    _ => epsilon_soft/policy.actions.len() as f64
                }
        }

        //Following Q(s,a) deriving policy, Choose a1 from s1
        randnum = random_actor(policy.prob1[policy.state0.binary_search(& s[1].clone()).unwrap()].clone());
        a[1] = policy.actions[randnum].clone();

        //Value Renewing
        if int_i > 1 {
            match vec_s0_fin.binary_search(&s[0].clone()) {
                Err(_) => {
                    vec_value
                    [policy.state0.clone().binary_search(&s[0].clone()).unwrap()]
                    [policy.actions.search(a[0].clone()).unwrap()[0]] 
                    +=  alpha * (
                        vec_value
                        [policy.state0.clone().binary_search(&s[1].clone()).unwrap()]
                        [policy.actions.search(a[1].clone()).unwrap()[0]] * gamma + r[1].reward
                        -vec_value
                        [policy.state0.clone().binary_search(&s[0].clone()).unwrap()]
                        [policy.actions.search(a[0].clone()).unwrap()[0]]);
                },
                _     => {}
            }
        }

        //Renewing State,Action,Reward
        s[0].state = s[1].state.clone();
        a[0].action = a[1].action.clone();
        r[0].reward = r[1].reward.clone();

        //Recording Trajectory of State, Action, Reward
        vec_traj[epi_num].push(Trajectory { state: s[0].clone(), action: a[0].clone(), reward: r[0].clone() });
        println!("{}th epi's {}th traj: {:?}{:?}{:?}",epi_num,vec_traj[epi_num].len()-1,s[0].clone(),a[0].clone(),r[0].clone());
        println!("-------------------------------------------------------------------");

        //Observing s0 is Finnish Statement
        match vec_s0_fin.binary_search(&s[0].clone()) {
            Ok(_) => {
                vec_traj.push(vec![]);
                epi_num += 1;
                s[0] = vec_s0_innit[0].clone();
                randnum = random_actor(policy.prob1[policy.state0.binary_search(& s[0].clone()).unwrap()].clone());
                a[0] = policy.actions[randnum].clone();
                r[0].reward = -1.0;
            },
            _     => {}
        };
    }

    return policy;
}

pub trait Searchable<T> {
    fn search(&self,x:T) -> Result<Vec<usize>,String> ;
}


impl<T: PartialEq + Clone> Searchable<T> for Vec<T>{
    fn search(&self,x:T) -> Result<Vec<usize>,String>
    {   
        let mut vec_search: Vec<usize> = vec![];
        for int_i in 0..self.len(){
            if self.clone()[int_i] == x {
                vec_search.push(int_i);
            }
        }
        match vec_search.len() {
            0 => Err("Can't Find".to_string()),
            _ => Ok(vec_search.clone())
        }
    }
}