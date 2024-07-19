use super::sample::key::SerieKey;
use super::sample::Sample;
use super::sample_serie::{SampleSerie, SampleSerieIterator};




type Point = (f32, f32);

/// Define a plottable serie (legend associated with points)
#[derive(Debug, Clone)]
pub struct PlottableSeries<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    paths : Vec<String>,
    x_key : K,
    y_key : K,
    legend_key : K,
    _sample : std::marker::PhantomData<S>,
}


impl<S, K> PlottableSeries<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    pub fn new(paths : Vec<String>, x_key : K, y_key : K, legend_key : K) -> Self {
        if legend_key.is_numeric() {
            panic!("legend_key must be a string key");
        }
        if !x_key.is_numeric() {
            panic!("x_key must be a numeric key");
        }
        if !y_key.is_numeric() {
            panic!("y_key must be a numeric key");
        }
        PlottableSeries {
            paths,
            x_key,
            y_key,
            legend_key,
            _sample : std::marker::PhantomData,
        }
    }

    pub fn get_x_key(&self) -> &K {
        &self.x_key
    }

    pub fn get_y_key(&self) -> &K {
        &self.y_key
    }

    pub fn get_legend_key(&self) -> &K {
        &self.legend_key
    }
}


impl<S, K> IntoIterator for PlottableSeries<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    type Item = (String, Point);
    type IntoIter = PlottableSerieIterator<S, K, SampleSerieIterator<S, K>>;

    fn into_iter(self) -> Self::IntoIter {
        let it = SampleSerie::new(self.paths).into_iter();
        PlottableSerieIterator::new(it, self.x_key, self.y_key, self.legend_key)
    }
}


// -----------------------------------------------------------------------------

/// An iterator over a plottable serie
#[derive(Debug, Clone)]
pub struct PlottableSerieIterator<S, K, It>
where
    S : Sample<K>,
    K : SerieKey,
    It : Iterator<Item = S>
{
    iterator : It,
    x_key : K,
    y_key : K,
    legend_key : K,
}

impl<S, K, It> PlottableSerieIterator<S, K, It>
where
    S : Sample<K>,
    K : SerieKey,
    It : Iterator<Item = S>
{
    pub fn new(iterator : It, x_key : K, y_key : K, legend_key : K) -> Self {
        if legend_key.is_numeric() {
            panic!("legend_key must be a string key");
        }
        if !x_key.is_numeric() {
            panic!("x_key must be a numeric key");
        }
        if !y_key.is_numeric() {
            panic!("y_key must be a numeric key");
        }
        PlottableSerieIterator {
            iterator,
            x_key,
            y_key,
            legend_key,
        }
    }

    pub fn get_x_key(&self) -> &K {
        &self.x_key
    }

    pub fn get_y_key(&self) -> &K {
        &self.y_key
    }

    pub fn get_legend_key(&self) -> &K {
        &self.legend_key
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
            let x = sample.get_numeric_value(&self.x_key);
            let y = sample.get_numeric_value(&self.y_key);
            let legend = sample.get_string_value(&self.legend_key);
            (legend, (x, y))
        })
    }
}