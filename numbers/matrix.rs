use proconio::input;

const MOD: i64 = 1_000_000_007;
// educationalDpR 
fn main() {
    input! {n: usize, k: usize, a: [[i64; n]; n]}
    // dot = mat1*mat2;
    // inv = mat.inv().unwrap();
    let ak = Matrix::new(a).pow(k);
    let mut ans: i64 = 0;

    for i in 0..n {
        for j in 0..n {
            ans +=ak.value[i][j];
        }
    }

    println!("{}", ans);
}

// matrix
// determinant not correct
#[derive(Clone)]
pub struct Matrix {
    value: Vec<Vec<i64>>,
    shape: (usize, usize),
}

impl Matrix {
    pub fn new(value: Vec<Vec<i64>>) -> Matrix {
        let shape = (value.len(), value[0].len());
        let value = {
            let mut res = vec![vec![0; shape.1]; shape.0];
            for i in 0..value.len() {
                for j in 0..value[0].len() {
                    res[i][j] = value[i][j];
                }
            }
            res
        };
        Matrix { value, shape }
    }

    pub fn eye(n: usize) -> Matrix {
        let mut res = vec![vec![0; n]; n];
        for i in 0..n {
            res[i][i] = 1;
        }
        Matrix {
            value: res,
            shape: (n, n),
        }
    }
    
    pub fn zero(shape: (usize, usize)) -> Matrix {
        let res = vec![vec![0; shape.1]; shape.0];
        Matrix { value: res, shape }
    }
    
    pub fn inv(mut a: Matrix) -> Option<Matrix> {
        assert_eq!(a.shape.0, a.shape.1);
        let n = a.shape.0;
        let mut b = Matrix::eye(n);
    
        for i in 0..n {
            for i1 in i.. {
                if i1 == n {
                    return None;
                }
                if a.value[i1][i] != 0 {
                    a.value.swap(i, i1);
                    b.value.swap(i, i1);
                    break;
                }
            }
            let temp = a.value[i][i];
            for j in 0..n {
                a.value[i][j] /= temp;
                b.value[i][j] /= temp;
            }
            for i1 in (0..n).filter(|i1| *i1 != i) {
                let temp = a.value[i1][i];
                for j in 0..n {
                    a.value[i1][j] = a.value[i1][j] - temp * a.value[i][j];
                    b.value[i1][j] = b.value[i1][j] - temp * b.value[i][j];
                }
            }
        }
    
        Some(b)
    }

    pub fn pow(&self, mut n: usize) -> Matrix {
        let mut res = Matrix::eye(self.value.len());
        let mut x = self.clone();
        while n > 0 {
            if n % 2 == 1 {
                res *= x.clone();
            }
            x *= x.clone();
            n /= 2;
        }
        res
    }

    pub fn det(&self) -> i64 {
        let mut cnt = 0;
        let mut u = self.clone();
        let n = u.value.len();
        for i in 0..n {
            if u.value[i][i] == 0 {
                for j in i + 1..n {
                    if u.value[j][i] != 0 {
                        unsafe {
                            (&mut u.value[i] as *mut Vec<i64>)
                                .swap(&mut u.value[j] as *mut Vec<i64>);
                        }
                        cnt += 1;
                        break;
                    }
                    if j == n - 1 {
                        return 0;
                    }
                }
            }

            for j in i + 1..n {
                let b = u.value[j][i] / u.value[i][i];
                for k in i..n {
                    u.value[j][k] = u.value[j][k] - b * u.value[i][k];
                }
            }
        }

        (0..n)
            .map(|i| u.value[i][i])
            .fold(1, |x, y| x * y)
            * if cnt % 2 == 0 {
                1
            } else {
                -1
            }
    }
}

impl std::ops::Add for Matrix {
    type Output = Matrix;
    fn add(self, other: Self) -> Self {
        let a = &self.value;
        let b = &other.value;

        let mut c = vec![vec![0; a[0].len()]; a.len()];

        for i in 0..a.len() {
            for j in 0..a[0].len() {
                c[i][j] = a[i][j] + b[i][j];
            }
        }

        Matrix {
            value: c,
            shape: self.shape,
        }
    }
}

impl std::ops::Sub for Matrix {
    type Output = Matrix;
    fn sub(self, other: Self) -> Self {
        let a = &self.value;
        let b = &other.value;

        let mut c = vec![vec![0; a[0].len()]; a.len()];

        for i in 0..a.len() {
            for j in 0..a[0].len() {
                c[i][j] = a[i][j] - b[i][j];
            }
        }

        Matrix {
            value: c,
            shape: self.shape,
        }
    }
}

impl std::ops::Mul for Matrix {
    type Output = Matrix;
    fn mul(self, other: Self) -> Self {
        assert_eq!(self.shape.1, other.shape.0);
        let a = &self.value;
        let b = &other.value;
        let l = a.len();
        let n = b[0].len();

        let mut c = vec![vec![0; n]; l];

        for (ci, ai) in c.iter_mut().zip(a.iter()) {
            for (aij, bj) in ai.iter().zip(b.iter()) {
                ci.iter_mut()
                    .zip(bj.iter())
                    .for_each(|(x, y)| *x += *aij * *y);
            }
        }

        Matrix {
            value: c,
            shape: (l, n),
        }
    }
}

impl std::ops::AddAssign for Matrix {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl std::ops::SubAssign for Matrix {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl std::ops::MulAssign for Matrix {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}