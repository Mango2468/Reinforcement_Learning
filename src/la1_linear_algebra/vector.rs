use std::fmt::{Display,Debug};
use std::ops::{Add,Sub,Mul,Div};
use std::string::String;
use crate::la1_linear_algebra::matrix::*;

#[allow(unused)]
pub fn vec_sum<T>(vec_a: Vec<T>,vec_b: Vec<T>) -> Result<Vec<T>,String>
    where T: PartialOrd + Display + Copy + Debug + Clone
    + std::ops::Add + Add<Output = T>
    // + std::ops::Sub + Sub<Output = T>
    // + std::ops::Mul + Mul<Output = T>
    // + std::ops::Div + Div<Output = T>
{
    if vec_a.len() == vec_b.len() {
        let mut vec_result : Vec<T> =vec![];
        for int_i in 0..vec_a.len(){
            vec_result.push(
                vec_a.clone()[int_i] + vec_b.clone()[int_i]
            );
        }
        Ok(vec_result)
    } else {
        Err("vector sizes are not matched each other".to_string())
    }
}


#[allow(unused)]
pub fn vec_subtract<T>(vec_a: Vec<T>,vec_b: Vec<T>) -> Result<Vec<T>,String>
    where T: PartialOrd + Display + Copy + Debug + Clone
    // + std::ops::Add + Add<Output = T>
    + std::ops::Sub + Sub<Output = T>
    // + std::ops::Mul + Mul<Output = T>
    // + std::ops::Div + Div<Output = T>
{
    if vec_a.len() == vec_b.len() {
        let mut vec_result : Vec<T> =vec![];
        for int_i in 0..vec_a.len(){
            vec_result.push(
                vec_a.clone()[int_i] - vec_b.clone()[int_i]
            );
        }
        Ok(vec_result)
    } else {
        Err("vector sizes are not matched each other".to_string())
    }
}


#[allow(unused)]
#[allow(unused_comparisons)]
pub fn vec_multiple<T>(vec_a: Vec<T>,b: T) -> Result<Vec<T>,String>
    where T: PartialOrd + Display + Copy + Debug + Clone
    // + std::ops::Add + Add<Output = T>
    // + std::ops::Sub + Sub<Output = T>
    + std::ops::Mul + Mul<Output = T>
    // + std::ops::Div + Div<Output = T>
{
    if vec_a.len() >= 0 {
    let mut vec_result : Vec<T> =vec![];
    for int_i in 0..vec_a.len(){
        vec_result.push(
            vec_a.clone()[int_i] * b
        );
    }
    Ok(vec_result)
    } else {
        Err("vector size can't be 0".to_string())
    }
}

#[allow(unused)]
pub fn vec_devide<T>(vec_a: Vec<T>,b: T) -> Result<Vec<T>,String>
    where T: PartialOrd + Display + Copy + Debug + Clone
    + std::ops::Add + Add<Output = T>
    + std::ops::Sub + Sub<Output = T>
    + std::ops::Mul + Mul<Output = T>
    + std::ops::Div + Div<Output = T>
{
    if b != zero(b) {
        let mut vec_result : Vec<T> =vec![];
        for int_i in 0..vec_a.len(){
            vec_result.push(
                vec_a.clone()[int_i]/b
            );
        }
        Ok(vec_result)
    } else {
        Err("devider can't be zero".to_string())
    }
}

#[allow(unused)]
pub fn vec_dot<T>(vec_a: Vec<T>,vec_b: Vec<T>) -> Result<Vec<T>,String>
    where T: PartialOrd + Display + Copy + Debug + Clone
    + std::ops::Add + Add<Output = T>
    + std::ops::Sub + Sub<Output = T>
    + std::ops::Mul + Mul<Output = T>
    + std::ops::Div + Div<Output = T>
{
    if vec_a.len() == vec_b.len() {
        let mut vec_result : Vec<T> = vec![];
        for int_i in 0..vec_a.len() {
            vec_result.push(vec_a.clone()[int_i]*vec_b.clone()[int_i]);
        }
        Ok(vec_result)
    } else {
        Err("vector sizes are not matched".to_string())
    }   
}

#[allow(unused)]
pub fn vec_cross<T>(vec_a: Vec<T>,vec_b: Vec<T>) -> Result<Vec<T>,String>
    where T: PartialOrd + Display + Copy + Debug + Clone
    + std::ops::Add + Add<Output = T>
    + std::ops::Sub + Sub<Output = T>
    + std::ops::Mul + Mul<Output = T>
    + std::ops::Div + Div<Output = T>
{
    if vec_a.len() == vec_b.len() {
        let mut vec_result : Vec<T> 
        = vec![
            vec_a.clone()[1]*vec_b.clone()[2]-vec_a.clone()[2]*vec_b.clone()[1],
            vec_a.clone()[2]*vec_b.clone()[0]-vec_a.clone()[0]*vec_b.clone()[2],
            vec_a.clone()[0]*vec_b.clone()[1]-vec_a.clone()[1]*vec_b.clone()[0]
            ];
        Ok(vec_result)
    } else {
        Err("vector sizes are not matched".to_string())
    }   
}