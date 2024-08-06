use std::collections::HashMap;
use std::ops::Range;
use crate::stat::stats_serie::StatsSerie;

use super::filtered_serie::{FilteredSerie, FilteredSerieIterator};
use super::filtering::Filters;
use super::rangeable::Rangeable;
use super::resetable::Resetable;
use super::sample::file_sample::FileSample;
use super::sample::key::SerieKey;
use super::sample::Sample;
use super::sample_serie::file_sample_serie::FileSampleSerieIterator;
use super::sample_serie::SampleSerie;




type Point = (f32, f32);

/// Define a plottable serie (legend associated with points)
#[derive(Debug, Clone)]
pub struct FilePlottableSerie<S, K>
where
    S : FileSample<K>,
    K : SerieKey
{
    paths : Vec<String>,
    _key : std::marker::PhantomData<K>,
    _sample : std::marker::PhantomData<S>,
}


impl<S, K> FilePlottableSerie<S, K>
where
    S : FileSample<K>,
    K : SerieKey,
{
    pub fn new(paths : Vec<String>) -> Self {
        FilePlottableSerie {
            paths,
            _key : std::marker::PhantomData,
            _sample : std::marker::PhantomData,
        }
    }

    pub fn into_iter_with_filter<'a>(&'a self, serie_keys : (K, K), legend_key : Option<K>, filters : &'a Filters<K>) 
        -> PlottableSerieIterator<S, K, FilteredSerieIterator<S, K, FileSampleSerieIterator<S, K>>>
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

    pub fn into_sample_iter(&self) -> FileSampleSerieIterator<S, K> {
        SampleSerie::new(self.paths.clone()).into_iter()
    }

    /// Collect statistics for multiple series sorted by a the uniquee value of a specified key.
    /// This function is optimized for speed but not for memory (O(n)).
    /// Warning: Avoid calling this function multiple times with different metrics as it may be slow.
    pub fn collect_stats_sorted_by_unique_values(
        &self, 
        stats_serie_keys: &Vec<K>, 
        sort_value_key: &K
    ) -> HashMap<String, HashMap<K, StatsSerie>> {
        let mut serie_by_sort: HashMap<String, HashMap<K, Vec<f32>>> = HashMap::new();

        // Iterate through the sample iterator
        let iter = self.into_sample_iter();
        for sample in iter {
            // Determine the sort value for this sample
            let sort_value = if sort_value_key.is_numeric() {
                sample.get_numeric_value(sort_value_key).to_string()
            } else {
                sample.get_string_value(sort_value_key)
            };

            // Obtain the hashmap corresponding to the sort value
            let sort_entry = serie_by_sort.entry(sort_value).or_insert_with(HashMap::new);

            // Iterate over each key in stats_serie_keys
            for key in stats_serie_keys {
                if !key.is_numeric() {
                    panic!("stats serie key must be numeric");
                }

                // Get the numeric value for the current key
                let key_value = sample.get_numeric_value(key);
                // Collect the values in a vector
                sort_entry.entry(*key).or_insert_with(Vec::new).push(key_value);
            }
        }

        // Transform buffered vectors into StatsSerie objects
        serie_by_sort.into_iter().map(|(sort_key, key_values_map)| {
            let stats_map = key_values_map.into_iter().map(|(key, values)| {
                let stats_serie = StatsSerie::new(&values);
                (key, stats_serie)
            }).collect::<HashMap<K, StatsSerie>>();
            (sort_key, stats_map)
        }).collect()
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