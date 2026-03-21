use proconio::input;
const MOD: i64 = 1_000_000_007;
// educationalDp R
// ワーシャルフロイド法 
fn main() {
    input! {n: usize, k: usize, a: [[i64; n]; n]}
    // dot = mat1*mat2;
    // inv = mat.inv().unwrap();
    let ak = Matrix::new(&a).pow(k);
    let mut ans: ModInt = ModInt(0);

    for i in 0..n {
        for j in 0..n {
            ans +=ak.value[i][j];
        }
    }

    println!("{}", ans);
}

// matrix
#[derive(Clone)]
pub struct Matrix {
    value: Vec<Vec<ModInt>>,
    shape: (usize, usize),
}

impl Matrix {
    pub fn new(value: &Vec<Vec<i64>>) -> Matrix {
        let shape = (value.len(), value[0].len());
        let value = {
            let mut res = vec![vec![ModInt(0); shape.1]; shape.0];
            for i in 0..value.len() {
                for j in 0..value[0].len() {
                    res[i][j] = ModInt::new(value[i][j]);
                }
            }
            res
        };
        Matrix { value, shape }
    }

    pub fn eye(n: usize) -> Matrix {
        let mut res = vec![vec![ModInt(0); n]; n];
        for i in 0..n {
            res[i][i] = ModInt(1);
        }
        Matrix {
            value: res,
            shape: (n, n),
        }
    }
    
    pub fn zero(shape: (usize, usize)) -> Matrix {
        let res = vec![vec![ModInt(0); shape.1]; shape.0];
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
                if a.value[i1][i] != ModInt::new(0) {
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

    pub fn det(&self) -> ModInt {
        let mut cnt = 0;
        let mut u = self.clone();
        let n = u.value.len();
        for i in 0..n {
            if u.value[i][i].0 == 0 {
                for j in i + 1..n {
                    if u.value[j][i].0 != 0 {
                        unsafe {
                            (&mut u.value[i] as *mut Vec<ModInt>)
                                .swap(&mut u.value[j] as *mut Vec<ModInt>);
                        }
                        cnt += 1;
                        break;
                    }
                    if j == n - 1 {
                        return ModInt::new(0);
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
            .fold(ModInt::new(1), |x, y| x * y)
            * if cnt % 2 == 0 {
                ModInt::new(1)
            } else {
                ModInt::new(MOD - 1)
            }
    }
}

impl std::ops::Add for Matrix {
    type Output = Matrix;
    fn add(self, other: Self) -> Self {
        let a = &self.value;
        let b = &other.value;

        let mut c = vec![vec![ModInt(0); a[0].len()]; a.len()];

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

        let mut c = vec![vec![ModInt(0); a[0].len()]; a.len()];

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

        let mut c = vec![vec![ModInt(0); n]; l];

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

// modint
#[derive(Copy, Clone, Hash, Debug, PartialEq)]
pub struct ModInt(i64);

impl ModInt {
    fn new(x: i64) -> Self {
        let x = x % MOD;
        Self (if x < 0 { x + MOD } else { x })
    }
    fn pow(self, e: i64) -> Self {
        if e == 0 {
            Self::new(1)
        } else if e % 2 == 1 {
            self.pow(e - 1) * self
        } else {
            let y = self.pow(e / 2);
            y * y
        }
    }
    #[allow(dead_code)]
    fn inv(self) -> Self {
        self.pow(MOD - 2)
    }
    #[allow(dead_code)]
    fn factorial(n: i64) -> Vec<Self> {
        let mut ret = vec![0.into(); n as usize + 1];
        ret[0] = 1.into();
        for i in 1..n + 1 {
            ret[i as usize] = ret[i as usize - 1] * i;
        }
        ret
    }
    #[allow(dead_code)]
    fn fact(n: i64) -> Self {
        let factorials = Self::factorial(n);
        factorials[n as usize]
    }
    #[allow(dead_code)]
    fn perm(n: i64, r: i64) -> Self {
        Self::fact(n) * Self::fact(n-r).inv()
    }
    #[allow(dead_code)]
    fn comb(n: i64, r:i64) -> Self {
        Self::fact(n) * Self::fact(r).inv() * Self::fact(n-r).inv()
    }
    #[allow(dead_code)]
    fn inv_factorial(fact: &Vec<Self>) -> Vec<Self> {
        let n = fact.len() - 1;
        let mut ret = vec![0.into(); n + 1];
        ret[n] = fact[n].inv();
        for i in (0..n).rev() {
            ret[i] = ret[i + 1] * (i + 1) as i64;
        }
        ret
    }
}
impl<T: Into<ModInt>> std::ops::Add<T> for ModInt {
    type Output = Self;
    fn add(self, other: T) -> Self {
        let other = other.into();
        let sum = self.0 + other.0;
        Self(if sum >= MOD { sum - MOD } else { sum })
    }
}
impl<T: Into<ModInt>> std::ops::Sub<T> for ModInt {
    type Output = Self;
    fn sub(self, other: T) -> Self {
        let other = other.into();
        let sum = self.0 - other.0;
        Self(if sum < 0 { sum + MOD } else { sum })
    }
}
impl<T: Into<ModInt>> std::ops::Mul<T> for ModInt {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        let other = other.into();
        Self(self.0 * other.0 % MOD)
    }
}
impl<T: Into<ModInt>> std::ops::Div<T> for ModInt {
    type Output = Self;
    fn div(self, other: T) -> Self {
        let other = other.into();
        self * other.inv()
    }
}
impl From<i64> for ModInt {
    fn from(x: i64) -> Self {
        ModInt::new(x)
    }
}
impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl<T: Into<ModInt>> std::ops::AddAssign<T> for ModInt {
    fn add_assign(&mut self, other: T) {
        *self = *self + other.into();
    }
}
impl<T: Into<ModInt>> std::ops::SubAssign<T> for ModInt {
    fn sub_assign(&mut self, other: T) {
        *self = *self - other.into();
    }
}
impl<T: Into<ModInt>> std::ops::MulAssign<T> for ModInt {
    fn mul_assign(&mut self, other: T) {
        *self = *self * other.into();
    }
}
impl<T: Into<ModInt>> std::ops::DivAssign<T> for ModInt {
    fn div_assign(&mut self, other: T) {
        *self = *self / other.into();
    }
}