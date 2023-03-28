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
use rl2_monte_carlo::{Trajectory,Episode, Track, Car, CarAction,car_race,car_race_simulator,random_actor_mili};
// use crate::ml2_decision_tree::{DecisionTree,NodeLeaf}

fn main() {
    let mut track1 : Track = Track { area: vec![] };
    //Track1 Processing
    {    for int_i in 0..32 {
        track1.area.push(vec![]);
        if int_i < 26 {
            for _int_j in 0..9{
                track1.area[int_i].push(true);
            }
            for _int_j in 9..17{
                track1.area[int_i].push(false);
            }
        } else {
            for _int_j in 0..17 {
                track1.area[int_i].push(true);
            }
        }
    }
    track1.area[25][9] = true;
    for int_i in 0..32 {
        if int_i < 3 {
            for int_j in 0..3{
                track1.area[int_i][int_j] = false;
            }
        } else if int_i < 10 {
            for int_j in 0..2{
                track1.area[int_i][int_j] = false;
            }
        } else if int_i < 18 {
            track1.area[int_i][0] = false;
        } else if int_i < 28{

        } else if int_i < 29 {
            track1.area[int_i][0] = false;
        } else if int_i < 31 {
            for int_j in 0..2{
                track1.area[int_i][int_j] = false;
            }
        } else {
            for int_j in 0..3{
                track1.area[int_i][int_j] = false;
            }
        }
    }
    for int_i in 0.. track1.area.len(){
        println!("layer: {} vector :{:?}",int_i, track1.area[int_i]);
        println!("-------------------------------------------------")
    }}

    let mut car_state: State<Car> = State { state: Car { position: (0,3), velocity: (0,0), inside: true } };

    let mut car_action: Vec<Action<CarAction>> = vec![
        Action {action: CarAction::Accel { x: -1, y: -1 }},
        Action {action: CarAction::Accel { x: -1, y:  0 }},
        Action {action: CarAction::Accel { x: -1, y:  1 }},
        
        Action {action: CarAction::Accel { x:  0, y: -1 }},
        Action {action: CarAction::Accel { x:  0, y:  0 }},
        Action {action: CarAction::Accel { x:  0, y:  1 }},

        Action {action: CarAction::Accel { x:  1, y: -1 }},
        Action {action: CarAction::Accel { x:  1, y:  0 }},
        Action {action: CarAction::Accel { x:  1, y:  1 }},

        Action {action: CarAction::Error { x:  0, y:  0 }},
    ];

    car_state = car_race(track1.clone(), car_state.clone(), car_action.clone()[2],Reward { reward: 0.0 }).0;
    println!("{:?}",car_state.clone());


    let mut cr_policy: Policy<(isize,isize,bool),CarAction> = Policy { 
        state0: vec![], 
        prob1: vec![], 
        actions: car_action.clone() 
    };
    {
        for int_i in 0..32{
            for int_j in 0..17{
                cr_policy.state0.push(State { state: (int_i,int_j,track1.area.clone()[int_i as usize][int_j as usize]) });
                cr_policy.prob1.push(vec![0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1]);
            }
        }
        for int_i in 0..cr_policy.state0.len(){
            println!("state0: {:?} / prob1: {:?}", cr_policy.state0[int_i].clone().state,cr_policy.prob1[int_i].clone());
        }
    }

    car_race_simulator(track1.clone(), cr_policy.clone());
    
}

