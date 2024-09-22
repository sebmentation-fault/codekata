use std::cmp::Ordering;

// the last attempt at doing this chop ...
// i've made efficient and readable code...
// lets do an abomination and have awful code
// how best to make awful code? using only copilot
pub fn chop(n: isize, ns: Vec<isize>) -> Option<usize> {
    let mut l = 0;
    let mut r = ns.len() as isize - 1;
    while l <= r {
        let m = (l + r) / 2;
        match n.cmp(&ns[m as usize]) {
            Ordering::Equal => return Some(m as usize),
            Ordering::Greater => l = m + 1,
            Ordering::Less => r = m - 1,
        }
    }
    None
}
