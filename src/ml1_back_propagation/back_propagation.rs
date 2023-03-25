use std::f64::consts::E;

fn main() {
    let m =   vec![vec![0.0,0.0]];
    let n =   vec![vec![0.0,0.0],vec![0.0,0.0]];
    println!("{:?}",mat_mult(m, n).expect(""));
    
}

#[allow(unused)]
struct BackProgation<F: Fn(f64) -> f64> {
    pub funct : F,
    pub input : Vec<Vec<f64>>,
    pub ouput : Vec<Vec<f64>>,
    pub result: Vec<Vec<f64>>,
    pub error : Vec<f64>,
    pub parameter : Vec<Vec<Vec<f64>>>,
}

#[allow(unused)]
fn f(x: f64) -> f64 {
    1.0 / (1.0 + E.powf(x*(-1.0)))
}

impl<F:Fn(f64) -> f64> BackProgation<F> {
    #[allow(unused)]
    fn f(self,x: f64) -> f64 {
        (self.funct)(x)
    }
    #[allow(unused)]
    fn perceptron(self) -> Result<BackProgation<F>,String>  {
        let vec_in = self.input.clone();
        let vec_result = self.result.clone();
        let vec_param = self.parameter.clone();
        let mut vec_out: Vec<Vec<f64>> = vec![];
        let mut e = 0;
        for int_j in 0..self.input.clone().len(){
            vec_out.push(vec![]);
            let mut a = vec![vec![]];
            a[0] = vec_in[int_j].clone();
            let mut vec_rs : Vec<Vec<f64>> = mat_trp(a.clone());
            for int_i in 0..self.parameter.clone().len() {
                match mat_mult(vec_param[int_i].clone(), vec_rs.clone()) {
                    Ok(b)=> vec_rs = b,
                    Err(_) => e +=1,
                }
            }
            let mut vec_rf : Vec<Vec<f64>> = vec_rs.clone();
            for int_i in 0..vec_rf.len() {
                for int_j in 0..vec_rf[0].len() {
                    vec_rf[int_i][int_j] = (self.funct)(vec_rf[int_i][int_j]) ;
                }
            }
            vec_out[int_j] = mat_trp(vec_rf)[0].to_vec();
        }
        let mut vec_error: Vec<f64> = vec![];
        if self.result.len() == self.ouput.len() && self.result[0].len() == self.ouput[0].len(){
            for int_i in 0..self.result.len(){
                vec_error.push(0.0);
                for int_j in 0..self.result[0].len() {
                    vec_error[int_i] += (vec_out[int_i][int_j]-vec_result[int_i][int_j]).powi(2)/2.0;
                }
            }
        } else {
            e +=1;
        }
        if e == 0 {
            return Ok(BackProgation { funct : self.funct, input: vec_in, ouput: vec_out , result: vec_result, error: vec_error, parameter: vec_param});
        } else {
            Err(format!("this code has {} errors",e).to_string())
        }
    }

    // fn back_prop(self,mu:Vec<f64>)-> Result<BackProgation<F>,String> {

    // }
}


//matrix multiple
//matrix_A:(aXb) X matrix_B:(bXc) => matrix_C:(aXc)
fn mat_mult(mat_a: Vec<Vec<f64>>, mat_b:Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>,String>  {
    if mat_a.clone()[0].len() != mat_b.clone().len() {
        Err("matrix size doesn't match".to_string())
    } else {
        let vec_a: Vec<Vec<f64>> = mat_a.clone();
        let vec_b: Vec<Vec<f64>> = mat_b.clone();
        let mut vec_c: Vec<Vec<f64>> = vec![];
        for int_i in 0..vec_a.len() {
            vec_c.push(vec![]);
            for _int_j in 0..vec_b[0].len() {
                vec_c[int_i].push(0.0);
            }
        }
        for int_i in 0..vec_c.len() {
            for int_j in 0..vec_c[0].len() {
                for int_k in 0..vec_a[0].len() {
                    vec_c[int_i][int_j] +=vec_a[int_i][int_k]*vec_b[int_k][int_j];
                }
            }
        }
        Ok(vec_c)
    }
}

//matrix transpose
//matrix_A:(aXb) => matrix_A_t:(bXa)
fn mat_trp(mat_a: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let vec_a = mat_a.clone();
    let mut vec_r : Vec<Vec<f64>> = vec![];
    for _int_i in 0..vec_a[0].len() {
        vec_r.push(vec![]);
        for _int_j in 0..vec_a.len() {
            vec_r[0].push(0.0);
        }
    }
    for int_i in 0..vec_a[0].len() {
        for int_j in 0..vec_a.len() {
            vec_r[int_i][int_j] = vec_a[int_j][int_i];
        }
    }
    return vec_r.clone();
}