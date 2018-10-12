use std::fmt::Display;

/// A newtype for a slice.
struct Slice<'a, T: 'a>(&'a [T]);

// -------------------------------------------------------------------------
// non-owning iteration
// -------------------------------------------------------------------------

/// A non-owning iterator for 'Slice'.
struct Iter<'a, T: 'a> {
    inner: &'a Slice<'a, T>,
    position: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.position += 1;
        if self.position <= self.inner.0.len() {
            Some(&self.inner.0[self.position - 1])
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a Slice<'a, T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Iter<'a, T> {
        Iter {
            inner: self,
            position: 0,
        }
    }
}

// -------------------------------------------------------------------------
// owning iteration
// -------------------------------------------------------------------------

/// An owning iterator for 'Slice'.
struct IntoIter<'a, T: 'a> {
    inner: Slice<'a, T>,
    position: usize,
}

impl<'a, T> Iterator for IntoIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.position += 1;
        if self.position <= self.inner.0.len() {
            Some(&self.inner.0[self.position - 1])
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for Slice<'a, T> {
    type Item = &'a T;
    type IntoIter = IntoIter<'a, T>;
    fn into_iter(self) -> IntoIter<'a, T> {
        IntoIter {
            inner: self,
            position: 0,
        }
    }
}

// -------------------------------------------------------------------------
// main
// -------------------------------------------------------------------------

fn run_iter<I: Display>(iter: impl Iterator<Item = I>) {
    for item in iter {
        println!("Pulled {} out of iterator", item)
    }
}

fn main() {
    let data = [1, 2, 3];
    let mt = Slice(&data);

    // run_iter(mt.iter());
    // run_iter(&mt.iter());
    run_iter((&mt).into_iter());
    run_iter(mt.into_iter());
}
