pub mod day01;

#[cfg(test)]
mod tests {
    use crate::day01;

    fn test_chop(chop_fn: fn(isize, Vec<isize>) -> Option<usize>) {
        assert_eq!(None, chop_fn(3, vec![]));
        assert_eq!(None, chop_fn(3, vec![1]));
        assert_eq!(Some(0), chop_fn(1, vec![1]));
        //
        assert_eq!(Some(0), chop_fn(1, vec![1, 3, 5]));
        assert_eq!(Some(1), chop_fn(3, vec![1, 3, 5]));
        assert_eq!(Some(2), chop_fn(5, vec![1, 3, 5]));
        assert_eq!(None, chop_fn(0, vec![1, 3, 5]));
        assert_eq!(None, chop_fn(2, vec![1, 3, 5]));
        assert_eq!(None, chop_fn(4, vec![1, 3, 5]));
        assert_eq!(None, chop_fn(6, vec![1, 3, 5]));
        //
        assert_eq!(Some(0), chop_fn(1, vec![1, 3, 5, 7]));
        assert_eq!(Some(1), chop_fn(3, vec![1, 3, 5, 7]));
        assert_eq!(Some(2), chop_fn(5, vec![1, 3, 5, 7]));
        assert_eq!(Some(3), chop_fn(7, vec![1, 3, 5, 7]));
        assert_eq!(None, chop_fn(0, vec![1, 3, 5, 7]));
        assert_eq!(None, chop_fn(2, vec![1, 3, 5, 7]));
        assert_eq!(None, chop_fn(4, vec![1, 3, 5, 7]));
        assert_eq!(None, chop_fn(6, vec![1, 3, 5, 7]));
        assert_eq!(None, chop_fn(8, vec![1, 3, 5, 7]));
    }

    #[test]
    fn test_day01() {
        test_chop(day01::chop);
    }
}
