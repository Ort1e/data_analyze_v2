use crate::data::sample::file_sample::FileSample;
use crate::data::sample::key::SerieKey;
use crate::data::sample_serie::file_sample_serie::{FileSampleSerie, FileSampleSerieIterator};

use super::Plottable;

/// Define a plottable serie (legend associated with points)
#[derive(Debug, Clone)]
pub struct FilePlottable<S, K>
where
    S : FileSample<K>,
    K : SerieKey
{
    paths : Vec<String>,
    _key : std::marker::PhantomData<K>,
    _sample : std::marker::PhantomData<S>,
}


impl<S, K> FilePlottable<S, K>
where
    S : FileSample<K>,
    K : SerieKey,
{
    pub fn new(paths : Vec<String>) -> Self {
        FilePlottable {
            paths,
            _key : std::marker::PhantomData,
            _sample : std::marker::PhantomData,
        }
    }
}

impl<S, K> Plottable<'static, S, K, FileSampleSerieIterator<S, K>> for FilePlottable<S, K>
where
    S: FileSample<K> + 'static,
    K: SerieKey + 'static,
{
    fn into_sample_iter<'a>(&'a self) -> FileSampleSerieIterator<S, K> where 'a : 'static {
        FileSampleSerie::new(self.paths.clone()).into_iter()
    }
}
