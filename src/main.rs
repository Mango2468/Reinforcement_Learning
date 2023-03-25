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
use rl2_monte_carlo::{Card, BlackJackAction,Trajectory,Episode,open_state,random_card,blackjack_simulator};
// use crate::ml2_decision_tree::{DecisionTree,NodeLeaf}

fn main() {
    //Decide repeats
    let repeating: usize = 5000;
    //Initialize episode & reward net vectors
    let mut net_episode_vectors : Vec<Vec<Vec<Vec<Episode<(usize,usize,bool),BlackJackAction,f64>>>>> = vec![vec![],vec![]];
    let mut net_reward_vectors : Vec<Vec<Vec<f64>>> = vec![vec![],vec![]];
    let mut sum_net_reward: f64 = 0.0;
    for int_i in 0..12{
        net_episode_vectors[0].push(vec![]);
        net_episode_vectors[1].push(vec![]);
        net_reward_vectors[0].push(vec![]);
        net_reward_vectors[1].push(vec![]);
        for _int_j in 12..22{
            net_episode_vectors[0][int_i].push(vec![]);
            net_episode_vectors[1][int_i].push(vec![]);
            net_reward_vectors[0][int_i].push(0.0);
            net_reward_vectors[1][int_i].push(0.0);
        }
    } 

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
                        bj_policy.prob1.push(vec![0.0,1.0]);
                    } else {
                        bj_policy.state0.push(State { state: (int_k,int_j,false) });
                        bj_policy.prob1.push(vec![0.0,1.0]);
                    }
                } else {
                    if int_i == 0 {
                        bj_policy.state0.push(State { state: (int_k,int_j,true) });
                        bj_policy.prob1.push(vec![1.0,0.0]);
                    } else {
                        bj_policy.state0.push(State { state: (int_k,int_j,false) });
                        bj_policy.prob1.push(vec![1.0,0.0]);
                    }
                }
            }
        }
    }
    
    //Repeat Episode
    for _int_z in 0..repeating{
        let bj_simul = blackjack_simulator(bj_policy.clone(), net_episode_vectors);
        net_episode_vectors = bj_simul.0;
        sum_net_reward += bj_simul.1;
    }

    for int_i in 0..2{
        for int_j in 0..12{
            for int_k in 12..22{
                for int_l in 0..net_episode_vectors[int_i][int_j][int_k-12].len(){
                    net_reward_vectors[int_i][int_j][int_k-12] += net_episode_vectors[int_i][int_j][int_k-12].clone()[int_l].trajectory[0].last().unwrap().reward.reward/net_episode_vectors[int_i][int_j][int_k-12].len() as f64*net_episode_vectors[int_i][int_j][int_k-12].clone()[int_l].prob2[0];
                    net_reward_vectors[int_i][int_j][int_k-12] += net_episode_vectors[int_i][int_j][int_k-12].clone()[int_l].trajectory[1].last().unwrap().reward.reward/net_episode_vectors[int_i][int_j][int_k-12].len() as f64*net_episode_vectors[int_i][int_j][int_k-12].clone()[int_l].prob2[1];
                }
            }
        }
    }
    
    sum_net_reward = sum_net_reward/repeating as f64;

    for int_i in 0..2{
        for int_j in 0..12{
            println!("bool : {:?} , dealer : {:?}, net_reward : {:?}" ,int_i,int_j,net_reward_vectors[int_i][int_j]);
            println!("------------------------------------------------------------------");
            // for int_k in 12..22{
            //     for int_l in 0..net_episode_vectors[int_i][int_j][int_k-12].len(){
            //         println!("bool : {:?} , dealer : {:?}, player : {:?}, net_reward : {:?}" ,int_i,int_j,int_k,net_episode_vectors[int_i][int_j][int_k-12][int_l].trajectory);
            //         println!("------------------------------------------------------------------");
            //     }
            // }
        }
    }
    println!("{:?}",sum_net_reward);
}
