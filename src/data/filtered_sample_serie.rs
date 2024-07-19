use super::filtering::Filter;
use super::sample::key::SerieKey;
use super::sample::Sample;
use super::sample_serie::{SampleSerie, SampleSerieIterator};


/// represent a serie of Sample, linked to a sample and a key (filtered)
#[derive(Debug)]
pub struct FilteredSampleSerie<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    sample_serie : SampleSerie<S, K>,
    filters : Filter<K>,
}

impl<S, K> FilteredSampleSerie<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    pub fn new(sample_serie : SampleSerie<S, K>, filters : Filter<K>) -> Self {
        FilteredSampleSerie {
            sample_serie,
            filters,
        }
    }
}

impl <S, K> IntoIterator for FilteredSampleSerie<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    type Item = S;
    type IntoIter = FilteredSampleSerieIterator<S, K>;

    fn into_iter(self) -> Self::IntoIter {
        FilteredSampleSerieIterator::new(self.sample_serie.into_iter(), self.filters)
    }
}

// -----------------------------------------------------------------------------

/// An iterator over a serie of Sample (filtered)
#[derive(Debug)]
pub struct FilteredSampleSerieIterator<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    sample_serie_iter : SampleSerieIterator<S, K>,
    filter : Filter<K>,
}

impl<S, K> FilteredSampleSerieIterator<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    pub fn new(sample_serie_iter : SampleSerieIterator<S, K>, filter : Filter<K>) -> Self {
        FilteredSampleSerieIterator {
            sample_serie_iter,
            filter,
        }
    }

    pub fn reset(&mut self) {
        self.sample_serie_iter.reset();
    }

    pub fn nb_files(&self) -> usize {
        self.sample_serie_iter.nb_files()
    }
}


impl<S, K> Iterator for FilteredSampleSerieIterator<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        loop { // Skip samples that match the filter
            match self.sample_serie_iter.next() {
                Some(sample) => {
                    if !self.filter.apply(&sample) {
                        return Some(sample);
                    }
                },
                None => return None,
            }
        }
    }
}

