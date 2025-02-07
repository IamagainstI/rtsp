/// A trait that provides additional methods for arrays and slices.
pub trait ArrayExt<T: PartialEq> {
    /// Separates the slice into two parts at the first occurrence of the specified element.
    ///
    /// # Arguments
    ///
    /// * `elems` - The elements to separate the slice at.
    ///
    /// # Returns
    ///
    /// An `Option` containing a tuple of two slices. The first slice contains the elements
    /// before the specified element, and the second slice contains the elements after the
    /// specified element. If the element is not found, `None` is returned.
    fn separate<'a>(&'a self, elems: &'a [T]) -> Option<(&'a [T], &'a [T])>;


    /// Separates the slice into two parts at the first occurrence of the specified element,
    /// and trims the specified element from both resulting slices.
    ///
    /// # Arguments
    ///
    /// * `elems` - The elements to separate the slice at.
    /// * `trim` - The elements to trim from both resulting slices.
    ///
    /// # Returns
    ///
    /// An `Option` containing a tuple of two trimmed slices. The first slice contains the
    /// elements before the specified element, and the second slice contains the elements
    /// after the specified element. If the element is not found, `None` is returned.
    fn separate_trimmed<'a>(&'a self, elems: &'a [T], trim: &'a [T]) -> Option<(&'a [T], &'a [T])>;

    /// Trims the specified element from both ends of the slice.
    ///
    /// # Arguments
    ///
    /// * `trim` - The elements to trim from both ends of the slice.
    ///
    /// # Returns
    ///
    /// A slice with the specified element trimmed from both ends.
    fn trim(&self, trim: &[T]) -> &[T];
}

impl<T: PartialEq> ArrayExt<T> for [T] {
    fn separate<'a>(&'a self, elems: &'a [T]) -> Option<(&'a [T], &'a [T])> {
        separate_internal(&self, elems)
    }

    fn separate_trimmed<'a>(&'a self, elems: &'a [T], trim: &'a [T]) -> Option<(&'a [T], &'a [T])> {
        separate_internal(&self, elems).map(|(left, right)| (left.trim(trim), right.trim(trim)))
    }

    fn trim(&self, trim: &[T]) -> &[T] {
        let mut start = 0;
        let mut end = self.len() - 1;
        while start <= end && trim.contains(&self[start]) {
            start += 1;
        }
        while end > 0 && trim.contains(&self[end]) {
            end -= 1;
        }
        if start > end {
            return &[];
        }
        &self[start..end + 1]
    }
}

fn separate_internal<'a, T: PartialEq>(slice: &'a [T], elems: &[T]) -> Option<(&'a [T], &'a [T])> {
    let slice_len = slice.len();
    let elems_len = elems.len();
    if slice_len <= elems_len {
        return None;
    }
    let window = elems_len;
    let mut current: usize = 0;
    while current + window <= slice_len {
        if slice[current..current + window] == *elems {
            return Some((&slice[..current], &slice[current + window..]));
        }
        current += 1;
    }
    None
}