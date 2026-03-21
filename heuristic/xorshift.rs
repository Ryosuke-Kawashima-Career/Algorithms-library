// min..max
#[allow(dead_code)]
fn range(rng_num: u32, min_num: usize, max_num: usize) -> usize {
    min_num + rng_num as usize % (max_num - min_num)
}
fn rangei(rng_num: u32, min_num: i64, max_num: i64) -> i64 {
    min_num + rng_num as i64 % (max_num - min_num)
}
#[allow(dead_code)]
fn rangef(rng_num: u32, min_num: usize, max_num: usize) -> f64 {
    // rng: 0..1
    let rng = (rng_num as f64) / (u32::MAX as f64);
    min_num as f64 + (rng * ((max_num - min_num) as f64))
}

/*
    let mut xor = xorshift();
    // index range: 1..n
    let mut idx: usize = range(xor(), 1, n);
    // rangef: 0..1
    if adoption_rate > rangef(xor(), 0, 1) {}
*/
#[allow(dead_code)]
fn xorshift() -> Box<dyn FnMut() -> u32> {
    let mut y = 2463534242;
    Box::new(move || {
        y = y ^ (y << 13);
        y = y ^ (y >> 17);
        y = y ^ (y << 5);
        y
    })
}

/*
    let mut xor = Xorshift::new(1);
    // index range: 1..n
    let mut idx: usize = xor.range(1, n);
    // rangef: 0..1
    if adoption_rate > xor.rangef(0, 1) {}
*/
#[allow(dead_code)]
pub struct Xorshift {
    state: u32,
}

impl Xorshift {
    pub fn new(seed: u32) -> Xorshift {
        Xorshift { state: seed }
    }

    pub fn next(&mut self) -> u32 {
        let mut x: u32 = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        self.state
    }

    pub fn range(&mut self, min_num: usize, max_num: usize) -> usize {
        min_num + self.next() as usize % (max_num - min_num)
    }

    pub fn rangei(&mut self, min_num: i64, max_num: i64) -> i64 {
        min_num + self.next() as i64 % (max_num - min_num)
    }

    pub fn rangef(&mut self, min_num: f64, max_num: f64) -> f64 {
        // rng: 0..1
        let rng = (self.next() as f64) / (u32::MAX as f64);
        min_num + (rng * (max_num - min_num))
    }
}
