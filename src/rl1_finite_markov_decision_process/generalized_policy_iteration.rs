use crate::rl1_finite_markov_decision_process::{ State, Action, Reward, Policy, ExpResult, Value};
use crate::rl2_monte_carlo::{Episode,Trajectory,random_actor};

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct GPI<S,T,U> {
    pub policy: Policy<S,T>,
    pub value : Vec<Value<U>>
}

impl<S: Clone + PartialEq+ std::cmp::Ord,T: Clone + PartialEq> GPI<S,T,f64>{
    ///Evaluates State Value of Policy in Dynamic Programming
    #[allow(unused)]
    pub fn evaluation_dp(&self,vec_exp_res: Vec<Vec<ExpResult<S,T,f64>>>,discount: f64) -> GPI<S,T,f64>
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
        let mut after_gpi: GPI<S,T,f64> = self.evaluation_dp(vec_exp_res.clone(), discount);

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
pub fn episode_mc<F: Fn(State<S>,Action<T>,Reward<f64>) -> (State<S>,Reward<f64>) >(mc_state: State<S>,mc_policy: Policy<S,T>,function: F,repeat: usize) -> Episode<S,T,f64>
    where State<S>   : Clone + PartialEq + PartialOrd + Ord,
          Action<T>  : Clone + PartialEq + PartialOrd + Ord,
          Policy<S,T>: Clone + PartialEq + PartialOrd,
          S: Clone + PartialEq + PartialOrd + Ord,
          T: Clone + PartialEq + PartialOrd + Ord,
{   
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
pub fn evaluate_action_mc<F: Fn(State<S>,Action<T>,Reward<f64>) -> (State<S>,Reward<f64>) >(mc_state: State<S>,mc_policy: Policy<S,T>,function: F,repeat1: usize, repeat2: usize) -> GPI<S,T,f64>
    where State<S>   : Clone + PartialEq + PartialOrd + Ord,
          Action<T>  : Clone + PartialEq + PartialOrd + Ord,
          Policy<S,T>: Clone + PartialEq + PartialOrd,
          S: Clone + PartialEq + PartialOrd + Ord,
          T: Clone + PartialEq + PartialOrd + Ord,
          F: Clone
{
    let mut ea_episode: Vec<Episode<S,T,f64>> = vec![];
    for int_i in 0..repeat2{
        ea_episode.push(Self::episode_mc(mc_state.clone(), mc_policy.clone(), function.clone(), repeat1));
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
pub fn evaluated_greedy_mc<F: Fn(State<S>,Action<T>,Reward<f64>) -> (State<S>,Reward<f64>) >(mc_state: State<S>,mc_policy: Policy<S,T>,function: F,repeat1: usize, repeat2: usize,epsilon_soft: f64) -> GPI<S, T, f64>
    where   State<S>   : Clone + PartialEq + PartialOrd + Ord,
            Action<T>  : Clone + PartialEq + PartialOrd + Ord,
            Policy<S,T>: Clone + PartialEq + PartialOrd,
            S: Clone + PartialEq + PartialOrd + Ord,
            T: Clone + PartialEq + PartialOrd + Ord,
            F: Clone
{
    let mut egmc_gpi: GPI<S, T, f64> = Self::evaluate_action_mc(mc_state.clone(), mc_policy.clone(), function.clone(), repeat1, repeat2);
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



