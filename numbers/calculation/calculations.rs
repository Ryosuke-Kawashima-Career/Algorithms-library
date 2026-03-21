fn power(base: usize, pow: usize) -> usize {
    // corner case
    if pow == 0 {
        return 1;
    }
    if pow == 1 {
        return base;
    }
    // recurrence formula
    if pow % 2 == 0 {
        return power(base*base, pow/2);
    } else {
        return base * power((base * base), (pow-1)/2);
    }
}

fn digit_sum(mut num: usize) -> usize {
    let mut res: usize = 0;
    while num > 0 {
        res += num % 10;
        num /= 10;
    }

    return res;
}

fn bit_count(mut num: usize) -> usize {
    let mut res: usize = 0;
    while num > 0 {
        res += num % 2;
        num /= 2;
    }

    return res;
}

// how many times a number can be divided by 2
fn count2(mut num: usize) -> usize {
    let mut res: usize = 0;
    while num % 2 == 0 && num > 0 {
        res += 1;
        num /= 2;
    }

    return res;
}

// 約数の組み合わせ
fn calc_divs(n: usize) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    for num in 1..=n {
        let mut i: usize = 1;
        while i * i <= num {
            if num % i == 0 {
                let j: usize = num / i;
                res.push((i, j));
            }
            i += 1;
        }
    }

    return res;
}