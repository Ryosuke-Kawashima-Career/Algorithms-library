use proconio::input;
// abc428D
fn main() {
    input!{t: usize}
    for _case in 0..t {
        input!{c: usize, d: usize}
        // 1 <= x <= d %% f(c, c + x) is a square number -> How many Xs are there?
        let ans = calc_each_digit(c, d);
        println!("{}", ans);
    }
}

// when C + x has D digits
fn calc_each_digit(c: usize, d: usize) -> usize {
    let mut res: usize = 0;
    let mut xmin: usize = 1;
    let mut xmax: usize = 9;
    let mut exp10: usize = 10;
    // when the digits of x are 1, 2, ... 
    while xmin <= c + d {
        let mut xl: usize = xmin.max(c + 1);
        let mut xr: usize = xmax.min(c + d);
        if (xl <= xr) {
            let value_l: usize = c * exp10 + xl;
            let value_r: usize = c * exp10 + xr;
            res += floor_sqrt(value_r) - floor_sqrt(value_l - 1);
        }

        xmin *= 10;
        xmax = (xmax+1) * 10 - 1;
        exp10 *= 10;
    }
    return res;
}

fn floor_sqrt(x: usize) -> usize {
    let mut y: usize = (x as f64).sqrt() as usize;
    // making sure y^2 is bigger than x
    while y * y > x {
        y -= 1;
    }
    // making y as large as possible
    while (y + 1) * (y + 1) <= x {
        y += 1;
    }
    return y;
}