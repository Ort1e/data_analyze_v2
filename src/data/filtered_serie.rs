use super::filtering::Filter;
use super::sample::key::SerieKey;
use super::sample::Sample;


/// represent a serie of Sample, linked to a sample and a key (filtered)
#[derive(Debug)]
pub struct FilteredSerie<S, K, It>
where
    S : Sample<K>,
    K : SerieKey,
{
    sample_serie : It,
    filter : Filter<K>,
    _sample : std::marker::PhantomData<S>,
}

impl<S, K, It> FilteredSerie<S, K, It>
where
    S : Sample<K>,
    K : SerieKey
{
    pub fn new(sample_serie : It, filter : Filter<K>) -> Self {
        FilteredSerie {
            sample_serie,
            filter,
            _sample : std::marker::PhantomData,
        }
    }
}

impl<S, K, It> Iterator for FilteredSerie<S, K, It>
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
                    if !self.filter.apply(&sample) {
                        return Some(sample);
                    }
                },
                None => return None,
            }
        }
    }
}


