pub fn chop(n: isize, ns: Vec<isize>) -> Option<usize> {
    if ns.is_empty() {
        return None;
    }
    chop_slice(n, &ns)
}

fn chop_slice(n: isize, slice: &[isize]) -> Option<usize> {
    if slice.is_empty() {
        return None;
    }

    let mid = slice.len() / 2;

    if n == slice[mid] {
        return Some(mid);
    }

    if n < slice[mid] {
        chop_slice(n, &slice[..mid])
    } else {
        chop_slice(n, &slice[mid + 1..]).map(|i| i + mid + 1)
    }
}
