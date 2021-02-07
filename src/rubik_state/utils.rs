// binomial coefficient [n choose k]
pub fn c_nk(n: usize, mut k: usize) -> usize { // FIXME, pas compris
    if n < k {
        return 0;
    }
    if k > (n as f64 / 2.0).floor() as usize {
        k = n - k
    }
    let mut res: usize = 1;
    let mut i: usize = n;
    let mut j: usize = 1;
    while i != n - k {
        res *= i;
        res = (res as f64 / j as f64).floor() as usize;
        i -= 1;
        j += 1;
    }
    return res;
}