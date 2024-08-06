use super::sample::key::SerieKey;
use super::sample::Sample;

pub mod file_sample_serie;

/// this is only a marker trait, to be able to be used as an iterator
pub trait SampleSerie<S, K> : Iterator<Item = S>
where
    S : Sample<K>,
    K : SerieKey
{

}