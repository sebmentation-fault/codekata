pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

#[cfg(test)]
mod tests {
    use crate::{day01, day02, day03, day04, day05};

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

    #[test]
    fn test_day02() {
        test_chop(day02::chop);
    }

    #[test]
    fn test_day03() {
        test_chop(day03::chop);
    }

    #[test]
    fn test_day04() {
        test_chop(day04::chop);
    }

    #[test]
    fn test_day05() {
        test_chop(day05::chop);
    }
}
