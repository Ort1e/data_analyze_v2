use std::fmt::Debug;
use std::ops::{Add, AddAssign};

use super::sample::key::SerieKey;
use super::sample::Sample;


/// Define a filter for a serie (with multiple keys)
pub struct Filters<Key>
where 
    Key : SerieKey
{
    filters : Vec<Filter<Key>>,
}

impl<Key> Default for Filters<Key>
where 
    Key : SerieKey
{
    fn default() -> Self {
        Self::empty()
    }
}

impl<Key> Debug for Filters<Key>
where 
    Key : SerieKey
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Filters({} filters)", self.filters.len())
    }
}

impl<Key> From<Vec<Filter<Key>>> for Filters<Key>
where 
    Key : SerieKey
{
    fn from(filters : Vec<Filter<Key>>) -> Self {
        Self::new(filters)
    }
}

impl<Key> Filters<Key>
where 
    Key : SerieKey
{
    pub fn empty() -> Self {
        Self {
            filters : Vec::new(),
        }
    }

    pub fn new(filters : Vec<Filter<Key>>) -> Self {
        let mut self_ = Self::empty();
        for filter in filters {
            self_.add_filter(filter);
        }

        self_
    }

    pub fn add_filter(&mut self, filter : Filter<Key>) {
        for f in self.filters.iter_mut() {
            if f.get_key() == filter.get_key() {
                *f += filter;
                return;
            }
        }
        
        self.filters.push(filter);
        
    }

    pub fn combine_ref(&mut self, other : Self) -> &Self {
        for filter in other.filters {
            self.add_filter(filter);
        }
        self
    }

    pub fn combine(mut self, other : Self) -> Self {
        self.combine_ref(other);
        self
    }

    pub fn apply<S>(&self, sample : &S) -> bool
    where
        S : Sample<Key>
    {
        self.filters.iter().all(|f| f.apply(sample))
    }
}


impl<Key> AddAssign for Filters<Key>
where
    Key : SerieKey
{
    fn add_assign(&mut self, other : Self) {
        self.combine_ref(other);
    }
}

impl<Key> Add for Filters<Key>
where
    Key : SerieKey
{
    type Output = Self;

    fn add(self, other : Self) -> Self {
        self.combine(other)
    }
}



// -----------------------------------------------------------------------------


/// Define a filter for a particular key
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
    /// Create a new filter for number with an identity function (all values are accepted)
    pub fn new_number_identity(key : Key) -> Self {
        if !key.is_numeric() {
            panic!("Cannot create a numeric filter with a string key");
        }
        Self {
            key,
            filter_number : Some(Box::new(|_| true)),
            filter_str : None,
        }
    }

    /// Create a new filter for str with an identity function (all values are accepted)
    pub fn new_str_identity(key : Key) -> Self {
        if !key.is_string() {
            panic!("Cannot create a string filter with a numeric key");
        }
        Self {
            key,
            filter_number : None,
            filter_str : Some(Box::new(|_| true)),
        }
    }

    pub fn new_number<F>(key : Key, filter_fn : F) -> Self
    where 
        F : Fn(f32) -> bool + 'static
    {
        if !key.is_numeric() {
            panic!("Cannot create a numeric filter with a string key");
        }
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
        if !key.is_string() {
            panic!("Cannot create a string filter with a numeric key");
        }
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

    pub fn combine_ref(&mut self, other : Self) -> &Self {
        if self.key != other.key {
            panic!("Cannot combine filters with different keys");
        }

        self.filter_number = match (self.filter_number.take(), other.filter_number) {
            (Some(f1), Some(f2)) => 
                Some(Box::new(move |x| f1(x) && f2(x))),
            (Some(_), None) => panic!("Cannot combine a numeric filter with a string filter"),
            (None, Some(_)) => panic!("Cannot combine a numeric filter with a string filter"),
            (None, None) => None,
        };

        self.filter_str = match (self.filter_str.take(), other.filter_str) {
            (Some(f1), Some(f2)) => 
                Some(Box::new(move |x| f1(x) && f2(x))),
            (Some(_), None) => panic!("Cannot combine a numeric filter with a string filter"),
            (None, Some(_)) => panic!("Cannot combine a numeric filter with a string filter"),
            (None, None) => None,
        };

        self
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

impl<Key> AddAssign for Filter<Key>
where
    Key : SerieKey
{
    fn add_assign(&mut self, other : Self) {
        self.combine_ref(other);
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