use proconio::{input, marker::Chars};
const MOD: i64 = 1_000_000_007;
// abc104D: 変数固定の問題はDPでも解ける場合がある(qubit dp)
// 変数がいくつかある時は、同時に考えるのではなく、あるものを固定して考えるとうまくいく
// ２変数あって全探索すると計算量がO(N^2)となる場合は、１つを固定して考えると、もう片方を工夫によって高速に求められ、計算量が落ちる
// 3つのものを考える時は、真ん中の物を固定すると上手くいく
// 4つのものは3つの仕切りがあると考えることもできるので、2つずつに分けて捉えるとうまくいくかも
// Q. 部分列A??Cの?をA,B,Cのいずれかに変えてできる部分列ABCの数
// A. Bの位置を全探索し，Aの場合の数×Cの場合の数
fn main() {
    input!{s: Chars}
    let n: usize = s.len();
    let mut ans = ModInt(0);
    let mut num_a_left: i64 = 0;
    // Aになるqubit(?)
    let mut num_q_left: i64 = 0;
    // Cになるqubit(?)
    let mut num_q_right: i64 = 0;
    let mut num_c_right: i64 = 0;

    // あらかじめ数字の数を計算する(クエリの先読みの応用)
    for i in 0..n {
        if s[i] == 'C' {
            num_c_right += 1;
        }
        if s[i] == '?' {
            num_q_right += 1;
        }
    }

    // num_rightを減らし，num_leftを増やす
    for i in 0..n {
        if s[i] == 'C' {
            num_c_right -= 1;
        }
        if s[i] == '?' {
            num_q_right -= 1;
        }
        // Bの位置の全探索
        if s[i] == 'B' || s[i] == '?' {
            // Aの数
            let mut num_left = ModInt::new(num_a_left) * (ModInt(3).pow(num_q_left));
            if num_q_left >= 1 {
                num_left += ModInt::new(num_q_left) * (ModInt(3).pow(num_q_left-1));
            }
            // Cの数
            let mut num_right = ModInt::new(num_c_right) * (ModInt(3).pow(num_q_right));
            if num_q_right >= 1 {
                num_right += ModInt::new(num_q_right) * (ModInt(3).pow(num_q_right-1));
            }
            ans += num_left * num_right;
        }
        // s[i] == 'B'の場合はここまで
        if s[i] == 'A' {
            num_a_left += 1;
        }
        if s[i] == '?' {
            num_q_left += 1;
        }
    }
    println!("{}", ans);
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