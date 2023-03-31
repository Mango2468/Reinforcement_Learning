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
    let int_repeats: usize = 3;
    //////////////////////////////////////////////////////
    let mut track1 : Track = Track { area: vec![] };
    //Track1 Processing
    {    
        for int_i in 0..32 {
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
        // for int_i in 0.. track1.area.len(){
        //     println!("layer: {} vector :{:?}",int_i, track1.area[int_i]);
        //     println!("-------------------------------------------------")
        // }
    }
    /////////////////////////////////////////////////////////////////////////////


    let car_action: Vec<Action<CarAction>> = vec![
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


    let mut cr_policy: Policy<(isize,isize,isize,isize,bool),CarAction> = Policy { 
        state0: vec![], 
        prob1: vec![], 
        actions: car_action.clone() 
    };
    {
        for int_i in 0..32{
            for int_j in 0..17{
                for int_m in 0..5{
                    for int_n in -2..5{
                        cr_policy.state0.push(State { state: (int_i,int_j,int_m,int_n,track1.area.clone()[int_i as usize][int_j as usize]) });
                        cr_policy.prob1.push(vec![0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1]);
                    }
                }
            }
        }

        // for int_i in 0..cr_policy.state0.len(){
        //     println!("state0: {:?} / prob1: {:?}", cr_policy.state0[int_i].clone().state,cr_policy.prob1[int_i].clone());
        // }
    }
    for int_b in (0..17).rev(){
        for int_a in (0..32).rev(){
            for int_c in 0..5{
                for int_d in -2..5{
                    if track1.area[int_a as usize][int_b as usize] == true {
                        let mut net_reward_vecter: Vec<Vec<f64>> = vec![vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![]];
                    
                        let car_race: (isize,isize,isize,isize) = (int_a,int_b,int_c,int_d);
                        for _int_i in 0..int_repeats{
                            let a = car_race_simulator(track1.clone(),car_race, cr_policy.clone());
                            for int_j in 0..cr_policy.actions.len(){
                                net_reward_vecter[int_j].push(a.trajectory.clone()[int_j].last().unwrap().reward.reward);
                            }
                        }
                        let mut vec_value : Vec<f64> = vec![];
                        for int_i in 0..cr_policy.actions.len(){
                            vec_value.push(0.0);
                            for int_j in 0..int_repeats{
                                vec_value[int_i] += net_reward_vecter.clone()[int_i][int_j] / int_repeats as f64;
                            }
                        }
                        let mut compare: f64 = -100.0;
                        let mut vec_compare : Vec<usize> = vec![];
                        for int_i in 0..vec_value.len(){
                            if compare < vec_value.clone()[int_i] {
                                compare = vec_value.clone()[int_i];
                            }
                        }
                        for int_i in 0..vec_value.len(){
                            if (compare - vec_value.clone()[int_i]).abs() <= 0.01{
                                vec_compare.push(int_i);
                            }
                        }
                        let j : usize = cr_policy.state0.binary_search(&State { state: (car_race.0,car_race.1,car_race.2,car_race.3,true) }).unwrap();
                        for int_i in 0..vec_value.len(){
                            match vec_compare.binary_search(&int_i){
                                Ok(_)  => {
                                    if int_i != 9{
                                        if vec_compare.last().unwrap() == &9
                                        {
                                            cr_policy.prob1[j][int_i] = 0.9/(vec_compare.len()-1) as f64;
                                        } else {
                                            cr_policy.prob1[j][int_i] = 0.9/vec_compare.len() as f64;
                                        }
                                    } else {
                                        cr_policy.prob1[j][int_i] = 0.1;
                                    }
                                    },
                                Err(_) => {
                                    if int_i != 9 {
                                        cr_policy.prob1[j][int_i] = 0.0;
                                    } else {
                                        cr_policy.prob1[j][int_i] = 0.1;
                                    }
                                }
                            }
                        }
                        println!("({},{},{},{}) => {:?}",int_a,int_b,int_c,int_d,vec_value.clone());
                        println!("({},{},{},{}) => {:?}",int_a,int_b,int_c,int_d,cr_policy.prob1[j].clone());
                        println!("--------------------------------------------------------------");
                    }
                }
            }
        }
    }
        
    
}

