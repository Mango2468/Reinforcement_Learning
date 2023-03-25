use std::{
    fmt::{Display,Debug},
};

//vector deduplictation
#[allow(unused)]
pub fn deduplication<T>(vec_a: Vec<T>) -> Vec<T>
    where   T: PartialOrd + Display + Copy + Debug + Clone +PartialEq,
            Vec<T> : Clone
{
    if vec_a.len() == 0{
        vec_a.clone()
    } else {
        let mut result : (Vec<T>,Vec<usize>) = (vec![vec_a.clone()[0]],vec![1]);
        for int_i in 1..vec_a.len(){
            let mut unique : usize = 1;
            for int_j in 0..result.0.len(){
                if vec_a[int_i] == result.0.clone()[int_j]{
                    result.1[int_j] += 1;
                    unique *= 0;
                } else {
                    unique *= 1;
                }
            }
            if unique == 1{
                result.0.push(vec_a.clone()[int_i]);
                result.1.push(1);
            }
        }
        result.0
    }
}

//structure of decision tree system
//dataset:(AxB), class:(1xB), num:(1xA)
#[derive(Debug, Clone)]
#[allow(unused)]
pub struct DecisionTree<T> {
    pub dataset: Vec<Vec<T>>,   //(AxB)
    pub num:   Vec<usize>,      //(1xA)
    pub class: Vec<usize>,      //(1xB)
}

//structure of decision tree node and leaf
//tree    : information of tree (type::DecisionTree)
//posirion: position of node : (floar num,room num in exact floar)
//history : vector of (floar num,room num in exact floar) 
//        : (1,1),(2,2),(3,2) means node departed from 
//        : (1st floar's 1st romm) => (2nd floar's 2nd room) => (3rd floar's 2nd room)
//entropy : information entropy of node
#[derive(Debug, Clone)]
#[allow(unused)]
pub struct NodeLeaf<T> {
    pub tree: DecisionTree<T>,
    pub position: (usize,usize),
    pub history: Vec<(usize,usize)>, 
    pub entropy: T,
}




impl NodeLeaf<f64> {
    pub fn is_err(&self) -> bool {
        if self.clone().tree.is_err() 
            && self.clone().entropy == self.clone().tree.info_entr().unwrap() 
            {
                true
            } else {
                false
            }
    }
    //
    #[allow(unused)]
    pub fn node_process(&self,x:usize)->Result<Vec<NodeLeaf<f64>>,String>{
        let mut vec_node: Vec<NodeLeaf<f64>> = vec![];
        let mut a = self.clone().tree.node_partition().unwrap();
        for int_i in 0..a.clone().len() {
            vec_node.push(NodeLeaf {
                tree      : a.clone()[int_i].clone(),
                position  : (self.clone().position.0 +1,int_i + x),
                history   : self.clone().history,
                entropy   : a.clone()[int_i].clone().info_entr().unwrap()
            });
            vec_node[int_i].history.push(self.clone().position)
        }
        if self.is_err() {
            Ok(vec_node.clone())
        } else {
            Err("".to_string())
        }
    }
}



impl DecisionTree<f64> {
    //judge DecisionTree has error
    #[allow(unused)]
    pub fn is_err(&self) -> bool{
        if self.clone().dataset.len() == self.clone().num.len()
        && self.clone().dataset[0].len() == self.clone().class.len() {
            true    //ok
        } else {
            false   //error
        }
    }


    //extract data which contained in property a
    //DecisionTree::dataset:(AxB) => the vector that extract only column number is a : (1xA)
    #[allow(unused)]
    pub fn extract(&self, a:usize) -> Result<Vec<f64>,String> {
        if self.dataset.len() == self.num.len() && a < self.class.len() {
            let vec_data : Vec<Vec<f64>> = self.dataset.clone();
            let mut vec_result :Vec<f64>= vec![];
            for int_i in 0..vec_data.len(){
                vec_result.push(vec_data[int_i][a]) ;
            }
            Ok(vec_result.clone())
        } else {
            Err("".to_string())
        }
    }

