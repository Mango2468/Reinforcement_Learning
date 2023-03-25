use crate::rl1_finite_markov_decision_process::{ State, Action, Reward, Policy};
use crate::rl2_monte_carlo::{ Trajectory, Episode};
use rand::Rng;

#[allow(unused)]
#[derive(Debug,Clone,Copy)]
pub struct Card {
    pub card: (usize,usize)
}

#[allow(unused)]
#[derive(Debug,Clone,PartialEq)]
pub enum BlackJackAction {
    Start,
    Stick,
    Hit
}

#[allow(unused)]
pub fn open_state(state: State<Vec<Vec<Card>>>) -> State<(usize,usize,bool)> {
    let mut blackjack_state: State<(usize,usize,bool)> = State { state: (0,0,false) };
    let mut player_usable_ace : usize = 0;
    let mut dealer_usable_ace : usize = 0;
    for int_i in 0..state.state[0].len(){
        if state.state.clone()[0][int_i].card.0 < 10 {
            blackjack_state.state.0 += state.state.clone()[0][int_i].card.0;
        } else if state.state.clone()[0][int_i].card.0 < 14 {
            blackjack_state.state.0 += 10
        } else {
            player_usable_ace +=1;
            if blackjack_state.state.2 == false {
                blackjack_state.state.2 = true;
            }
        }
    }
    for int_j in 0..player_usable_ace {
        if blackjack_state.state.0 < 10 {
            blackjack_state.state.0 += 11;
        } else {
            blackjack_state.state.0 += 1;
        }
    }

    for int_i in 0..state.state[1].len()-1{
        if state.state.clone()[1][int_i].card.0 < 10 {
            blackjack_state.state.1 += state.state.clone()[1][int_i].card.0;
        } else if state.state.clone()[0][int_i].card.0 < 14 {
            blackjack_state.state.1 += 10
        } else {
            dealer_usable_ace +=1;
        }
    }
    for int_j in 0..dealer_usable_ace {
        if blackjack_state.state.1 < 10 {
            blackjack_state.state.1 += 11;
        } else {
            blackjack_state.state.1 += 1;
        }
    }
    return blackjack_state;
}

#[allow(unused)]
pub fn random_card(deck_len: usize) -> usize {
    rand::thread_rng().gen_range(0..deck_len)
}

