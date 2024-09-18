pub fn chop(n: isize, ns: Vec<isize>) -> Option<usize> {
    if ns.len() == 0 {
        return None;
    }
    chop_slice(n, &ns, 0, ns.len())
}

fn chop_slice(n: isize, ns: &Vec<isize>, lo: usize, hi: usize) -> Option<usize> {
    let mid = (lo + hi) / 2;

    if mid >= hi || mid < lo {
        return None;
    }

    let val = ns.get(mid).unwrap();

    if n == *val {
        return Some(mid);
    }

    if n < *val {
        return chop_slice(n, ns, lo, mid);
    } else {
        return chop_slice(n, ns, mid + 1, hi);
    }
}
