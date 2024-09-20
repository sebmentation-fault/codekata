// yesterday i used recursion, so today lets try doing this iteratively
pub fn chop(n: isize, ns: Vec<isize>) -> Option<usize> {
    if ns.is_empty() {
        return None;
    }

    let mut lo: usize = 0;
    let mut hi: usize = ns.len() - 1;
    let mut mid: usize = ns.len() / 2;
    let mut mid_val: isize = *ns.get(mid).expect("mid not in array?!");

    while n != mid_val {
        if lo == hi {
            if n != *ns.get(lo).expect("lo hi too big/small?") {
                return None;
            } else {
                return Some(lo);
            }
        }

        if n > mid_val {
            lo = mid + 1;
        } else {
            hi = mid;
        }
        mid = (lo + hi) / 2;
        mid_val = *ns.get(mid).expect("mid no there ??");
    }

    Some(mid)
}
