// today i will try iteration style of storing the slices
// becuase i'm being uncreative
// literally just trying to merge day 02 and day 03
pub fn chop(n: isize, ns: Vec<isize>) -> Option<usize> {
    let mut slice: &[isize] = &ns;
    let mut offset: usize = 0;

    while !slice.is_empty() {
        let mid = slice.len() / 2;

        if n == slice[mid] {
            return Some(mid + offset);
        }

        if n < slice[mid] {
            slice = &slice[..mid];
        } else {
            slice = &slice[mid + 1..];
            offset += mid + 1;
        }
    }

    None
}
