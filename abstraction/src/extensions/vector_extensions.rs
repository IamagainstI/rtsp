use std::cmp::PartialEq;

pub trait VecExt<T: PartialEq> {
    fn separate(&self, elem: T) -> Option<(&[T], &[T])>;
    fn separate_trimmed(&self, elem: T, trim: &T) -> Option<(&[T], &[T])>;
    fn trim(&self, trim: &T) -> &[T];
}

impl<T: PartialEq> VecExt<T> for [T] {
    /// Splits the slice into two slices at the first occurrence of `elem`.
    fn separate(&self, elem: T) -> Option<(&[T], &[T])> {
        match self.iter().position(|x| *x == elem) {
            Some(pos) => Some((&self[0..pos], &self[(pos + 1)..])),
            None => None,
        }
    }
    
    fn separate_trimmed(&self, elem: T, trim: &T) -> Option<(&[T], &[T])> {
        match self.separate(elem) {
            Some((first, second)) => Some((first.trim(trim), second.trim(trim))),
            None => None,
        }
    }
    
    fn trim(&self, trim: &T) -> &[T] {
        let start = self.iter()
            .position(|x| x != trim)
            .unwrap_or(0);

        let end = self.iter()
            .rposition(|x| x != trim)
            .map_or(0, |pos| pos + 1);

        &self[start..end]
    }
}