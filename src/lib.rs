// Use generics
pub fn unique<T: Ord>(mut v: Vec<T>) -> Vec<T> {
    v.sort();
    v.dedup();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let e: Vec<i64> = vec![];
        let u = unique(e);
        assert_eq!(u.len(), 0);
    }

    #[test]
    fn test_sorted() {
        let nums = vec![2, 3];
        let u = unique(nums);
        assert_eq!(u.len(), 2);
        assert_eq!(u[0], 2);
        assert_eq!(u[1], 3);
    }

    #[test]
    fn test_sorted_repeated() {
        let nums = vec![2, 2, 3];
        let u = unique(nums);
        assert_eq!(u.len(), 2);
        assert_eq!(u[0], 2);
        assert_eq!(u[1], 3);
    }

    #[test]
    fn test_unsorted() {
        let nums = vec![5, 2, 3];
        let u = unique(nums);
        assert_eq!(u.len(), 3);
        assert_eq!(u[0], 2);
        assert_eq!(u[1], 3);
        assert_eq!(u[2], 5);
    }

    #[test]
    fn test_unsorted_repeated() {
        let nums = vec![5, 2, 3, 5];
        let u = unique(nums);
        assert_eq!(u.len(), 3);
        assert_eq!(u[0], 2);
        assert_eq!(u[1], 3);
        assert_eq!(u[2], 5);
    }
}
