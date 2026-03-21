use proconio::input;
const MOD: i64 = 998244353;
// abc353D: 主客転倒テクニック
// f(3, 14) = 314 := f(x, y) = 10^len(y)*x + y
// Q. Σi=0..n-1 Σj=i+1..n (f(Ai, Aj))
// A. 1. f(*,Ai)という形でのAiの寄与 2. f(Ai,∗) という形でのAiの寄与
// 1. (i-1) * Ai 
// 2. i<j かつ len(Aj)=k なるjの個数をdkとすると、寄与は∑k=1..=10 (dk*10^k*Ai)
fn main() {
    input!{n: usize, a: [i64; n]}
    let mut cnt_len: Vec<i64> = vec![0; 11];
    // 条件を満たすi<jのjの数え方
    // 1. あらかじめすべて計算する
    // 2. 左からfor文を回し，cntをリセットしていく
    for i in 0..n {
        cnt_len[a[i].to_string().len()] += 1;
    }
    let mut power10: Vec<ModInt> = vec![ModInt(1); 11];
    
    for i in 1..=10 {
        power10[i] = power10[i-1] * 10;
    }

    let mut ans = ModInt(0);
    for i in 0..n {
        // 1.の計算
        ans += ModInt::new(i as i64) * ModInt::new(a[i]);

        // a[i]を消去する ->  i<j かつ len(Aj)=k なるjの個数をdkを計算する
        cnt_len[a[i].to_string().len()] -= 1;
        // 2.の計算
        for j in 1..=10 {
            ans += power10[j] * ModInt::new(a[i]) * ModInt::new(cnt_len[j]);
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