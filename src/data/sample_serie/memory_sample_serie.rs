use std::collections::VecDeque;

use crate::data::resetable::Resetable;
use crate::data::sample::file_sample::FileSample;
use crate::data::sample::key::SerieKey;
use crate::data::sample::sqlite_sample::SqliteSample;
use crate::data::sample::Sample;

use super::file_sample_serie::FileSampleSerie;

#[cfg(feature = "sqlite")]
use super::sqlite_sample_serie::SqliteSampleSerie;



/// represent a serie of Sample, linked to a sample and a key
#[derive(Debug, Clone)]
pub struct MemorySampleSerie<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    samples : VecDeque<S>,

    _key : std::marker::PhantomData<K>,
}

impl<S, K> MemorySampleSerie<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    /// Create a new serie of Sample
    pub fn new<I : IntoIterator<Item = S>>(samples : I) -> Self {
        MemorySampleSerie {
            samples : samples.into_iter().collect(),
            _key : std::marker::PhantomData,
        }
    }

    /// Get the number of files in the serie
    pub fn nb_samples(&self) -> usize {
        self.samples.len()
    }
}

impl<S, K> IntoIterator for MemorySampleSerie<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    type Item = S;
    type IntoIter = MemorySampleSerieIntoIterator<S, K>;

    fn into_iter(self) -> Self::IntoIter {
        MemorySampleSerieIntoIterator::new(self.samples)
    }
}

// ----------------------------------- FROM ------------------------------------------

impl<'a, S, K> From<FileSampleSerie<S, K>> for MemorySampleSerie<S, K>
where
    S : FileSample<K>,
    K : SerieKey
{
    fn from(samples : FileSampleSerie<S, K>) -> Self {
        MemorySampleSerie {
            samples : samples.into_iter().collect(),
            _key : std::marker::PhantomData,
        }
    }
}

#[cfg(feature = "sqlite")]
impl<'a, S, K, Sub> From<SqliteSampleSerie<'a, S, K, Sub>> for MemorySampleSerie<S, K>
where
    S : SqliteSample<K, Sub>,
    K : SerieKey
{
    fn from(samples : SqliteSampleSerie<'a, S, K, Sub>) -> Self {
        MemorySampleSerie {
            samples : samples.into_iter().collect(),
            _key : std::marker::PhantomData,
        }
    }
}

// ----------------------------------- ITERATOR ------------------------------------------

#[derive(Debug, Clone)]
pub struct MemorySampleSerieIterator<'a, S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    samples : &'a MemorySampleSerie<S, K>,
    index : usize,

    _key : std::marker::PhantomData<K>,
}

impl<'a, S, K> MemorySampleSerieIterator<'a, S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    /// Create a new iterator over a serie of Sample
    pub fn new(samples : &'a MemorySampleSerie<S, K>) -> Self {
        MemorySampleSerieIterator {
            samples,
            index : 0,
            _key : std::marker::PhantomData,
        }
    }
}

impl<'a, S, K> ExactSizeIterator for MemorySampleSerieIterator<'a, S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    fn len(&self) -> usize {
        self.samples.samples.len() - self.index
    }
}

impl<'a, S, K> Resetable for MemorySampleSerieIterator<'a, S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    fn reset(&mut self) {
        self.index = 0;
    }
}

impl<'a, S, K> Iterator for MemorySampleSerieIterator<'a, S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    type Item = &'a S;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.samples.samples.len() {
            let result = Some(&self.samples.samples[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}


// ----------------------------------- INTO ITERATOR ------------------------------------------

/// An iterator over a serie of Sample
#[derive(Debug, Clone)]
pub struct MemorySampleSerieIntoIterator<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    samples : VecDeque<S>,

    _key : std::marker::PhantomData<K>,
}

impl <S, K> MemorySampleSerieIntoIterator<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    /// Create a new iterator over a serie of Sample
    fn new(samples : VecDeque<S>) -> Self {
        MemorySampleSerieIntoIterator {
            samples,
            _key : std::marker::PhantomData,
        }
    }

    /// Get the number of files in the serie
    pub fn nb_samples(&self) -> usize {
        self.samples.len()
    }
}

impl<S, K> Iterator for MemorySampleSerieIntoIterator<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        self.samples.pop_front()
    }
}