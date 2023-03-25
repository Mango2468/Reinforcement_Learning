/* every matrix is rectangular matrix */

use std::fmt::{Display,Debug};
use std::ops::{Add,Sub,Mul,Div};
use std::string::String;
use crate::la1_linear_algebra::vector::*;

/// 두 행렬의 덧셈을 구하는 함수입니다.
/// 
/// 행렬은 모두 Vec<Vec<[T]>> 형태로 나타내어야 합니다.
/// 
/// 행렬안에 들어갈 수 있는 값은 정수형태와 소수 형태 모두 가능합니다.
/// 
/// ([T] 의 조건에 해당하는 타입을 이용해하고 같은 타입이어야 값을 구합니다)
/// 
/// 각 행렬의 행의 갯수와 열의 갯수는 모두 같습니다.
/// 
/// 만약 각 행렬의 행의 갯수와 열의 갯수가 다른 경우, 에러가 발생합니다.
/// 
/// # Examples
///
/// ```
///
/// use la::matrix::matrix_sum;
/// 
/// let vec_a: Vec<Vec<f64>> = vec![
///     vec![1.0,0.0,0.0,0.0],
///     vec![0.0,1.0,0.0,0.0],
///     vec![0.0,0.0,1.0,0.0],
///     vec![0.0,0.0,0.0,1.0]
///     ];
/// ```
/// 위의 행렬의 경우 4x4 행렬입니다.
/// 
/// 행렬의 크기를 알고 싶은 경우, [len](https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html#method.len)을 사용하면 됩니다.
/// 
/// ```
/// let row_size_matrix_a: usize = vec_a.len();
/// let column_size_matrix_a: usize = vec_a[0].len();
/// ```
/// ```
/// let vec_b: Vec<Vec<f64>> = vec![
///     vec![3.0,2.0,1.0,0.0],
///     vec![2.0,3.0,2.0,1.0],
///     vec![1.0,2.0,3.0,2.0],
///     vec![0.0,1.0,2.0,3.0]
///     ];
/// 
/// ```
/// [matrix_sum] 함수를 이용할 때, 각 행렬의 크기를 확인하셔야합니다.
/// 
/// ```
/// let row_size_matrix_b: usize = vec_b.len();
/// let column_size_matrix_b: usize = vec_b[0].len();
/// assert_eq!(row_size_matrix_a,row_size_matrix_b);
/// assert_eq!(column_size_matrix_a,column_size_matrix_b);
/// 
/// ```
/// [matrix_sum] 함수의 결과를 확인해봅시다.
/// 
/// ```
/// 
/// let vec_c: Vec<Vec<f64>> = vec![
///     vec![4.0,2.0,1.0,0.0],
///     vec![2.0,4.0,2.0,1.0],
///     vec![0.0,0.0,1.0,0.0],
///     vec![0.0,0.0,0.0,1.0]
///     ];
/// 
/// assert_eq!(vec_c,matrix_sum(vec_a,vec_b).unwrap());
/// 
/// 
/// ```
#[allow(unused)]
pub fn matrix_sum<T>(matrix_a: Vec<Vec<T>>,matrix_b: Vec<Vec<T>>) -> Result<Vec<Vec<T>>,String> 
    where T: PartialOrd + Display + Copy + Debug + Clone + std::ops::Add + Add<Output = T>
{
    if matrix_a.len() == matrix_b.len() && matrix_a[0].len() == matrix_b[0].len() {
        let mut vec_result : Vec<Vec<T>> = vec![];
        for int_i in 0..matrix_a.len() {
            vec_result.push(vec![]);
            for int_j in 0..matrix_a[0].len() {
                vec_result[int_i].push(matrix_a[int_i][int_j] + matrix_b[int_i][int_j]);
            }
        }
        Ok(vec_result)
    } else {
        Err("matrix's sizes can't be diffent to each other".to_string())
    } 
}

 
#[allow(unused)]
pub fn matrix_scalar_multiple<T>(matrix_a: Vec<Vec<T>>,scalar: T) -> Result<Vec<Vec<T>>,String> 
    where T: PartialOrd + Display + Copy + Debug + Clone + std::ops::Mul + Mul<Output = T>
{
    if matrix_a.len() != 0 {
        let mut vec_result : Vec<Vec<T>> = vec![];
        for int_i in 0..matrix_a.len() {
            vec_result.push(vec![]);
            for int_j in 0..matrix_a[0].len() {
                vec_result[int_i].push(matrix_a[int_i][int_j] * scalar);
            }
        }
        Ok(vec_result)
    }else {
        Err("matrix size can't be 0".to_string())
    }
}

