
// mod la1_linear_algebra{
//     mod matrix;
//     pub use matrix::*;

//     mod vector;
//     pub use vector::*;
// }
// mod rl1_finite_markov_decision_process{
//     mod environment;
//     pub use environment::*;

//     mod value;
//     pub use value::*;

// }
// mod ml2_decision_tree {
//     mod deci_trees;
//     pub use deci_trees::*;
// }

use crate::ml2_decision_tree::{DecisionTree,NodeLeaf};


fn main() {
    let data1: DecisionTree<f64> = DecisionTree {
        dataset : vec![
            vec![1.0,2.0,2.0,2.0,2.0,1.0,1.0],
            vec![2.0,2.0,1.0,2.0,2.0,1.0,1.0],
            vec![2.0,2.0,2.0,2.0,2.0,1.0,1.0],
            vec![1.0,2.0,1.0,2.0,2.0,1.0,1.0],
            vec![0.0,2.0,2.0,2.0,2.0,1.0,1.0],
            vec![1.0,1.0,2.0,2.0,1.0,0.0,1.0],
            vec![2.0,1.0,2.0,1.0,1.0,0.0,1.0],
            vec![2.0,1.0,2.0,2.0,1.0,1.0,1.0],
            vec![2.0,1.0,1.0,1.0,1.0,1.0,0.0],
            vec![1.0,0.0,0.0,2.0,0.0,0.0,0.0],
            vec![0.0,0.0,0.0,0.0,0.0,1.0,0.0],
            vec![0.0,2.0,2.0,0.0,0.0,1.0,0.0],
            vec![1.0,1.0,2.0,1.0,2.0,1.0,0.0],
            vec![0.0,1.0,1.0,1.0,2.0,1.0,0.0],
            vec![2.0,1.0,2.0,2.0,1.0,0.0,0.0],
            vec![0.0,2.0,2.0,0.0,0.0,1.0,0.0],
            vec![1.0,2.0,1.0,1.0,1.0,1.0,0.0],
        ],
        num: vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17],
        class : vec![1,2,3,4,5,6,7],
    };
    let node1 : NodeLeaf<f64>= data1.clone().node_change((0,0), vec![]);
    let mut vec_nodeleaf : Vec<Vec<NodeLeaf<f64>>> = vec![vec![node1.clone()]];
    let mut a: f64 = 1.0;
    let mut f: usize = 0;
    #[allow(unused_assignments)]
    let mut x: usize = 0;
    while a !=0.0 {
        a=0.0;
        x=0;
        vec_nodeleaf.push(vec![]); //floar
        for int_i in 0..vec_nodeleaf.clone()[f].len(){
            if vec_nodeleaf.clone()[f][int_i].entropy != 0.0{
                let c = vec_nodeleaf.clone()[f][int_i].node_process(x).unwrap().to_vec();
                for int_j in 0..c.clone().len(){
                    vec_nodeleaf[f+1].push(
                        NodeLeaf { 
                            tree    : c[int_j].clone().tree,
                            position: c[int_j].clone().position,
                            history : c[int_j].clone().history,
                            entropy : c[int_j].clone().entropy 
                        });
                }
                x+=c.clone().len();
            }
        }
        for int_i in 0..vec_nodeleaf[f+1].len() {
            a += vec_nodeleaf[f+1][int_i].entropy;
        }
        f+=1;
    }
    for int_i in 0..vec_nodeleaf.len() {
        for int_j in 0..vec_nodeleaf[int_i].len(){
            println!("Tree Dataset f{} x{} :{:?}",int_i,int_j,vec_nodeleaf.clone()[int_i][int_j].tree.dataset);
            println!("Tree Num     f{} x{} :{:?}",int_i,int_j,vec_nodeleaf.clone()[int_i][int_j].tree.num);
            println!("Tree Class   f{} x{} :{:?}",int_i,int_j,vec_nodeleaf.clone()[int_i][int_j].tree.class);
            println!("Position     f{} x{} :{:?}",int_i,int_j,vec_nodeleaf.clone()[int_i][int_j].position);
            println!("History      f{} x{} :{:?}",int_i,int_j,vec_nodeleaf.clone()[int_i][int_j].history);
            println!("Entropy      f{} x{} :{:?}",int_i,int_j,vec_nodeleaf.clone()[int_i][int_j].entropy);
            println!();
        }
    }
}

