fn next_multiple(num: usize, modulo: usize, remain: usize) -> usize {
    let mut next: usize = (num / modulo) * modulo + remain;
    if next < num {
        next += modulo;
    }
    return next;
}