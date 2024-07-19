use std::fmt::Debug;
use std::ops::Add;

use super::sample::key::SerieKey;
use super::sample::Sample;



/// Define a filter
pub struct Filter<Key> 
where 
    Key : SerieKey
{
    key : Key,
    filter_number : Option<Box<dyn Fn(f32) -> bool>>,
    filter_str : Option<Box<dyn Fn(&str) -> bool>>,
}

impl<Key> Debug for Filter<Key>
where 
    Key : SerieKey
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let filter_type = if self.key.is_numeric() {
            "numeric"
        } else {
            "string"
        };
        write!(f, "Filter({} : {})", self.key.get_display_name(), filter_type)
    }
}

impl<Key> Filter<Key>
where 
    Key : SerieKey
{
    pub fn new_number<F>(key : Key, filter_fn : F) -> Self
    where 
        F : Fn(f32) -> bool + 'static
    {
        assert!(key.is_numeric());
        Self {
            key,
            filter_number : Some(Box::new(filter_fn)),
            filter_str : None,
        }
    }

    pub fn new_str<F>(key : Key, filter_fn : F) -> Self
    where 
        F : Fn(&str) -> bool + 'static
    {
        assert!(key.is_string());
        Self {
            key,
            filter_number : None,
            filter_str : Some(Box::new(filter_fn)),
        }
    }

    pub fn get_key(&self) -> &Key {
        &self.key
    }

    pub fn get_filter_number(&self) -> &Box<dyn Fn(f32) -> bool> {
        self.filter_number.as_ref().unwrap()
    }

    pub fn get_filter_str(&self) -> &Box<dyn Fn(&str) -> bool> {
        self.filter_str.as_ref().unwrap()
    }

    /// Combine two filters
    pub fn combine(mut self, other : Self) -> Self {
        if self.key != other.key {
            panic!("Cannot combine filters with different keys");
        }

        self.filter_number = match (self.filter_number, other.filter_number) {
            (Some(f1), Some(f2)) => 
                Some(Box::new(move |x| f1(x) && f2(x))),
            (Some(_), None) => panic!("Cannot combine a numeric filter with a string filter"),
            (None, Some(_)) => panic!("Cannot combine a numeric filter with a string filter"),
            (None, None) => None,
        };
       
        self.filter_str = match (self.filter_str, other.filter_str) {
            (Some(f1), Some(f2)) => 
                Some(Box::new(move |x| f1(x) && f2(x))),
            (Some(_), None) => panic!("Cannot combine a numeric filter with a string filter"),
            (None, Some(_)) => panic!("Cannot combine a numeric filter with a string filter"),
            (None, None) => None,
        };

        self
    }

    /// Apply the filter to a sample
    pub fn apply<S>(&self, sample : &S) -> bool
    where
        S : Sample<Key>
    {
        if self.key.is_numeric() {
            let value = sample.get_numeric_value(&self.key);
            self.filter_number.as_ref().unwrap()(value)
        } else {
            let value = sample.get_string_value(&self.key);
            self.filter_str.as_ref().unwrap()(value.as_str())
        }
    }
}

impl<Key> Add for Filter<Key>
where
    Key : SerieKey
{
    type Output = Self;

    fn add(self, other : Self) -> Self {
        self.combine(other)
    }
}