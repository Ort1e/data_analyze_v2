use crate::data::sample::file_sample::FileSample;
use crate::data::sample::key::SerieKey;
use crate::data::sample_serie::file_sample_serie::{FileSampleSerie, FileSampleSerieIterator};

use super::PlottableSerie;

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
}

impl<S, K> PlottableSerie<S, K, FileSampleSerieIterator<S, K>> for FilePlottableSerie<S, K> 
where 
    S : FileSample<K>,
    K : SerieKey,
{
    fn into_sample_iter(&self) -> FileSampleSerieIterator<S, K> {
        FileSampleSerie::new(self.paths.clone()).into_iter()
    }
}

