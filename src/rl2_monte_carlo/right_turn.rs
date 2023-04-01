
use rand::Rng;

use crate::rl1_finite_markov_decision_process::{ State, Action, Reward, Policy};
use crate::rl2_monte_carlo::{ Trajectory, Episode};

#[allow(unused)]
#[derive(Debug,Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Track {
    pub area: Vec<Vec<bool>> 
}

#[allow(unused)]
#[derive(Debug,Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Car {
    pub position: (isize,isize),
    pub velocity: (isize,isize),
    pub inside  : bool
}

#[allow(unused)]
#[derive(Debug,Clone,Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CarAction {
    Start { x: isize, y: isize},
    Accel { x: isize, y: isize},
    Error { x: isize, y: isize}
}

#[allow(unused)]
pub fn car_race(track: Track, state: State<Car>, action: Action<CarAction>,reward: Reward<f64>) -> (State<Car>,Reward<f64>)
    where State<Car> : Clone + PartialEq, Action<CarAction>: Clone + PartialEq
{
    let mut s = state.clone();
    let mut v: (isize,isize) = s.state.clone().velocity;
    match action.action {
        CarAction::Accel { x, y } => {
            if (v.0 + y > 0 && v.0 + y < 5) && (v.1 + x > -2 && v.1 + x < 5) {
                v = (v.0 + y,v.1 + x);
            } else if (v.0 + y <= 0 || v.0 + y >= 5) && (v.1 + x > -2 && v.1 + x < 5) {
                v.1 = v.1 + x;
            } else if (v.1 + x <= -2 || v.1 + x >= 5) && (v.0 + y > 0 && v.0 + y < 5) {
                v.0 = v.0 + y;
            }
        },
        CarAction::Error  { x, y } => {
            v = v;
        }
        _=>{}
    }
    s.state.position.0 += v.0;
    s.state.position.1 += v.1;
    s.state.velocity = v;
    if s.state.position.0 >= 0 && s.state.position.0 < 32 && s.state.position.1 >= 0 && s.state.position.1 < 17{
        s.state.inside = track.area[s.state.position.0.abs() as usize][s.state.position.1.abs() as usize];
    } else {
        s.state.inside = false;
    }


    if s.state.inside == false {
        if s.state.position.1 > 16 && s.state.position.0 >= 24 && s.state.position.0 <= 33 {
            return (s,Reward { reward: reward.reward + 70.0 });
        }else {
            s.state.position = (0,rand::thread_rng().gen_range(3..9));
            s.state.velocity = (0,0);
            s.state.inside = true;
            return (s,Reward { reward: reward.reward -1.0 });
        }
    } else {
        if s.state.position.1 == 16 {
            return (s,Reward { reward: reward.reward + 70.0 });
        } else {
            return (s,Reward { reward: reward.reward -1.0});
        }
    }
    
}

#[allow(unused)]
pub fn car_race_simulator(track: Track ,car: (isize,isize,isize,isize),policy: Policy<(isize,isize,isize,isize,bool),CarAction>) -> Episode<(isize,isize,isize,isize,bool),CarAction,f64> {
    let track1: Track = track.clone();

    
    let mut cr_episode: Episode<(isize,isize,isize,isize,bool),CarAction,f64> 
        = Episode { 
            trajectory: vec![vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![]], 
            prob2:  policy.prob1[policy.clone().state0.binary_search(&State { state: (car.0,car.1,car.2,car.3,track1.area[car.0 as usize][car.1 as usize]) }).unwrap()].clone()
        };
    
    for int_i in 0..policy.actions.len(){
        let mut race: (State<Car>,Reward<f64>) = (State { state: Car { position: (car.0,car.1), velocity: (car.2,car.3), inside: true } },Reward { reward: -1.0});
        let mut trajectory: Vec<Trajectory<(isize,isize,isize,isize,bool),CarAction,f64>>=vec![ ];
        race = car_race(track1.clone(), race.0.clone(), policy.actions.clone()[int_i], race.1.clone()); 
        trajectory.push(
            Trajectory { 
                state: State { state: (race.0.state.position.0,race.0.state.position.1,race.0.state.velocity.0,race.0.state.velocity.1,race.0.state.inside) }, 
                action: policy.actions.clone()[int_i] ,
                reward: race.1
            }
        );
        let mut int_repeat: usize = 0;
        while trajectory.last().unwrap().reward.reward < 0.0 && int_repeat < 70{
            let states = State { state: (race.0.state.position.0,race.0.state.position.1,race.0.state.velocity.0,race.0.state.velocity.1,race.0.state.inside) };
            race = car_race(track1.clone(), race.0.clone(), policy.actions.clone()[random_actor_mili(policy.clone().prob1[policy.clone().state0.binary_search(& states.clone()).unwrap()].clone())] , race.1.clone()); 
            trajectory.push(
                Trajectory { 
                    state: states.clone(), 
                    action: policy.actions.clone()[random_actor_mili(policy.clone().prob1[policy.clone().state0.binary_search(& states.clone()).unwrap()].clone())],
                    reward: race.1
                }
            );
            // println!("repeat: {} trajec: {:?}",int_repeat,trajectory.clone().last().unwrap());
            // println!("-------------------------------------------");
            int_repeat += 1;
        }

        cr_episode.trajectory[int_i] = trajectory.clone();
    }
    return cr_episode;
    


}

#[allow(unused)]
pub fn random_actor_mili(vector: Vec<f64>) -> usize{
    let mut int_vector: Vec<usize> = vec![];
    let mut sum_vector: Vec<usize> = vec![];
    let mut int_a: usize = 0;
    for int_i in 0..vector.len(){
        int_vector.push((vector[int_i]*1000.0 )as usize) ;
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