impl Policy<(usize,usize,bool),BlackJackAction> {
    #[allow(unused)]
    pub fn game(&mut self,card_deck: Vec<Card>,card_state: State<Vec<Vec<Card>>>,open_state: State<(usize,usize,bool)>) -> Vec<(Reward<f64>,BlackJackAction,f64,Vec<Card>,State<Vec<Vec<Card>>>,State<(usize,usize,bool)>)>  {
        let mut vec_result : Vec<(Reward<f64>,BlackJackAction,f64,Vec<Card>,State<Vec<Vec<Card>>>,State<(usize,usize,bool)>)> = vec![];
        for int_i in 0..self.state0.len() {
            for int_j in 0..self.actions.len() {
                if open_state.clone().state == self.state0[int_i].clone().state {
                    let mut deck     : Vec<Card>                   = card_deck.clone();
                    let mut c_state  : State<Vec<Vec<Card>>>       = card_state.clone();
                    let mut op_state : State<(usize, usize, bool)> = open_state.clone();
                    let mut fin : bool = false ;
    
                    //If BlackJackAction is Stick
                    if self.actions[int_j].action == BlackJackAction::Stick {
                        //Busted Judgement
                        if op_state.state.0 > 21 && fin == false {//when Player's Cards are Busted
                            vec_result.push((Reward { reward : -1.0},BlackJackAction::Stick,self.prob1[int_i].clone()[int_j],deck.clone(),c_state.clone(),op_state.clone()));
                            fin = true;
                        } else {
                            //Dealer Card Open
                            if c_state.clone().state[1].last().unwrap().card.0 < 10 {
                                op_state.state.1 += c_state.clone().state[1].last().unwrap().card.0;
                            } else if c_state.clone().state[1].last().unwrap().card.0 < 14 {
                                op_state.state.1 += 10;
                            } else if op_state.state.1 < 10 {
                                op_state.state.1 += 11;
                            } else if op_state.state.1 == 10 {
                                op_state.state.1 += 11;
                                if fin == false {
                                    vec_result.push((Reward { reward : -1.0},BlackJackAction::Stick,self.prob1[int_i].clone()[int_j],deck.clone(),c_state.clone(),op_state.clone()));
                                    fin = true;
                                }
                            } else {
                                op_state.state.1 += 1;
                            }
    
                            //Dealer Card Hit
                            while op_state.state.1 < 17 {
                                let rand_num = random_card(deck.len());
                                c_state.state[1].push(deck.clone()[rand_num]);
    
                                if deck.clone()[rand_num].card.0 < 10 {
                                    op_state.state.1 += deck.clone()[rand_num].card.0;
                                } else if deck.clone()[rand_num].card.0 < 14 {
                                    op_state.state.1 += 10;
                                } else {
                                    op_state.state.1 += 1;
                                }
                                deck.remove(rand_num);
    
                                //When Dealer Cards are Busted
                                if op_state.state.1 > 21 && fin == false{
                                    vec_result.push((Reward { reward : 1.0},BlackJackAction::Stick,self.prob1[int_i].clone()[int_j],deck.clone(),c_state.clone(),op_state.clone()));
                                    fin = true;
                                }
                            }
    
                            //Push Vec_Result
                            if fin == false {
                                if op_state.state.1 > 21 {
                                    vec_result.push((Reward { reward : 1.0},BlackJackAction::Stick,self.prob1[int_i].clone()[int_j],deck.clone(),c_state.clone(),op_state.clone()));
                                } else if (21-op_state.state.0) > (21-op_state.state.1) {
                                    vec_result.push((Reward { reward : -1.0},BlackJackAction::Stick,self.prob1[int_i].clone()[int_j],deck.clone(),c_state.clone(),op_state.clone()));
                                } else if (21-op_state.state.0) < (21-op_state.state.1) {
                                    vec_result.push((Reward { reward : 1.0},BlackJackAction::Stick,self.prob1[int_i].clone()[int_j],deck.clone(),c_state.clone(),op_state.clone()));
                                } else if (21-op_state.state.0) == (21-op_state.state.1) {
                                    vec_result.push((Reward { reward : 0.0},BlackJackAction::Stick,self.prob1[int_i].clone()[int_j],deck.clone(),c_state.clone(),op_state.clone()));
                                }
                                fin = true;
                            }
                        }
    
                    } else if self.actions[int_j].action == BlackJackAction::Hit {
                        //when Player's Cards are Busted
                        if op_state.state.0 > 21 && fin == false{
                            vec_result.push((Reward { reward : -1.0},BlackJackAction::Hit,self.prob1[int_i].clone()[int_j],deck.clone(),c_state.clone(),op_state.clone()));
                            fin = true;
                        } else {
                            //player card hit
                            let rand_num = random_card(deck.len());
                            c_state.state[0].push(deck.clone()[rand_num]);
    
                            if deck.clone()[rand_num].card.0 < 10 {
                                op_state.state.0 += deck.clone()[rand_num].card.0 ;
                            } else if deck.clone()[rand_num].card.0 < 14 {
                                op_state.state.0 += 10;
                            } else if op_state.state.0 <= 10 {
                                op_state.state.0 += 11;
                                op_state.state.2 = true;
                            } else {
                                op_state.state.0 += 1;
                                op_state.state.2 = true;
                            }
                            deck.remove(rand_num);
    
                            //When Player Cards are Busted
                            if op_state.state.0 > 21 && fin == false {
                                vec_result.push((Reward { reward : -1.0},BlackJackAction::Hit,self.prob1[int_i].clone()[int_j],deck.clone(),c_state.clone(),op_state.clone()));
                                fin = true;
                            } else if op_state.state.0 == 21 && fin == false{
                                vec_result.push((Reward { reward : 1.0},BlackJackAction::Hit,self.prob1[int_i].clone()[int_j],deck.clone(),c_state.clone(),op_state.clone()));
                                fin = true;
                            }
    
                            //Dealer Card Open
                            if c_state.clone().state[1].last().unwrap().card.0 < 10 {
                                op_state.state.1 += c_state.clone().state[1].last().unwrap().card.0;
                            } else if c_state.clone().state[1].last().unwrap().card.0 < 14 {
                                op_state.state.1 += 10;
                            } else if op_state.state.1 < 10 {
                                op_state.state.1 += 11;
                            } else if op_state.state.1 == 10 {
                                op_state.state.1 += 11;
                                if fin == false {
                                    vec_result.push((Reward { reward : -1.0},BlackJackAction::Hit,self.prob1[int_i].clone()[int_j],deck.clone(),c_state.clone(),op_state.clone()));
                                    fin = true;
                                }                            
                            } else {
                                op_state.state.1 += 1;
                            }
    
                            //Dealer Card Hit
                            while op_state.state.1 < 17 {
                                let rand_num = random_card(deck.len());
                                c_state.state[1].push(deck.clone()[rand_num]);
    
                                if deck.clone()[rand_num].card.0 < 10 {
                                    op_state.state.1 += deck.clone()[rand_num].card.0;
                                } else if deck.clone()[rand_num].card.0 < 14 {
                                    op_state.state.1 += 10;
                                } else {
                                    op_state.state.1 += 1;
                                }
                                deck.remove(rand_num);
    
                                //When Dealer Cards are Busted
                                if op_state.state.1 > 21 && fin == false{
                                    vec_result.push((Reward { reward : 1.0},BlackJackAction::Hit,self.prob1[int_i].clone()[int_j],deck.clone(),c_state.clone(),op_state.clone()));
                                    fin = true;
                                }
                            }
    
                            //Push Vec_Result
                            if fin == false {
                                if op_state.state.1 > 21 {
                                    vec_result.push((Reward { reward : 1.0},BlackJackAction::Hit,self.prob1[int_i].clone()[int_j],deck.clone(),c_state.clone(),op_state.clone()));
                                } else if (21-op_state.state.0) > (21-op_state.state.1) {
                                    vec_result.push((Reward { reward : -1.0},BlackJackAction::Hit,self.prob1[int_i].clone()[int_j],deck.clone(),c_state.clone(),op_state.clone()));
                                } else if (21-op_state.state.0) < (21-op_state.state.1) {
                                    vec_result.push((Reward { reward : 1.0},BlackJackAction::Hit,self.prob1[int_i].clone()[int_j],deck.clone(),c_state.clone(),op_state.clone()));
                                } else if (21-op_state.state.0) == (21-op_state.state.1) {
                                    vec_result.push((Reward { reward : 0.0},BlackJackAction::Hit,self.prob1[int_i].clone()[int_j],deck.clone(),c_state.clone(),op_state.clone()));
                                }
                                fin = true;
                            }
                        }
                    }
                }

            }
        }
        return vec_result;
    }
}

#[allow(unused)]
pub fn blackjack_simulator(policy: Policy<(usize,usize,bool),BlackJackAction>,episode_vector: Vec<Vec<Vec<Vec<Episode<(usize,usize,bool),BlackJackAction,f64>>>>>) -> (Vec<Vec<Vec<Vec<Episode<(usize,usize,bool),BlackJackAction,f64>>>>>,f64)
    where Policy<(usize,usize,bool),BlackJackAction>: Clone, Vec<Vec<Vec<Vec<Episode<(usize,usize,bool),BlackJackAction,f64>>>>>: Clone, Vec<Vec<Vec<f64>>>:Clone
{
    //Cloning
    let mut bj_policy: Policy<(usize,usize,bool),BlackJackAction> = policy.clone();
    let mut net_episode_vectors: Vec<Vec<Vec<Vec<Episode<(usize,usize,bool),BlackJackAction,f64>>>>> = episode_vector.clone();

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

    return (net_episode_vectors,net_reward);
}

