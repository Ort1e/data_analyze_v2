use super::filtering::Filters;
use super::sample::key::SerieKey;
use super::sample::Sample;


/// represent a serie of Sample, linked to a sample and a key (filtered)
#[derive(Debug)]
pub struct FilteredSerie<S, K, It>
where
    S : Sample<K>,
    K : SerieKey,
    It : Iterator<Item = S>
{
    sample_serie : It,
    filters : Filters<K>,
    _sample : std::marker::PhantomData<S>,
}

impl<S, K, It> FilteredSerie<S, K, It>
where
    S : Sample<K>,
    K : SerieKey,
    It : Iterator<Item = S>
{
    pub fn new(sample_serie : It, filters : Filters<K>) -> Self {
        FilteredSerie {
            sample_serie,
            filters,
            _sample : std::marker::PhantomData,
        }
    }

    pub fn get_iter(&self) -> &It {
        &self.sample_serie
    }
}

impl<S, K, It> IntoIterator for FilteredSerie<S, K, It>
where
    S : Sample<K>,
    K : SerieKey,
    It : Iterator<Item = S>
{
    type Item = S;
    type IntoIter = FilteredSerieIterator<S, K, It>;

    fn into_iter(self) -> Self::IntoIter {
        FilteredSerieIterator {
            sample_serie : self.sample_serie,
            filters : self.filters,
            _sample : std::marker::PhantomData,
        }
    }
}

pub struct FilteredSerieIterator<S, K, It>
where
    S : Sample<K>,
    K : SerieKey,
    It : Iterator<Item = S>
{
    sample_serie : It,
    filters : Filters<K>,
    _sample : std::marker::PhantomData<S>,
}

impl<S, K, It> Iterator for FilteredSerieIterator<S, K, It>
where
    S : Sample<K>,
    K : SerieKey,
    It : Iterator<Item = S>
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        loop { // Skip samples that match the filter
            match self.sample_serie.next() {
                Some(sample) => {
                    if !self.filters.apply(&sample) {
                        return Some(sample);
                    }
                },
                None => return None,
            }
        }
    }
}


