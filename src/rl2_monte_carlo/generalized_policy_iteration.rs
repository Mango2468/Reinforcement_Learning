use crate::rl1_finite_markov_decision_process::{ State, Action, Reward, Policy, ExpResult, Value};

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct GPI<S,T,U> {
    pub policy: Policy<S,T>,
    pub value : Vec<Value<U>>
}

impl<S: Clone + PartialEq+ std::cmp::Ord,T: Clone + PartialEq> GPI<S,T,f64>{

    #[allow(unused)]
    pub fn evaluation(&self,vec_exp_res: Vec<Vec<ExpResult<S,T,f64>>>,discount: f64) -> GPI<S,T,f64>
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
    //Using Greedy Algorithm with Epsilon Soft Exploring
    //Epsilon Soft means the minimal probablity of doing Exploring
    pub fn evaluated_greedy(&self,vec_exp_res: Vec<Vec<ExpResult<S,T,f64>>>,discount: f64,epsilon_soft: f64) -> (GPI<S,T,f64>,f64) 
        where   Policy<S,T>: Clone,
                ExpResult<S,T,f64> : Clone,
                State<S> : Clone + Copy + PartialEq,
                Action<T> : Clone + Copy + PartialEq,
                Reward<f64> : Clone + Copy,
                Value<f64>  : Clone + Copy,
    {
        let mut dispersion: f64 = 0.0;
        let mut before_gpi: GPI<S,T,f64> = self.clone();
        let mut after_gpi: GPI<S,T,f64> = self.evaluation(vec_exp_res.clone(), discount);

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

}

