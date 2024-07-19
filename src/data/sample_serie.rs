use std::collections::VecDeque;

use super::sample::key::SerieKey;
use super::sample::Sample;



/// represent a serie of Sample, linked to a sample and a key
#[derive(Debug, Clone)]
pub struct SampleSerie<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    paths : Vec<String>,

    _key : std::marker::PhantomData<K>,
    _sample : std::marker::PhantomData<S>,
}

impl<S, K> SampleSerie<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    /// Create a new serie of Sample
    pub fn new(paths : Vec<String>) -> Self {
        SampleSerie {
            paths,
            _key : std::marker::PhantomData,
            _sample : std::marker::PhantomData,
        }
    }

    /// Get the number of files in the serie
    pub fn nb_files(&self) -> usize {
        self.paths.len()
    }
}

impl<S, K> IntoIterator for SampleSerie<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    type Item = S;
    type IntoIter = SampleSerieIterator<S, K>;

    fn into_iter(self) -> Self::IntoIter {
        SampleSerieIterator::new(self.paths)
    }
}

// -----------------------------------------------------------------------------

/// An iterator over a serie of Sample
#[derive(Debug, Clone)]
pub struct SampleSerieIterator<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    paths : Vec<String>,
    next_index : usize,

    current_sample : VecDeque<S>,

    _key : std::marker::PhantomData<K>,
}

impl <'a, S, K> SampleSerieIterator<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    /// Create a new iterator over a serie of Sample
    fn new(paths : Vec<String>) -> Self {
        SampleSerieIterator {
            paths,
            next_index : 0,
            current_sample : VecDeque::new(),
            _key : std::marker::PhantomData,
        }
    }

    /// Get the number of files in the serie
    pub fn nb_files(&self) -> usize {
        self.paths.len()
    }

    /// Reset the iterator
    pub fn reset(&mut self) {
        self.next_index = 0;
        self.current_sample = VecDeque::new();
    }
}

impl<'a, S, K> Iterator for SampleSerieIterator<S, K>
where
    S : Sample<K>,
    K : SerieKey
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        // If we have no more sample to read, we try the load the next file
        if self.current_sample.is_empty() {
            if self.next_index >= self.paths.len() {
                return None;
            }

            let sample = VecDeque::from(S::new_from_file_path(&self.paths[self.next_index])
                .expect(format!("Error while loading file {}", &self.paths[self.next_index]).as_str()));

            self.current_sample = sample;
            self.next_index += 1;
               
        }

        // If we have a sample, we return the next point
        let sample = self.current_sample.pop_front().expect("Error while reading sample");
        
        Some(sample)
    }
}