#[allow(unused)]
pub fn matrix_subtract<T>(matrix_a: Vec<Vec<T>>, matrix_b: Vec<Vec<T>>) -> Result<Vec<Vec<T>>,String>
    where T: PartialOrd + Display + Copy + Debug + Clone + std::ops::Sub + Sub<Output = T>
{
    if matrix_a.len() == matrix_b.len() && matrix_a[0].len() == matrix_b[0].len() {
        let mut vec_result : Vec<Vec<T>> = vec![];
        for int_i in 0..matrix_a.len() {
            vec_result.push(vec![]);
            for int_j in 0..matrix_a[0].len() {
                vec_result[int_i].push(matrix_a[int_i][int_j] - matrix_b[int_i][int_j]);
            }
        }
        Ok(vec_result)
    } else {
        Err("matrix's sizes can't be diffent to each other".to_string())
    } 
}

#[allow(unused)]
pub fn matrix_multiple<T>(matrix_a: Vec<Vec<T>>, matrix_b: Vec<Vec<T>>) -> Result<Vec<Vec<T>>,String>
    where T: PartialOrd + Display + Copy + Debug + Clone + std::ops::Add + Add<Output = T> + std::ops::Mul + Mul<Output = T>
{
    if matrix_a[0].len() == matrix_b.len() && matrix_b.len() > 1{
        let mut vec_result : Vec<Vec<T>> = vec![];
        for int_i in 0..matrix_a.len() {
            vec_result.push(vec![]);
            for int_j in 0..matrix_b[0].len(){
                vec_result[int_i].push(matrix_a[int_i][0]*matrix_b[0][int_j]);
                for int_k in 1..matrix_b.len(){
                    vec_result[int_i][int_j] = vec_result[int_i][int_j]+ matrix_a[int_i][int_k]*matrix_b[int_k][int_j];
                }
            }
        }
        Ok(vec_result)
    } else {
        Err("matrix's sizes can't be diffent to each other".to_string())
    } 
}

#[allow(unused)]
pub fn permutation(a:usize) ->Vec<(Vec<usize>,isize)>{
    let mut vec_basic: Vec<usize> = vec![];
    let mut vec_start: Vec<Vec<usize>> = vec![] ;
    let mut vec_result: Vec<(Vec<usize>,isize)> = vec![] ;
    for int_i in 0..a{
        vec_basic.push(int_i);
    }
    for int_i in 0..factorial(a){
        vec_start.push(vec_basic.clone());
        vec_result.push((vec![],(-1_isize).pow((int_i as u32 +1)/2)));
    }
    for int_i in (0..a).rev(){
        for int_j in 0..factorial(a){
            vec_result[int_j].0.push(vec_start[int_j][(int_j/factorial(int_i))%(int_i+1)]);
            vec_start[int_j].remove((int_j/factorial(int_i))%(int_i+1));
        }
    }
    return vec_result;
}

#[allow(unused)]
pub fn factorial(a: usize) -> usize {
    let mut result: usize = 1;
    if a == 0{
        return result;
    } else {
        for int_i in 1..=a{
            result = result *int_i;
        }
        return result;
    }
}

#[allow(unused)]
pub fn sign(a:usize, total: usize) -> isize {
    permutation(total)[a].1
}

