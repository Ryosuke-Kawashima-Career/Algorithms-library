// 正負で分類する
// 分子: numerator, 分母: denominator
fn ceil_divide(num: i64, denom: i64) -> i64 {
    let res = if num >= 0 {
        if num % denom == 0 {
            num / denom
        } else {
            num / denom + 1
        }
    } else {
        num / denom
    };
    res
}
fn floor_divide(num: i64, denom: i64) -> i64 {
    let res = if num >= 0 {
        num / denom
    } else {
        if num % denom == 0 {
            num / denom
        } else {
            num / denom - 1
        }
    };
    res
}