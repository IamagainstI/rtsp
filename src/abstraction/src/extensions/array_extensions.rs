/// A trait that provides additional methods for arrays and slices.
pub trait ArrayExt<T: PartialEq> {
    /// Separates the slice into two parts at the first occurrence of the specified element.
    ///
    /// # Arguments
    ///
    /// * `elem` - The element to separate the slice at.
    ///
    /// # Returns
    ///
    /// An `Option` containing a tuple of two slices. The first slice contains the elements
    /// before the specified element, and the second slice contains the elements after the
    /// specified element. If the element is not found, `None` is returned.
    fn separate(&self, elem: T) -> Option<(&[T], &[T])>;

    /// Separates the slice into two parts at the first occurrence of the specified element,
    /// and trims the specified element from both resulting slices.
    ///
    /// # Arguments
    ///
    /// * `elem` - The element to separate the slice at.
    /// * `trim` - The element to trim from both resulting slices.
    ///
    /// # Returns
    ///
    /// An `Option` containing a tuple of two trimmed slices. The first slice contains the
    /// elements before the specified element, and the second slice contains the elements
    /// after the specified element. If the element is not found, `None` is returned.
    fn separate_trimmed(&self, elem: T, trim: &T) -> Option<(&[T], &[T])>;

    /// Trims the specified element from both ends of the slice.
    ///
    /// # Arguments
    ///
    /// * `trim` - The element to trim from both ends of the slice.
    ///
    /// # Returns
    ///
    /// A slice with the specified element trimmed from both ends.
    fn trim(&self, trim: &T) -> &[T];
}

impl<T: PartialEq> ArrayExt<T> for [T] {
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