#[allow(unused)]
pub fn determinant<T>(matrix_a: Vec<Vec<T>>) -> Result<T,String>
    where T: PartialOrd + Display + Copy + Debug + Clone
    + std::ops::Add + Add<Output = T>
    + std::ops::Sub + Sub<Output = T>
    + std::ops::Mul + Mul<Output = T>
    + std::ops::Div + Div<Output = T>
{
    if matrix_a[0].len() == matrix_a.len() && matrix_a.len() > 1{
        let permutate: Vec<(Vec<usize>,isize)> = permutation(matrix_a.len());
        let mut sum: T = matrix_a[0][permutate[0].0[0]];
        for int_i in 1..matrix_a.len(){
            sum = sum * matrix_a[int_i][permutate[0].0[int_i]];
        }
        for int_j in 1..permutate.len(){
            let mut multiple_i : T = matrix_a[0][permutate[int_j].0[0]];
            for int_i in 1..matrix_a.len(){
                multiple_i = multiple_i * matrix_a[int_i][permutate[int_j].0[int_i]];
            }
            if permutate[int_j].1 == 1{
                sum = sum + multiple_i;
            } else {
                sum = sum - multiple_i;
            }
        }
        Ok(sum)
    } else {
        Err("row size and column size doesn't matched".to_string())
    }
}

#[allow(unused)]
pub fn zero<T>(a:T)->T
    where T: PartialOrd + Display + Copy + Debug + Clone
    + std::ops::Add + Add<Output = T>
    + std::ops::Sub + Sub<Output = T>
    + std::ops::Mul + Mul<Output = T>
    + std::ops::Div + Div<Output = T>
{
    a-a
}

#[allow(unused)]
pub fn plus<T>(a: T)->Result<T,String>
    where T: PartialOrd + Display + Copy + Debug + Clone
    + std::ops::Add + Add<Output = T>
    + std::ops::Sub + Sub<Output = T>
    + std::ops::Mul + Mul<Output = T>
    + std::ops::Div + Div<Output = T>
{
    if a != zero(a){
        Ok(a/a)
    } else {
        Err("this vector is not suitable to calculate this function".to_string())
    }
}

#[allow(unused)]
pub fn minus<T>(a: T)->Result<T,String>
    where T: PartialOrd + Display + Copy + Debug + Clone
    + std::ops::Add + Add<Output = T>
    + std::ops::Sub + Sub<Output = T>
    + std::ops::Mul + Mul<Output = T>
    + std::ops::Div + Div<Output = T>
{
    if a != zero(a){
        Ok((zero(a)-a)/a)
    } else {
        Err("this vector is not suitable to calculate this function".to_string())
    }
}

#[allow(unused)]
pub fn minor_determinant<T>(matrix_a: Vec<Vec<T>>,row: usize, col:usize) -> Result<T,String>
    where T: PartialOrd + Display + Copy + Debug + Clone
    + std::ops::Add + Add<Output = T>
    + std::ops::Sub + Sub<Output = T>
    + std::ops::Mul + Mul<Output = T>
    + std::ops::Div + Div<Output = T>
{
    if matrix_a[0].len() == matrix_a.len() && matrix_a.len() > 1 && matrix_a.len() > row && matrix_a.len() > col{
        let mut vec_result: Vec<Vec<T>> = matrix_a.clone();
        for int_i in 0..vec_result.len(){
            vec_result[int_i].remove(col);
        }
        vec_result.remove(row);
        Ok(determinant(vec_result.clone()).unwrap())
    } else {
        Err("row size and column size doesn't matched".to_string())
    }
}


#[allow(unused)]
pub fn adjugate_matrix<T>(matrix_a: Vec<Vec<T>>,p:T) -> Result<Vec<Vec<T>>,String>
    where T: PartialOrd + Display + Copy + Debug + Clone
    + std::ops::Add + Add<Output = T>
    + std::ops::Sub + Sub<Output = T>
    + std::ops::Mul + Mul<Output = T>
    + std::ops::Div + Div<Output = T>
{
    if matrix_a[0].len() == matrix_a.len() && matrix_a.len() > 1 && p!= zero(p){
        let mut vec_result : Vec<Vec<T>> = vec![];
        for int_i in 0..matrix_a.len(){
            vec_result.push(vec![]);
            for int_j in 0..matrix_a.len(){
                vec_result[int_i].push(zero(p));
                if (int_i+int_j)%2 ==0 {
                    vec_result[int_i][int_j] = vec_result[int_i][int_j] + minor_determinant(matrix_a.clone(), int_j, int_i).unwrap();
                } else {
                    vec_result[int_i][int_j] = vec_result[int_i][int_j] - minor_determinant(matrix_a.clone(), int_j, int_i).unwrap();
                }
            }
        }
        Ok(vec_result)
    } else {
        Err("row size and column size doesn't matched".to_string())
    }
}

