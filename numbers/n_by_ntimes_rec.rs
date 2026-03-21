use proconio::input;
use std::collections::HashMap;
const MOD: i128 = 998244353;
// abc357D
// Q. NをN個つなげた整数をVnとし，Vn(mod)を求める Ex. V3 = 333
// A. 3|33: 再帰関数で表現する = 3 * 10 ^ (2桁) + 33
// A. メモ化で再帰関数を高速化する
fn main() {
    input!{n: i128}
    let mut map = HashMap::new();
    map.insert(0, ModInt::new(0));
    map.insert(1, ModInt::new(n));
    let ans = solve(n, n, &mut map);
    println!("{}", ans);
}

// n: 書かれる数, pow: 繰り返しの数
fn solve(n: i128, pow: i128, map: &mut HashMap<i128, ModInt>) -> ModInt {
    let n_str = n.to_string();
    let digit: i128 = n_str.len() as i128;
    let mut res = ModInt::new(0);
    if pow == 0 {
        return res;
    }
    if pow == 1 {
        return ModInt::new(n);
    }

    if pow % 2 == 0 {
        let pow_half = pow / 2;
        let half = if map.contains_key(&pow_half) {
            map[&pow_half]
        } else {
            let res_half = solve(n, pow_half, map);
            map.insert(pow_half, res_half);
            res_half
        };
        let digit_half = digit * (pow_half);
        res += half + half * ModInt::new(10).pow(digit_half);
    } else {
        let pow1 = pow / 2;
        let pow2 = pow - pow / 2;
        let half1 = if map.contains_key(&pow1) {
            map[&pow1]
        } else {
            let res1 = solve(n, pow1, map);
            map.insert(pow1, res1);
            res1
        };
        let half2 = if map.contains_key(&pow2) {
            map[&pow2]
        } else {
            let res2 = solve(n, pow2, map);
            map.insert(pow2, res2);
            res2
        };
        let digit1 = digit * (pow1);
        let digit2 = digit * (pow2);
        res += half2 + half1 * ModInt::new(10).pow(digit2);
    }

    return res;
}

// 困ったときはi128をつかう, 定数にModInt(num), 変数にModInt::new(var)
#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
struct ModInt(i128);

impl ModInt {
    fn new(x: i128) -> Self {
        let x = x % MOD;
        Self (if x < 0 { x + MOD } else { x })
    }
    fn pow(self, e: i128) -> Self {
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
    fn factorial(n: i128) -> Vec<Self> {
        let mut ret = vec![0.into(); n as usize + 1];
        ret[0] = 1.into();
        for i in 1..n + 1 {
            ret[i as usize] = ret[i as usize - 1] * i;
        }
        ret
    }
    #[allow(dead_code)]
    fn fact(n: i128) -> Self {
        let factorials = Self::factorial(n);
        factorials[n as usize]
    }
    #[allow(dead_code)]
    fn perm(n: i128, r: i128) -> Self {
        Self::fact(n) * Self::fact(n-r).inv()
    }
    #[allow(dead_code)]
    fn comb(n: i128, r:i128) -> Self {
        Self::fact(n) * Self::fact(r).inv() * Self::fact(n-r).inv()
    }
    // n種類から重複ありでr個(1≦a1≦a2≦…≦ak≦nの選び方): n+r-1Cr
    // n個の玉とr-1の個の仕切りを並び替えることに等しい
    #[allow(dead_code)]
    fn comb_replace(n: i128, r: i128) -> Self {
        Self::comb(n+r-1, r)
    }
    #[allow(dead_code)]
    fn inv_factorial(fact: &Vec<Self>) -> Vec<Self> {
        let n = fact.len() - 1;
        let mut ret = vec![0.into(); n + 1];
        ret[n] = fact[n].inv();
        for i in (0..n).rev() {
            ret[i] = ret[i + 1] * (i + 1) as i128;
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
impl From<i128> for ModInt {
    fn from(x: i128) -> Self {
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