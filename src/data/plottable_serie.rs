use std::ops::Range;
use super::filtered_serie::{FilteredSerie, FilteredSerieIterator};
use super::filtering::Filters;
use super::rangeable::Rangeable;
use super::resetable::Resetable;
use super::sample::key::SerieKey;
use super::sample::Sample;
use super::sample_serie::{SampleSerie, SampleSerieIterator};




type Point = (f32, f32);

/// Define a plottable serie (legend associated with points)
#[derive(Debug, Clone)]
pub struct PlottableSerie<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    paths : Vec<String>,
    _key : std::marker::PhantomData<K>,
    _sample : std::marker::PhantomData<S>,
}


impl<S, K> PlottableSerie<S, K>
where
    S : Sample<K>,
    K : SerieKey,
{
    pub fn new(paths : Vec<String>) -> Self {
        PlottableSerie {
            paths,
            _key : std::marker::PhantomData,
            _sample : std::marker::PhantomData,
        }
    }

    pub fn into_iter_with_filter<'a>(&'a self, serie_keys : (K, K), legend_key : Option<K>, filters : &'a Filters<K>) 
        -> PlottableSerieIterator<S, K, FilteredSerieIterator<S, K, SampleSerieIterator<S, K>>>
    {
        if let Some(legend_key) = legend_key.as_ref() {
            if legend_key.is_numeric() {
                panic!("legend_key must be a string key");
            }
        }
        if !serie_keys.0.is_numeric() {
            panic!("x_key must be a numeric key");
        }
        if !serie_keys.1.is_numeric() {
            panic!("y_key must be a numeric key");
        }
        
        
        let sample_serie = SampleSerie::new(self.paths.clone());
        let filtered_serie = FilteredSerie::new(sample_serie.into_iter(), filters);
        PlottableSerieIterator::new(filtered_serie.into_iter(), serie_keys, legend_key)
    }
}


// -----------------------------------------------------------------------------

/// An iterator over a plottable serie
/// Note: the iterator is not sorted
/// Note: the iterator return a tuple (legend, points) with points as a vector of (x, y) points corresponding to the series_keys in order
#[derive(Debug, Clone)]
pub struct PlottableSerieIterator<S, K, It>
where
    S : Sample<K>,
    K : SerieKey,
    It : Iterator<Item = S>
{
    iterator : It,
    serie_keys : (K, K),
    legend_key : Option<K>,
    x_min : Option<f32>,
    x_max : Option<f32>,
    y_min : Option<f32>,
    y_max : Option<f32>,
}

impl<S, K, It> PlottableSerieIterator<S, K, It>
where
    S : Sample<K>,
    K : SerieKey,
    It : Iterator<Item = S>
{
    pub fn new(iterator : It, serie_keys : (K, K), legend_key : Option<K>) -> Self {
        if let Some(legend_key) = legend_key.as_ref() {
            if legend_key.is_numeric() {
                panic!("legend_key must be a string key");
            }
        }
        
        if !serie_keys.0.is_numeric() {
            panic!("x_key must be a numeric key");
        }

        if !serie_keys.1.is_numeric() {
            panic!("y_key must be a numeric key");
        }
        
        PlottableSerieIterator {
            iterator,
            serie_keys,
            legend_key,
            x_min : None,
            x_max : None,
            y_min : None,
            y_max : None,
        }
    }

    pub fn get_serie_keys(&self) -> (K, K) {
        self.serie_keys
    }

    pub fn get_legend_key(&self) -> &Option<K> {
        &self.legend_key
    }
}

impl<S, K, It> Rangeable for PlottableSerieIterator<S, K, It>
where
    S : Sample<K>,
    K : SerieKey,
    It : Iterator<Item = S>
{
    fn get_range(&self) -> Option<(Range<f32>, Range<f32>)> {
        match (self.x_min, self.x_max, self.y_min, self.y_max) {
            (Some(x_min), Some(x_max), Some(y_min), Some(y_max)) => {
                Some((x_min..x_max, y_min..y_max))
            },
            (None, None, None, None) => None,
            _ => panic!("Incomplete range"),
        }
    }

    fn add_point(&mut self, x : f32, y : f32) {
        match (self.x_min, self.x_max, self.y_min, self.y_max) {
            (Some(x_min), Some(x_max), Some(y_min), Some(y_max)) => {
                self.x_min = Some(x_min.min(x));
                self.x_max = Some(x_max.max(x));
                self.y_min = Some(y_min.min(y));
                self.y_max = Some(y_max.max(y));
            },
            (None, None, None, None) => {
                self.x_min = Some(x);
                self.x_max = Some(x);
                self.y_min = Some(y);
                self.y_max = Some(y);
            },
            _ => panic!("Incomplete range"),
        }
    }
}

impl<S, K, It> Iterator for PlottableSerieIterator<S, K, It>
where
    S : Sample<K>,
    K : SerieKey,
    It : Iterator<Item = S>
{
    type Item = (String, Point);

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(|sample| {
            let x = sample.get_numeric_value(&self.serie_keys.0);
            let y = sample.get_numeric_value(&self.serie_keys.1);
            
            self.add_point(x, y);
            

            let legend = if let Some(legend_key) = self.legend_key.as_ref() {
                sample.get_string_value(legend_key)
            } else {
                "All".to_string()
            };
            (legend, (x, y))
        })
    }
}

impl<S, K, It> Resetable for PlottableSerieIterator<S, K, It>
where
    S : Sample<K>,
    K : SerieKey,
    It : Iterator<Item = S> + Resetable
{
    fn reset(&mut self) {
        self.iterator.reset();
        self.x_min = None;
        self.x_max = None;
        self.y_min = None;
        self.y_max = None;
    }
}