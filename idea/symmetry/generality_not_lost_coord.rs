use proconio::input;
const MOD: i64 = 1_000_000_007;
// HHKB2020D: xyを入れ替えても，一般性は失われない!!!
// 座標の対称性: x座標とy座標の対称性から1次元の問題に帰着
// 正方形が登場する問題は、x座標とy座標に対称性があるので、それぞれ独立した1次元の問題として考えることができる
// Q. 白い正方形NxNの中で赤い正方形AxAと青い正方形BxBの内部が重ならないように置く方法の数
// A. 余事象を考える: 赤い正方形と青い正方形が重なる方法の数を数えて全体から引く
fn main() {
    input!{query: usize}
    for _ in 0..query {
        // a < b としても一般性は失われない!!!
        input!{n: i64, mut a: i64, mut b: i64}
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        if n < a + b {
            println!("0");
            continue;
        }
        // 1つの軸を計算する
        // 1つの正方形の置き方
        let cases_a = ModInt::new(n - a + 1);
        let cases_b = ModInt::new(n - b + 1);
        // 2つの軸を考える
        let total = cases_a.pow(2) * cases_b.pow(2);

        // 条件をに満たさない余事象: 赤と青が重なる
        // (2つの正方形がかぶっている事象の数) = (数直線[0,N]の範囲に長さAの棒と長さBの棒をかぶって配置する事象の数)^2
        // 2つの正方形がかぶっている方法の数は、縦と横で独立に考えることができる
        let compl_inner = ModInt::new(n - b + 1) * ModInt::new(b - a + 1);
        let compl_outer = ModInt::new(a - 2) * ModInt::new(a - 1) / 2 + ModInt::new(a - 1) * ModInt::new(n - a - b + 2);
        let compl = (compl_inner + compl_outer * 2).pow(2);
        let ans = total - compl;
        println!("{}", ans);
    }
}

// 困ったときはi128をつかう, 定数にModInt(num), 変数にModInt::new(var)
#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
struct ModInt(i64);

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
    // n種類から重複ありでr個(1≦a1≦a2≦…≦ak≦nの選び方): n+r-1Cr
    // n個の玉とr-1の個の仕切りを並び替えることに等しい
    #[allow(dead_code)]
    fn comb_replace(n: i64, r: i64) -> Self {
        Self::comb(n+r-1, r)
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