    //informational entropy
    //DecisionTree::dataset:(AxB) => Result<f64,String>
    //judge only based on last property
    //vec_result contains (DecisionTree::num)'s elements
    #[allow(unused)]
    pub fn info_entr(&self) -> Result<f64,String>  {
        let mut entropy: f64 = 0.0;
        let mut vec_result : Vec<Vec<usize>> = vec![]; 
        let vec_extract : Vec<f64> = self.clone().extract(self.clone().class.len()-1).unwrap();
        let vec_deduplication : Vec<f64> = deduplication(self.clone().extract(self.clone().class.len()-1).unwrap()) ;
        for _int_i in 0..vec_deduplication.clone().len(){
            vec_result.push(vec![]);
        }
        for int_j in 0..self.clone().num.len(){
            for int_i in 0..vec_deduplication.clone().len(){
                if vec_extract.clone()[int_j] == vec_deduplication.clone()[int_i]{
                    vec_result[int_i].push(self.clone().num[int_j]);
                }
            }
        }
        for int_i in 0..vec_result.clone().len() {
            entropy -= (vec_result.clone()[int_i].len() as f64/vec_extract.clone().len() as f64)*(vec_result.clone()[int_i].len() as f64/vec_extract.clone().len() as f64).log2();
        }
        if self.clone().is_err() {
            Ok(entropy)
        } else {
            Err("type DecisionTree has an error".to_string())
        }
    }

    //information gain
    //calculate the information purity increasement
    #[allow(unused)]
    pub fn info_gain(&self, a:usize) -> Result<f64,String> {
        let mut gain : f64 = self.clone().info_entr().unwrap();
        let vec_extract : Vec<f64> = self.clone().extract(a).unwrap();
        let vec_deduplication : Vec<f64> = deduplication(self.clone().extract(a).unwrap()) ;
        let mut vec_deci: Vec<DecisionTree<f64>> =vec![];
        for _int_i in 0..vec_deduplication.clone().len() {
            vec_deci.push(DecisionTree {dataset: vec![], num: vec![], class: vec![]});
        }
        for int_i in 0..vec_extract.clone().len(){
            for int_j in 0..vec_deduplication.clone().len() {
                if vec_extract.clone()[int_i] == vec_deduplication.clone()[int_j] {
                    vec_deci[int_j].dataset.push(self.clone().dataset[int_i].to_vec());
                    vec_deci[int_j].num.push(self.clone().num[int_i]);
                    vec_deci[int_j].class = self.clone().class.to_vec();
                }
            }
        }
        for int_i in 0..vec_deci.len() { 
            gain -=  vec_deci[int_i].info_entr().unwrap()*(vec_deci[int_i].num.len() as f64/ self.clone().clone().num.len() as f64);
        }
        if self.clone().is_err() && a < self.clone().class.len(){
            Ok(gain)
        } else {
            Err("type DecisionTree has an error".to_string())
        }
    }

    
    //node partitioning
    #[allow(unused)]
    pub fn node_partition(&self) -> Result<Vec<DecisionTree<f64>>, String>{
        let mut vec_result : Vec<DecisionTree<f64>> = vec![];
        let mut vec_infogain: Vec<f64> = vec![];
        let mut infogain: usize = 0;
        for int_i in 0..self.class.len()-1 {
           vec_infogain.push(self.clone().info_gain(int_i).unwrap());
        }
        for int_i in 0..self.class.len()-1 {
            if vec_infogain[infogain] < vec_infogain[int_i] {
                infogain = int_i;
            }
        }

        let vec_extract : Vec<f64> = self.clone().extract(infogain).unwrap();
        let vec_deduplication : Vec<f64> = deduplication(self.clone().extract(infogain).unwrap()) ;
        for _int_i in 0..vec_deduplication.clone().len() {
            vec_result.push(DecisionTree {dataset: vec![], num: vec![], class: vec![]});
        }
        for int_i in 0..vec_extract.clone().len(){
            for int_j in 0..vec_deduplication.clone().len() {
                if vec_extract.clone()[int_i] == vec_deduplication.clone()[int_j] {
                    vec_result[int_j].dataset.push(self.clone().dataset[int_i].to_vec());
                    vec_result[int_j].num.push(self.clone().num[int_i]);
                    vec_result[int_j].class = self.clone().class.to_vec();
                }
            }
        }

        for int_i in 0..vec_result.len(){
            for int_j in 0..vec_result[int_i].num.len(){
                vec_result[int_i].dataset[int_j].remove(infogain);
            }
            vec_result[int_i].class.remove(infogain);
        }

        if self.is_err() {
            Ok(vec_result.clone())
        }else {
            Err("".to_string())
        }
        
    }

