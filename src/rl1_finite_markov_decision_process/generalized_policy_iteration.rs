use crate::rl1_finite_markov_decision_process::{ State, Action, Reward, Policy, ExpResult, Value};
use crate::rl2_monte_carlo::{Episode,Trajectory,random_actor};
use std::fmt::Debug;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct GPI<S,T,U> {
    pub policy: Policy<S,T>,
    pub value : Vec<Value<U>>
}

impl<S: Clone + PartialEq+ std::cmp::Ord,T: Clone + PartialEq> GPI<S,T,f64>{
    ///Evaluates State Value of Policy in Dynamic Programming
    #[allow(unused)]
    pub fn evaluate_state_dp(&self,vec_exp_res: Vec<Vec<ExpResult<S,T,f64>>>,discount: f64) -> GPI<S,T,f64>
        where Policy<S,T>: Clone,
              ExpResult<S,T,f64> : Clone,
              State<S> : Clone + Copy + PartialEq,
              Action<T> : Clone + Copy + PartialEq,
              Reward<f64> : Clone + Copy,
              Value<f64>  : Clone + Copy,
    {
        let policy : Policy<S,T> = self.policy.clone();
        let ex_value: Vec<Value<f64>> = self.value.clone();
        let mut vec_result : Vec<Value<f64>> = vec![];
        for int_i in 0..policy.state0.len() {
            let mut net_value : f64 = 0.0;
            for int_j in 0..policy.actions.len() {
                for int_k in 0..vec_exp_res[int_i][int_j].len {

                    for int_l in 0..policy.state0.len() {
                        if policy.clone().state0[int_l]
                        == vec_exp_res.clone()[int_i][int_j].result[int_k]{
                            net_value +=  
                                policy.prob1[int_i][int_j]
                                *vec_exp_res.clone()[int_i][int_j].prob2[int_k]
                                *(vec_exp_res.clone()[int_i][int_j].reward[int_k].reward
                                + discount * ex_value[int_l].value);
                            break;
                        }
                    }
                }
            }
            vec_result.push(Value { value: net_value });
        }
        return GPI { policy : policy.clone(), value : vec_result};
    }


