use std::ops::Range;






/// Trait for a type that can be ranged
pub trait Rangeable {
    fn get_range(&self) -> Option<(Range<f32>, Range<f32>)>;

    fn add_point(&mut self, x : f32, y : f32);

}