    #[allow(unused)]
    pub fn node_change(&self,pos: (usize,usize),his: Vec<(usize,usize)>) ->NodeLeaf<f64>  {
        NodeLeaf {
            tree : self.clone(),
            position : pos,
            history : his,
            entropy : self.clone().info_entr().unwrap(),
        }
    }
}

impl NodeLeaf<f32> {
    pub fn is_err(&self) -> bool {
        if self.clone().tree.is_err() 
            && self.clone().entropy == self.clone().tree.info_entr().unwrap() 
            {
                true
            } else {
                false
            }
    }
    //
    #[allow(unused)]
    pub fn node_process(&self,x:usize)->Result<Vec<NodeLeaf<f32>>,String>{
        let mut vec_node: Vec<NodeLeaf<f32>> = vec![];
        let mut a = self.clone().tree.node_partition().unwrap();
        for int_i in 0..a.clone().len() {
            vec_node.push(NodeLeaf {
                tree      : a.clone()[int_i].clone(),
                position  : (self.clone().position.0 +1,int_i + x),
                history   : self.clone().history,
                entropy   : a.clone()[int_i].clone().info_entr().unwrap()
            });
            vec_node[int_i].history.push(self.clone().position)
        }
        if self.is_err() {
            Ok(vec_node.clone())
        } else {
            Err("".to_string())
        }
    }
}



impl DecisionTree<f32> {
    //judge DecisionTree has error
    #[allow(unused)]
    pub fn is_err(&self) -> bool{
        if self.clone().dataset.len() == self.clone().num.len()
        && self.clone().dataset[0].len() == self.clone().class.len() {
            true    //ok
        } else {
            false   //error
        }
    }


    //extract data which contained in property a
    //DecisionTree::dataset:(AxB) => the vector that extract only column number is a : (1xA)
    #[allow(unused)]
    pub fn extract(&self, a:usize) -> Result<Vec<f32>,String> {
        if self.dataset.len() == self.num.len() && a < self.class.len() {
            let vec_data : Vec<Vec<f32>> = self.dataset.clone();
            let mut vec_result :Vec<f32>= vec![];
            for int_i in 0..vec_data.len(){
                vec_result.push(vec_data[int_i][a]) ;
            }
            Ok(vec_result.clone())
        } else {
            Err("".to_string())
        }
    }

    //informational entropy
    //DecisionTree::dataset:(AxB) => Result<f32,String>
    //judge only based on last property
    //vec_result contains (DecisionTree::num)'s elements
    #[allow(unused)]
    pub fn info_entr(&self) -> Result<f32,String>  {
        let mut entropy: f32 = 0.0;
        let mut vec_result : Vec<Vec<usize>> = vec![]; 
        let vec_extract : Vec<f32> = self.clone().extract(self.clone().class.len()-1).unwrap();
        let vec_deduplication : Vec<f32> = deduplication(self.clone().extract(self.clone().class.len()-1).unwrap()) ;
        for _int_i in 0..vec_deduplication.clone().len(){
            vec_result.push(vec![]);
        }
        for int_j in 0..self.clone().num.len(){
            for int_i in 0..vec_deduplication.clone().len(){
                if vec_extract.clone()[int_j] == vec_deduplication.clone()[int_i]{
                    vec_result[int_i].push(self.clone().num[int_j]);
                }
            }
        }
        for int_i in 0..vec_result.clone().len() {
            entropy -= (vec_result.clone()[int_i].len() as f32/vec_extract.clone().len() as f32)*(vec_result.clone()[int_i].len() as f32/vec_extract.clone().len() as f32).log2();
        }
        if self.clone().is_err() {
            Ok(entropy)
        } else {
            Err("type DecisionTree has an error".to_string())
        }
    }