#[allow(unused)]
pub fn inverse_matrix<T>(matrix_a: Vec<Vec<T>>,p:T) -> Result<Vec<Vec<T>>,String>
    where T: PartialOrd + Display + Copy + Debug + Clone
    + std::ops::Add + Add<Output = T>
    + std::ops::Sub + Sub<Output = T>
    + std::ops::Mul + Mul<Output = T>
    + std::ops::Div + Div<Output = T>
{
    if matrix_a[0].len() == matrix_a.len() && matrix_a.len() > 1 
    && determinant(matrix_a.clone()) !=  Ok(zero(p)) {
        matrix_scalar_multiple(adjugate_matrix(matrix_a.clone(), p).unwrap(), plus(p).unwrap()/determinant(matrix_a.clone()).unwrap())
    } else {
        Err("matrix doesn't have an inverse matrix".to_string())
    }
}

#[allow(unused)]
pub fn gauss_jordan_elimination<T>(matrix_a: Vec<Vec<T>>,matrix_b: Vec<Vec<T>>) -> Result<Vec<Vec<T>>,String>
    where T: PartialOrd + Display + Copy + Debug + Clone
    + std::ops::Add + Add<Output = T>
    + std::ops::Sub + Sub<Output = T>
    + std::ops::Mul + Mul<Output = T>
    + std::ops::Div + Div<Output = T>
{
    if matrix_a.len() == matrix_b.len() {
        //첨가 행렬로 표현
        let mut vec_result : Vec<Vec<T>> = vec![];
        for int_i in 0..matrix_a.len() {
            let mut vec_inter: Vec<T> = matrix_a[int_i].clone();
            for int_j in 0..matrix_b[0].len() {
                vec_inter.push(matrix_b.clone()[int_i][int_j]);
            }
            vec_result.push(vec_inter.clone());
        }
        let p : T = matrix_a[0][0];

        //순행 
        for int_i in 0..matrix_a[0].len(){
            //int_i열만 검색 시작
            let mut int_row: usize = int_i;
            for int_j in int_i..vec_result.len() {
                if vec_result[int_j][int_i] != zero(p) {
                    break;
                }
                int_row = int_j;
            }
            //int_row행에서는 선행 1이 될 수 있게 만들고, 다른 행에서는 행 빼기
            vec_result[int_row] = vec_devide(vec_result[int_row].clone(),vec_result.clone()[int_row][int_i]).unwrap();

            for int_j in int_row+1..vec_result.len() {
                vec_result[int_j] = vec_subtract(vec_result[int_j].clone(), vec_multiple(vec_result[int_row].clone(), vec_result.clone()[int_j][int_i]).unwrap() ).unwrap();
            }
        }

        // 역행
        for int_i in (0..matrix_a[0].len()).rev() {
            //int_i열만 검색 시작
            let mut int_row: usize = int_i;
            for int_j in (0..=int_i).rev() {
                if vec_result[int_j][int_i] != zero(p) {
                    break;
                }
                int_row = int_j;
            }
            //int_row행에서는 선행 1이 될 수 있게 만들고, 다른 행에서는 행 빼기
            for int_j in (0..int_row).rev() {
                vec_result[int_j] = vec_subtract(vec_result[int_j].clone(), vec_multiple(vec_result[int_row].clone(), vec_result.clone()[int_j][int_i]).unwrap() ).unwrap();
            }
        }
        return Ok(vec_result);
    } else {
        Err("matrix size aren't matched".to_string())
    }
}

#[allow(unused)]
pub fn transpose_matrix<T>(matrix_a: Vec<Vec<T>>) -> Vec<Vec<T>>
    where T: PartialOrd + Display + Copy + Debug + Clone
    + std::ops::Add + Add<Output = T>
    + std::ops::Sub + Sub<Output = T>
    + std::ops::Mul + Mul<Output = T>
    + std::ops::Div + Div<Output = T>
 {
    let mut vec_result: Vec<Vec<T>> = vec![];
    for int_i in 0..matrix_a[0].len(){
        vec_result.push(vec![]);
        for int_j in 0..matrix_a.len(){
            vec_result[int_i].push(matrix_a.clone()[int_j][int_i]);
        }
    }
    return vec_result.to_vec();
 }

 #[allow(unused)]
 pub fn matrix_vector_multiple<T>(matrix_a: Vec<Vec<T>>, vector_b: Vec<T>) -> Result<Vec<Vec<T>>,String>
    where T: PartialOrd + Display + Copy + Debug + Clone
    + std::ops::Add + Add<Output = T>
    + std::ops::Sub + Sub<Output = T>
    + std::ops::Mul + Mul<Output = T>
    + std::ops::Div + Div<Output = T>
{
    let matrix_b : Vec<Vec<T>> = transpose_matrix(vec![vector_b.clone()]);
    matrix_multiple(matrix_a.clone(), matrix_b.clone())
}

