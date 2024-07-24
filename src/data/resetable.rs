

/// A trait for resetting the state of an iterator
pub trait Resetable : Iterator {
    fn reset(&mut self);
}