//! This module defines a  `Slide` iterator over `Vector`s and slices

/// Immutable iterator that returns both an element, and slice
/// representing the remaining elements
///
/// This iterator will not return an empty slice upon reaching the
/// last element, but will instead return a `None` instead of a
/// `Some(&[..])`
/// # Example
/// ```
/// use argparse::slide::{Slide, Slider};
///
/// let v = vec![1, 2, 3, 4, 5];
///
/// for (x, opt_rest) in v.slide() {
///     if let Some(rest) = opt_rest {
///         println!("{}", x + rest[0]) // rest guaranteed at least 1 element
///     }
/// }
/// ```
pub struct Slide<'a, T: 'a> {
    v: &'a [T],
    pos: usize,
}

impl<'a, T: Sized> Iterator for Slide<'a, T> {
    type Item = (&'a T, Option<&'a [T]>);
    
    #[inline]
    fn next(&mut self) -> Option<(&'a T, Option<&'a [T]>)> {
        self.v.get(self.pos).map(|val| {
            self.pos = self.pos + 1;
            
            if self.v.len() > self.pos {
                (val, Some(&self.v[self.pos..]))
            } else {
                (val, None)
            }
        })
    }
    
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let diff = self.v.len() - self.pos;
        
        (diff, Some(diff))
    }
}

/// Interface for all types that can produce a `Slide` iterator
pub trait Slider<'a, T: Sized> {
    /// Calling this method shall produce a `Slide` iterator
    /// # Example
    /// ```
    /// use argparse::slide::{Slide, Slider};
    ///
    /// let v = vec![1, 2, 3, 4, 5];
    ///
    /// for (x, opt_rest) in v.slide() {
    ///     if let Some(rest) = opt_rest {
    ///         println!("{}", x + rest[0]) // rest guaranteed at least 1 element
    ///     }
    /// }
    /// ```
    fn slide(&'a self) -> Slide<'a, T>;
}

impl<'a, T> Slider<'a, T> for &'a [T] {
    fn slide(&'a self)  -> Slide<'a, T> {
        Slide { v: self, pos: 0}
    }
}

impl<'a, T> Slider<'a, T> for Vec<T> {
    fn slide(&'a self)  -> Slide<'a, T> {
        Slide { v: &self[..], pos: 0}
    }
}

#[cfg(test)]
mod test {
    use super::{Slider};
    
    #[test]
    fn test_zero() {
        let v: Vec<u8> = vec![];
        let mut it = v.slide();
        
        assert_eq!(it.next(), None);
    }
    
    #[test]
    fn test_one() {
        let v = vec![1];
        let mut it = v.slide();
        
        assert_eq!(it.next(), Some((&1, None)));
        assert_eq!(it.next(), None);
    }
    
    #[test]
    fn test_two() {
        let v = vec![1, 2];
        let mut it = v.slide();
        
        assert_eq!(it.next(), Some((&1, Some(&[2][..]))));
        assert_eq!(it.next(), Some((&2, None)));
        assert_eq!(it.next(), None);
    }
    
    #[test]
    fn test_ten() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut it = v.slide();
        
        assert_eq!(it.next(), Some((&1, Some(&[2, 3, 4, 5, 6, 7, 8, 9, 10][..]))));
        assert_eq!(it.next(), Some((&2, Some(&[3, 4, 5, 6, 7, 8, 9, 10][..]))));
        assert_eq!(it.next(), Some((&3, Some(&[4, 5, 6, 7, 8, 9, 10][..]))));
        assert_eq!(it.next(), Some((&4, Some(&[5, 6, 7, 8, 9, 10][..]))));
        assert_eq!(it.next(), Some((&5, Some(&[6, 7, 8, 9, 10][..]))));
        assert_eq!(it.next(), Some((&6, Some(&[7, 8, 9, 10][..]))));
        assert_eq!(it.next(), Some((&7, Some(&[8, 9, 10][..]))));
        assert_eq!(it.next(), Some((&8, Some(&[9, 10][..]))));
        assert_eq!(it.next(), Some((&9, Some(&[10][..]))));
        assert_eq!(it.next(), Some((&10, None)));
        assert_eq!(it.next(), None);
    }
}