#[allow(unused)]
pub fn identity_matrix<T>(size:usize, p:T ) -> Result<Vec<Vec<T>>,String>
    where T: PartialOrd + Display + Copy + Debug + Clone
    + std::ops::Add + Add<Output = T>
    + std::ops::Sub + Sub<Output = T>
    + std::ops::Mul + Mul<Output = T>
    + std::ops::Div + Div<Output = T>
{
    if p != zero(p){
        let mut vec_result: Vec<Vec<T>> = vec![];
        for int_i in 0..size {
            vec_result.push(vec![]);
            for int_j in 0..size {
                if int_i == int_j {
                    vec_result[int_i].push(plus(p).unwrap());
                } else {
                    vec_result[int_i].push(zero(p));
                }
                
            }
        }
        Ok(vec_result)
    } else {
        Err("p can't be a zero".to_string())
    }
}

///행렬을 분리할 때 사용하는 함수입니다.
/// 
/// [matrix_a]에 분리하고 싶은 행렬을 넣습니다.
/// 
/// 행을 기준으로 분리하고 싶다면 [row_or_column]을 [true](https://doc.rust-lang.org/std/keyword.true.html)로 입력합니다.
/// 
/// 열을 기준으로 분리하고 시다면 [row_or_column]을 [false](https://doc.rust-lang.org/std/keyword.false.html)로 입력합니다.
/// 
/// 행을 기준으로 분리하는 경우 [matrix_a]의 행의 수가 [size]보다 커야합니다.
/// 
/// 열을 기준으로 분리하는 경우 [matrix_a]의 열의 수가 [size]보다 커야합니다.
#[allow(unused)]
pub fn matrix_seperate<T>(matrix_a: Vec<Vec<T>>,row_or_column: bool,size:usize) -> Result<(Vec<Vec<T>>,Vec<Vec<T>>),String>
    where T: PartialOrd + Display + Copy + Debug + Clone
    + std::ops::Add + Add<Output = T>
    + std::ops::Sub + Sub<Output = T>
    + std::ops::Mul + Mul<Output = T>
    + std::ops::Div + Div<Output = T>
{
    if row_or_column == true {
        //seperate into row 
        if size < matrix_a.len() {
            let mut vec_result1: Vec<Vec<T>> = vec![];
            let mut vec_result2: Vec<Vec<T>> = vec![];
            for int_i in 0..size{
                vec_result1.push(vec![]);
                for int_j in 0..matrix_a[0].len(){
                    vec_result1[int_i].push(matrix_a.clone()[int_i][int_j]);
                }
            }
            for int_i in size..matrix_a.len(){
                vec_result2.push(vec![]);
                for int_j in 0..matrix_a[0].len(){
                    vec_result2[int_i-size].push(matrix_a.clone()[int_i][int_j]);
                }
            }
            Ok((vec_result1,vec_result2))

        } else {
            Err("size can't be over origin matrix's size".to_string())
        }
    } else {
        //seperate into column
        if size < matrix_a[0].len() {
            let mut vec_result1: Vec<Vec<T>> = vec![];
            let mut vec_result2: Vec<Vec<T>> = vec![];
            for int_i in 0..matrix_a.len(){
                vec_result1.push(vec![]);
                vec_result2.push(vec![]);
                for int_j in 0..size{
                    vec_result1[int_i].push(matrix_a.clone()[int_i][int_j]);
                }
                for int_j in size..matrix_a[0].len(){
                    vec_result2[int_i].push(matrix_a.clone()[int_i][int_j]);
                }
            }
            Ok((vec_result1,vec_result2))
        } else {
            Err("size can't be over origin matrix's size".to_string())
        }
    }
}