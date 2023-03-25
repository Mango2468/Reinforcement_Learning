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


use rl1_finite_markov_decision_process::{State, Action, Reward, Policy};
use rl2_monte_carlo::{Card, BlackJackAction,Trajectory,Episode,open_state,random_card};
// use crate::ml2_decision_tree::{DecisionTree,NodeLeaf}

fn main() {
    //Decide repeats
    let repeating: usize = 10000;
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
    
    //Repeat and Generate 10000 Episodes
    for _int_z in 0..repeating{

        //Card Deck Processing
        let mut card_deck : Vec<Card> = vec![];
        for int_i in 0..4{
            for int_j in 2..15{//11:Jack,12:Queen,13:King,14:Ace
                card_deck.push(Card {card : (int_j,int_i)});
            }
        }

        //Card State Processing
        let mut card_state: State<Vec<Vec<Card>>> = State { 
            state: vec![
                vec![],           //Player's Card Vector
                vec![]            //Dealer's Card Vector
            ] 
        };
        #[allow(unused_assignments)]
        let mut bj_state : State<(usize,usize,bool)> = State {
            state : (0,0,false)
        };

        //Random Card Partition
        //Player: 2 Cards, Dealer 2 Cards
        for _int_i in 0..2{
            for int_j in 0..2{
                let rand_num = random_card(card_deck.len());
                card_state.state[int_j].push(card_deck.clone()[rand_num]);
                card_deck.remove(rand_num);
            }
        }

        //Player Open 2 Cards and Dealer Open 1 Card
        bj_state = open_state(card_state.clone());

        //If Player's cards are lower then 12 then Player Always Hits
        while bj_state.state.0 < 12 {
            let rand_num = random_card(card_deck.len());
            card_state.state[0].push(card_deck.clone()[rand_num]);
            card_deck.remove(rand_num);
            bj_state = open_state(card_state.clone());
        }

        //Processing Episode of BlackJack
        let mut bj_episode : Episode<(usize,usize,bool),BlackJackAction,f64> = Episode { 
            trajectory: vec![
                vec![                
                    Trajectory {
                        state  : bj_state.clone(),
                        action : Action { action: BlackJackAction::Start },
                        reward : Reward { reward: 0.0 }
                    }
                ],
                vec![                
                    Trajectory {
                        state  : bj_state.clone(),
                        action : Action { action: BlackJackAction::Start },
                        reward : Reward { reward: 0.0 }
                    }
                ]
            ], 
            prob2 : vec![]
        };

        let next_turn = bj_policy.game(card_deck.clone(), card_state.clone(), bj_state.clone());
        let mut net_reward: f64 = 0.0;
        for int_i in 0..next_turn.len() {
            net_reward += next_turn.clone()[int_i].0.reward * next_turn.clone()[int_i].2;
        }
        // println!("{:?}",next_turn[0].4);
        // println!("{:?}",next_turn[0].5);
        // println!("{:?}",next_turn[0].0);
        // println!("------------------------------------");
        // println!("{:?}",next_turn[1].4);
        // println!("{:?}",next_turn[1].5);
        // println!("{:?}",next_turn[1].0);
        // println!("//------------------------------------//");

        //Renew Episode
        for int_i in 0..next_turn.len() {
            bj_episode.trajectory[int_i].push(Trajectory { 
                state: next_turn.clone()[int_i].5, 
                action: Action { action: next_turn.clone()[int_i].clone().1} , 
                reward: next_turn.clone()[int_i].0
            });
            bj_episode.prob2.push(next_turn.clone()[int_i].2);
        }

        if bj_state.state.2 {
            net_episode_vectors[0][bj_state.state.clone().1][bj_state.state.clone().0-12].push(bj_episode.clone());
        } else {
            net_episode_vectors[1][bj_state.state.clone().1][bj_state.state.clone().0-12].push(bj_episode.clone());
        }
        sum_net_reward += net_reward;
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