    #[allow(unused)]
    ///Using Greedy Algorithm with Epsilon Soft Exploring in Dynamic Programming
    ///Epsilon Soft means the minimal probablity of doing Exploring
    pub fn evaluated_greedy_dp(&self,vec_exp_res: Vec<Vec<ExpResult<S,T,f64>>>,discount: f64,epsilon_soft: f64) -> (GPI<S,T,f64>,f64) 
        where   Policy<S,T>: Clone,
                ExpResult<S,T,f64> : Clone,
                State<S> : Clone + Copy + PartialEq,
                Action<T> : Clone + Copy + PartialEq,
                Reward<f64> : Clone + Copy,
                Value<f64>  : Clone + Copy,
    {
        let mut dispersion: f64 = 0.0;
        let mut before_gpi: GPI<S,T,f64> = self.clone();
        let mut after_gpi: GPI<S,T,f64> = self.evaluate_state_dp(vec_exp_res.clone(), discount);

        //Renewing Dispersion using every policy's state0 elements
        for int_i in 0..self.policy.state0.len() {
            dispersion += (after_gpi.value.clone()[int_i].value - before_gpi.value.clone()[int_i].value).powi(2);
        }

        //Update Policy by Greedy way
        for int_i in 0..self.policy.state0.len() {
            //Search the Maximized Value
            let mut compare_value: f64 = -10.0;
            let mut compare_vec: Vec<usize> = vec![];

            for int_j in 0..self.policy.actions.len(){
                let a = self.policy.state0.binary_search(&vec_exp_res.clone()[int_i][int_j].result[0]).unwrap();
                if compare_value < after_gpi.clone().value[a].value {
                    compare_value = after_gpi.clone().value[a].value
                }
            }

            //search the maximizing value actions
            for int_j in 0..self.policy.actions.len(){
                let a = self.policy.state0.binary_search(&vec_exp_res.clone()[int_i][int_j].result[0]).unwrap();
                if (compare_value - after_gpi.clone().value[a].value).abs() <= 0.0001 {
                    compare_vec.push(int_j);
                }
            }

            //change the policy which can satisfy maximizing the values
            for int_j in 0..self.policy.actions.len(){
                after_gpi.policy.prob1[int_i][int_j] = 
                    match compare_vec.binary_search(&int_j){
                        Ok(_) => (1.0 - epsilon_soft)/compare_vec.len() as f64 + epsilon_soft/self.policy.actions.len() as f64,
                        Err(_) => epsilon_soft/self.policy.actions.len() as f64
                    }
            }
        }

        return (after_gpi,dispersion);
    }

///Epsilon Soft Monte Carlo Episode Processor
#[allow(unused)]
pub fn episode_mc<F: Fn(State<S>,Action<T>,Reward<f64>) -> (State<S>,Reward<f64>) >(&self,mc_state: State<S>,function: F,repeat: usize) -> Episode<S,T,f64>
    where State<S>   : Clone + PartialEq + PartialOrd + Ord,
          Action<T>  : Clone + PartialEq + PartialOrd + Ord,
          Policy<S,T>: Clone + PartialEq + PartialOrd,
          S: Clone + PartialEq + PartialOrd + Ord,
          T: Clone + PartialEq + PartialOrd + Ord,
{   
    let mc_policy: Policy<S, T> = self.policy.clone();
    //Make empty episode lists
    let mut mc_episode: Episode<S,T,f64> 
    = Episode { 
        trajectory: vec![], 
        prob2: mc_policy.prob1[mc_policy.state0.binary_search(&mc_state.clone()).unwrap()].clone()
    };
    for int_i in 0..mc_episode.prob2.len(){
        mc_episode.trajectory.push(vec![]);
    }

    for int_i in 0..mc_policy.actions.len(){
        //Initialize Trajectory
        let mut states: State<S> = mc_state.clone();
        let mut state_reward: (State<S>,Reward<f64>) = (mc_state.clone(),Reward { reward: 0.0});
        let mut trajectory: Vec<Trajectory<S,T,f64>> = vec![];
        state_reward = function(state_reward.0.clone(),mc_policy.actions[int_i].clone(),state_reward.1.clone());
        trajectory.push(
            Trajectory { 
                state: states.clone(), 
                action: mc_policy.actions[int_i].clone(), 
                reward: state_reward.1.clone() 
            }
        );

        //Repeating Processing Episodes
        let mut int_repeat1: usize = 0;
        let mut random_num : usize = 0;
        while trajectory.last().unwrap().reward.reward < 0.0 && int_repeat1 < repeat {
            states = state_reward.0.clone();
            random_num = random_actor(mc_policy.prob1[mc_policy.state0.binary_search(& states.clone()).unwrap()].clone());
            state_reward = function(state_reward.0.clone(),mc_policy.actions[random_num].clone(),state_reward.1.clone());
            trajectory.push(
                Trajectory { 
                    state: states.clone(), 
                    action: mc_policy.actions[random_num].clone(), 
                    reward: state_reward.1.clone() 
                }
            );
            int_repeat1 += 1;
        }
        mc_episode.trajectory[int_i] = trajectory.clone();
    }
    return mc_episode;
}

///Evaluate Action Value in Monte Carlo Algorithm
#[allow(unused)]
pub fn evaluate_action_mc<F: Fn(State<S>,Action<T>,Reward<f64>) -> (State<S>,Reward<f64>) >(&self,mc_state: State<S>,function: F,repeat1: usize, repeat2: usize) -> GPI<S,T,f64>
    where State<S>   : Clone + PartialEq + PartialOrd + Ord,
          Action<T>  : Clone + PartialEq + PartialOrd + Ord,
          Policy<S,T>: Clone + PartialEq + PartialOrd,
          S: Clone + PartialEq + PartialOrd + Ord,
          T: Clone + PartialEq + PartialOrd + Ord,
          F: Clone
{
    let mut mc_policy: Policy<S,T> = self.policy.clone();
    let mut ea_episode: Vec<Episode<S,T,f64>> = vec![];
    for int_i in 0..repeat2{
        ea_episode.push(self.episode_mc(mc_state.clone(), function.clone(), repeat1));
    }
    let mut ea_gpi: GPI<S,T,f64> = GPI { 
        policy: mc_policy.clone(), 
        value: vec![] };
    for int_i in 0..mc_policy.actions.len(){
        ea_gpi.value.push(Value { value: 0.0 });
        let mut vec_value: Vec<f64> = vec![];
        for int_j in 0..repeat2{
            ea_gpi.value[int_i].value += ea_episode[int_j].trajectory[int_i].clone().last().unwrap().reward.reward / repeat2 as f64;
        }
    }
    return ea_gpi;
}

///Using Greedy Algorithm with Epsilon Soft Exploring in Monte Carlo Algorithm
#[allow(unused)]
pub fn evaluated_greedy_mc<F: Fn(State<S>,Action<T>,Reward<f64>) -> (State<S>,Reward<f64>) >(&self,mc_state: State<S>,function: F,repeat1: usize, repeat2: usize,epsilon_soft: f64) -> GPI<S, T, f64>
    where   State<S>   : Clone + PartialEq + PartialOrd + Ord,
            Action<T>  : Clone + PartialEq + PartialOrd + Ord,
            Policy<S,T>: Clone + PartialEq + PartialOrd,
            S: Clone + PartialEq + PartialOrd + Ord,
            T: Clone + PartialEq + PartialOrd + Ord,
            F: Clone
{
    let mut mc_policy: Policy<S,T> = self.policy.clone();
    let mut egmc_gpi: GPI<S, T, f64> = self.evaluate_action_mc(mc_state.clone(), function.clone(), repeat1, repeat2);
    let mut vec_compare: Vec<usize> = vec![];

    let mut compare: f64 = -10000.0;
    for int_i in 0..mc_policy.actions.len(){
        if compare < egmc_gpi.value[int_i].clone().value {
            compare = egmc_gpi.value[int_i].clone().value;
        }
    }
    for int_i in 0..mc_policy.actions.len(){
        if (compare - egmc_gpi.value[int_i].clone().value) <= 0.001 {
            vec_compare.push(int_i);
        }
    }
    for int_i in 0..mc_policy.actions.len(){
        egmc_gpi.policy.prob1[egmc_gpi.policy.state0.binary_search(&mc_state.clone()).unwrap()][int_i] = 
            match vec_compare.binary_search(&int_i){
                Ok(_) => (1.0 - epsilon_soft)/vec_compare.len() as f64 + epsilon_soft/egmc_gpi.policy.actions.len() as f64 ,
                Err(_) => epsilon_soft/egmc_gpi.policy.actions.len() as f64
            }
    }
    return egmc_gpi;
}

}


