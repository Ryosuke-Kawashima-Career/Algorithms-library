use proconio::input;
const MOD: i64 = 998244353;
// abc358E
// Q. 長さ1..=kの文字列でそれぞれのアルファベットの使用数に上限があるときの場合の数
// A. 文字ごとに場所を決定する
// a~zをa^yに帰着させる．これを繰り返し，漸化式を立てる
// 小さいサイズに問題を帰着できたときには dp が有効なことが多い
fn main() {
    input!{k: usize, c: [usize; 26]}
    // dp[alphabet][length]
    let mut dp: Vec<Vec<ModInt>> = vec![vec![ModInt(0); k+1]; 26+1];
    dp[0][0] = ModInt(1);
    
    let (fact, _inv, fact_inv) = fact_mod(k);
    let comb = |n: usize, r: usize| -> ModInt {
        fact[n] * fact_inv[r] * fact_inv[n-r]
    };

    for alpha in 0..26 {
        // 全体の文字列の長さ
        for length in 0..=k {
            // alphaを使う数: letter
            for letter in 0..=(length.min(c[alpha])) {
                let prev = dp[alpha][length - letter].clone();
                // 場所を選ぶ
                dp[alpha+1][length] += prev * comb(length, letter);
            }
        }
    }

    let mut ans = ModInt(0);
    for length in 1..=k {
        ans += dp[26][length];
    }
    println!("{}", ans);
}

fn fact_mod(max_num: usize) -> (Vec<ModInt>, Vec<ModInt>, Vec<ModInt>) {
    let mut fact = vec![ModInt(1); max_num+1];
    let mut inv = vec![ModInt(1); max_num+1];
    let mut fact_inv = vec![ModInt(1); max_num+1];
    // init: fac[0] = 1; fac[1] = 1; inv[1] = 1; fact_inv[0] = 1; fact_inv[1] = 1;
    for n in 2..=max_num {
        fact[n] = fact[n-1] * ModInt::new(n as i64);
        inv[n] = -inv[MOD as usize % n] * ModInt::new(MOD / (n as i64));
        fact_inv[n] = fact_inv[n-1] * inv[n];
    }

    return (fact, inv, fact_inv);
}

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
impl std::ops::Neg for ModInt {
    type Output = Self;
    fn neg(self) -> Self { ModInt::new(0) - self }
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