use std::fmt::Display;

use number_filter::DisplayNumberFilter;
use serde::{Deserialize, Serialize};
use string_filter::DisplayStringFilter;

use super::sample::key::SerieKey;
use super::sample::Sample;

pub mod number_filter;
pub mod string_filter;

// ------------------------------------- Operator -------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

impl Operator {
    pub fn compare_string(&self, a: &str, b: &str) -> bool {
        match self {
            Operator::Equal => a == b,
            Operator::NotEqual => a != b,
            Operator::GreaterThan => a > b,
            Operator::GreaterThanOrEqual => a >= b,
            Operator::LessThan => a < b,
            Operator::LessThanOrEqual => a <= b,
        }
    }

    pub fn compare_number(&self, a: f32, b: f32) -> bool {
        match self {
            Operator::Equal => a == b,
            Operator::NotEqual => a != b,
            Operator::GreaterThan => a > b,
            Operator::GreaterThanOrEqual => a >= b,
            Operator::LessThan => a < b,
            Operator::LessThanOrEqual => a <= b,
        }
    }

    pub fn get_all() -> Vec<Operator> {
        vec![
            Operator::Equal,
            Operator::NotEqual,
            Operator::GreaterThan,
            Operator::GreaterThanOrEqual,
            Operator::LessThan,
            Operator::LessThanOrEqual,
        ]
    }
}


impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Equal => write!(f, "=="),
            Operator::NotEqual => write!(f, "!="),
            Operator::GreaterThan => write!(f, ">"),
            Operator::GreaterThanOrEqual => write!(f, ">="),
            Operator::LessThan => write!(f, "<"),
            Operator::LessThanOrEqual => write!(f, "<="),
        }
    }
}

// ------------------------------------- FilterGraph -------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Filter<K>
where
    K: SerieKey,
{
    NumberFilter(DisplayNumberFilter<K>),
    StringFilter(DisplayStringFilter<K>),
}

impl<K> Filter<K>
where
    K: SerieKey,
{

    pub fn new_number(key: K, operator: Operator, value: f32) -> Self {
        Filter::NumberFilter(DisplayNumberFilter::new(key, operator, value))
    }

    pub fn new_string(key: K, operator: Operator, value: String) -> Self {
        Filter::StringFilter(DisplayStringFilter::new(key, operator, value))
    }

    pub fn apply<S>(&self, sample: &S) -> bool
    where
        S: Sample<K>,
    {
        match self {
            Filter::NumberFilter(filter) => filter.apply(sample),
            Filter::StringFilter(filter) => filter.apply(sample),
        }
    }

    pub fn get_key(&self) -> K {
        match self {
            Filter::NumberFilter(filter) => filter.get_key(),
            Filter::StringFilter(filter) => filter.get_key(),
        }
    }


}

impl<K> Into<Filter<K>> for DisplayNumberFilter<K> 
where
    K: SerieKey,
{
    fn into(self) -> Filter<K> {
        Filter::NumberFilter(self)
    }
}

impl<K> Into<Filter<K>> for DisplayStringFilter<K> 
where
    K: SerieKey,
{
    fn into(self) -> Filter<K> {
        Filter::StringFilter(self)
    }
}

// -------------------------------------- Filters -------------------------------------
#[derive(Debug, Clone, PartialEq)]
pub struct Filters<K>
where
    K: SerieKey,
{
    filters: Vec<Filter<K>>,
}



impl <K> Filters<K>
where
    K: SerieKey,
{

    pub fn new(filters : Vec<Filter<K>>) -> Self {
        Self { filters }
    }

    pub fn empty() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    pub fn add_filter(&mut self, filter: Filter<K>) {
        self.filters.push(filter);
    }

    pub fn remove_filter(&mut self, n : usize) {
        if n < self.filters.len() {
            self.filters.remove(n);
        } else {
            panic!("Index out of bounds");
        }
    }

    pub fn apply<S>(&self, sample: &S) -> bool
    where
        S: Sample<K>,
    {
        self.filters.iter().all(|f| f.apply(sample))
    }
}

impl<K> Default for Filters<K>
where
    K: SerieKey,
{
    fn default() -> Self {
        Self {
            filters: Vec::new(),
        }
    }
}

impl<K> From<Filter<K>> for Filters<K>
where
    K: SerieKey,
{
    fn from(filter: Filter<K>) -> Self {
        Self {
            filters: vec![filter],
        }
    }
}

impl<K> From<&Vec<Filter<K>>> for Filters<K>
where
    K: SerieKey,
{
    fn from(filters: &Vec<Filter<K>>) -> Self {
        Self {
            filters : filters.clone(),
        }
    }
}

impl<Key> From<Vec<Filter<Key>>> for Filters<Key>
    where 
        Key : SerieKey
{
    fn from(filters : Vec<Filter<Key>>) -> Self {
        Self {
            filters,
        }
    }
}