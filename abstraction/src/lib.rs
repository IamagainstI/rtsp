pub mod parsing;
pub mod instancing;
pub mod extensions;
pub mod media;

#[cfg(test)]
mod tests {
    use super::VecExt;

    #[test]
    fn test_trim() {
        let vec = vec![0, 0, 1, 2, 3, 0, 4, 5, 0, 0];
        assert_eq!(vec.trim(0), &[1, 2, 3, 0, 4, 5]);

        let vec = vec![1, 2, 3];
        assert_eq!(vec.trim(0), &[1, 2, 3]);

        let vec = vec![0, 0, 0];
        assert_eq!(vec.trim(0), &[]);
    }

    #[test]
    fn test_separate() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(vec.separate(3), Some((&[1, 2][..], &[4, 5][..])));
        assert_eq!(vec.separate(6), None);
    }

    #[test]
    fn test_separate_trimmed() {
        let vec = vec![0, 1, 2, 3, 0, 4, 5, 0];
        assert_eq!(vec.separate_trimmed(3, 0), Some((&[1, 2][..], &[4, 5][..])));
        assert_eq!(vec.separate_trimmed(6, 0), None);
    }
}