//Epsilon Soft Temporal Difference Value Expector
#[allow(unused)]
pub fn evaluate_sarsa<S,A,F: Fn(State<S>,Action<A>,Reward<f64>) -> (State<S>,Reward<f64>) >(td_policy: Policy<S,A>,function: F,repeat: usize, alpha: f64, gamma: f64,epsilon_soft: f64,innit: Vec<State<S>>,fin: Vec<State<S>>) -> Policy<S,A>
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
        // randnum = random_actor(policy.prob1[policy.state0.binary_search(& s[0].clone()).unwrap()].clone());
        a.push(policy.actions[1].clone());
        a.push(policy.actions[1].clone());
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
        let mut compare: f64 = f64::MIN;
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
            match vec_s0_fin.search(s[0].clone()) {
                Err(_) => {
                    vec_value
                    [policy.state0.search(s[0].clone()).unwrap()[0]]
                    [policy.actions.search(a[0].clone()).unwrap()[0]] 
                    +=  alpha * (
                        vec_value
                        [policy.state0.search(s[1].clone()).unwrap()[0]]
                        [policy.actions.search(a[1].clone()).unwrap()[0]] * gamma + r[1].reward
                        -vec_value
                        [policy.state0.search(s[0].clone()).unwrap()[0]]
                        [policy.actions.search(a[0].clone()).unwrap()[0]]);
                },
                _     => {}
            }
        }

        //Recording Trajectory of State, Action, Reward
        vec_traj[epi_num].push(Trajectory { state: s[0].clone(), action: a[0].clone(), reward: r[0].clone() });
        println!("{}th epi's {}th traj: [r:{:?},a:{:?}r:{:?}]",epi_num,vec_traj[epi_num].len()-1,s[0].clone().state,a[0].clone().action,r[0].clone().reward);
        println!("-------------------------------------------------------------------");

        //Renewing State,Action,Reward
        s[0].state = s[1].state.clone();
        a[0].action = a[1].action.clone();
        r[0].reward = r[1].reward.clone();

        //Observing s0 is Finnish Statement
        match vec_s0_fin.binary_search(&s[0].clone()) {
            Ok(_) => {
                vec_traj.push(vec![]);
                epi_num += 1;
                s[0] = vec_s0_innit[0].clone();
                s[1] = vec_s0_innit[0].clone();
                randnum = random_actor(policy.prob1[policy.state0.binary_search(& s[0].clone()).unwrap()].clone());
                a[0] = policy.actions[randnum].clone();
                a[1] = policy.actions[randnum].clone();
                r[0].reward = -1.0;
                r[1].reward = -1.0;
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

//Epsilon Soft Temporal Difference Value Expector
#[allow(unused)]
pub fn evaluate_expected_sarsa<S,A,F: Fn(State<S>,Action<A>,Reward<f64>) -> (State<S>,Reward<f64>) >(td_policy: Policy<S,A>,function: F,repeat: usize, alpha: f64, gamma: f64,epsilon_soft: f64,innit: Vec<State<S>>,fin: Vec<State<S>>) -> Policy<S,A>
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
        // randnum = random_actor(policy.prob1[policy.state0.binary_search(& s[0].clone()).unwrap()].clone());
        a.push(policy.actions[1].clone());
        a.push(policy.actions[1].clone());
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
        let mut compare: f64 = f64::MIN;
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

        //Value Renewing
        if int_i > 1 {
            match vec_s0_fin.search(s[0].clone()) {
                Err(_) => {

                    //Average (Value * Prob1) => ExpVal
                    //ExpVal = Exp:π{Q(s[t+1],a[t+1])|s[t+1]}
                    //Exp:π{Q(s[t+1],a[t+1])|s[t+1]}  <- Σ:a{π(a|s[t+1]) * Q(s[t+1],a)}
                    let mut expected_value: f64 = 0.0;
                    for int_j in 0..policy.actions.len(){
                        expected_value 
                        += 
                        (vec_value[policy.state0.search(s[1].clone()).unwrap()[0]][int_j]
                        * policy.prob1[policy.state0.search(s[1].clone()).unwrap()[0]][int_j]);
                    }

                    //Value Renew
                    //Q(s[t],a[t]) <- Q(s[t],a[t]) + α * [r[t+1] + γ * Σ:a{π(a|s[t+1]) * Q(s[t+1],a)} - Q(s[t+1],a)]
                    //Q(s[t],a[t]) <- Q(s[t],a[t]) + α * [r[t+1] + γ * Exp:π{Q(s[t+1],a[t+1])|s[t+1]} - Q(s[t+1],a)]
                    vec_value
                    [policy.state0.search(s[0].clone()).unwrap()[0]]
                    [policy.actions.search(a[0].clone()).unwrap()[0]] 
                    +=  alpha * (
                        expected_value * gamma + r[1].reward
                        -vec_value
                        [policy.state0.search(s[0].clone()).unwrap()[0]]
                        [policy.actions.search(a[0].clone()).unwrap()[0]]);
                },
                _     => {}
            }
        }

        //Choose a[1] which Follows Changed Policy
        randnum = random_actor(policy.prob1[policy.state0.binary_search(& s[1].clone()).unwrap()].clone());
        a[1] = policy.actions[randnum].clone();

        //Recording Trajectory of State, Action, Reward
        vec_traj[epi_num].push(Trajectory { state: s[0].clone(), action: a[0].clone(), reward: r[0].clone() });
        println!("{}th epi's {}th traj: [r:{:?},a:{:?}r:{:?}]",epi_num,vec_traj[epi_num].len()-1,s[0].clone().state,a[0].clone().action,r[0].clone().reward);
        println!("-------------------------------------------------------------------");

        //Renewing State,Action,Reward
        s[0].state = s[1].state.clone();
        a[0].action = a[1].action.clone();
        r[0].reward = r[1].reward.clone();

        //Observing s0 is Finnish Statement
        match vec_s0_fin.binary_search(&s[0].clone()) {
            Ok(_) => {
                vec_traj.push(vec![]);
                epi_num += 1;
                s[0] = vec_s0_innit[0].clone();
                s[1] = vec_s0_innit[0].clone();
                randnum = random_actor(policy.prob1[policy.state0.binary_search(& s[0].clone()).unwrap()].clone());
                a[0] = policy.actions[randnum].clone();
                a[1] = policy.actions[randnum].clone();
                r[0].reward = -1.0;
                r[1].reward = -1.0;
            },
            _     => {}
        };
    }

    return policy;
}

