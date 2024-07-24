use super::filtering::Filters;
use super::resetable::Resetable;
use super::sample::key::SerieKey;
use super::sample::Sample;


/// represent a serie of Sample, linked to a sample and a key (filtered)
#[derive(Debug)]
pub struct FilteredSerie<'a, S, K, It>
where
    S : Sample<K>,
    K : SerieKey,
    It : Iterator<Item = S>
{
    sample_serie : It,
    filters : &'a Filters<K>,
    _sample : std::marker::PhantomData<S>,
}

impl<'a, S, K, It> FilteredSerie<'a, S, K, It>
where
    S : Sample<K>,
    K : SerieKey,
    It : Iterator<Item = S>
{
    pub fn new(sample_serie : It, filters : &'a Filters<K>) -> Self {
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

impl<'a, S, K, It> IntoIterator for FilteredSerie<'a, S, K, It>
where
    S : Sample<K>,
    K : SerieKey,
    It : Iterator<Item = S>
{
    type Item = S;
    type IntoIter = FilteredSerieIterator<'a, S, K, It>;

    fn into_iter(self) -> Self::IntoIter {
        FilteredSerieIterator {
            sample_serie : self.sample_serie,
            filters : self.filters,
            _sample : std::marker::PhantomData,
        }
    }
}

// -----------------------------------------------------------------------------

/// An iterator over a serie of Sample (filtered)
pub struct FilteredSerieIterator<'a, S, K, It>
where
    S : Sample<K>,
    K : SerieKey,
    It : Iterator<Item = S>
{
    sample_serie : It,
    filters : &'a Filters<K>,
    _sample : std::marker::PhantomData<S>,
}

impl<'a, S, K, It> Iterator for FilteredSerieIterator<'a, S, K, It>
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
                    if self.filters.apply(&sample) {
                        return Some(sample);
                    }
                },
                None => return None,
            }
        }
    }
}

impl<'a, S, K, It> Resetable for FilteredSerieIterator<'a, S, K, It>
where
    S : Sample<K>,
    K : SerieKey,
    It : Iterator<Item = S> + Resetable
{
    fn reset(&mut self) {
        self.sample_serie.reset();
    }
}