    //information gain
    //calculate the information purity increasement
    #[allow(unused)]
    pub fn info_gain(&self, a:usize) -> Result<f32,String> {
        let mut gain : f32 = self.clone().info_entr().unwrap();
        let vec_extract : Vec<f32> = self.clone().extract(a).unwrap();
        let vec_deduplication : Vec<f32> = deduplication(self.clone().extract(a).unwrap()) ;
        let mut vec_deci: Vec<DecisionTree<f32>> =vec![];
        for _int_i in 0..vec_deduplication.clone().len() {
            vec_deci.push(DecisionTree {dataset: vec![], num: vec![], class: vec![]});
        }
        for int_i in 0..vec_extract.clone().len(){
            for int_j in 0..vec_deduplication.clone().len() {
                if vec_extract.clone()[int_i] == vec_deduplication.clone()[int_j] {
                    vec_deci[int_j].dataset.push(self.clone().dataset[int_i].to_vec());
                    vec_deci[int_j].num.push(self.clone().num[int_i]);
                    vec_deci[int_j].class = self.clone().class.to_vec();
                }
            }
        }
        for int_i in 0..vec_deci.len() { 
            gain -=  vec_deci[int_i].info_entr().unwrap()*(vec_deci[int_i].num.len() as f32/ self.clone().clone().num.len() as f32);
        }
        if self.clone().is_err() && a < self.clone().class.len(){
            Ok(gain)
        } else {
            Err("type DecisionTree has an error".to_string())
        }
    }

    
    //node partitioning
    #[allow(unused)]
    pub fn node_partition(&self) -> Result<Vec<DecisionTree<f32>>, String>{
        let mut vec_result : Vec<DecisionTree<f32>> = vec![];
        let mut vec_infogain: Vec<f32> = vec![];
        let mut infogain: usize = 0;
        for int_i in 0..self.class.len()-1 {
           vec_infogain.push(self.clone().info_gain(int_i).unwrap());
        }
        for int_i in 0..self.class.len()-1 {
            if vec_infogain[infogain] < vec_infogain[int_i] {
                infogain = int_i;
            }
        }

        let vec_extract : Vec<f32> = self.clone().extract(infogain).unwrap();
        let vec_deduplication : Vec<f32> = deduplication(self.clone().extract(infogain).unwrap()) ;
        for _int_i in 0..vec_deduplication.clone().len() {
            vec_result.push(DecisionTree {dataset: vec![], num: vec![], class: vec![]});
        }
        for int_i in 0..vec_extract.clone().len(){
            for int_j in 0..vec_deduplication.clone().len() {
                if vec_extract.clone()[int_i] == vec_deduplication.clone()[int_j] {
                    vec_result[int_j].dataset.push(self.clone().dataset[int_i].to_vec());
                    vec_result[int_j].num.push(self.clone().num[int_i]);
                    vec_result[int_j].class = self.clone().class.to_vec();
                }
            }
        }

        for int_i in 0..vec_result.len(){
            for int_j in 0..vec_result[int_i].num.len(){
                vec_result[int_i].dataset[int_j].remove(infogain);
            }
            vec_result[int_i].class.remove(infogain);
        }

        if self.is_err() {
            Ok(vec_result.clone())
        }else {
            Err("".to_string())
        }
        
    }

    #[allow(unused)]
    pub fn node_change(&self,pos: (usize,usize),his: Vec<(usize,usize)>) ->NodeLeaf<f32>  {
        NodeLeaf {
            tree : self.clone(),
            position : pos,
            history : his,
            entropy : self.clone().info_entr().unwrap(),
        }
    }
}





