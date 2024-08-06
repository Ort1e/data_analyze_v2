use std::collections::VecDeque;

use crate::data::resetable::Resetable;
use crate::data::sample::file_sample::FileSample;
use crate::data::sample::key::SerieKey;



/// represent a serie of Sample, linked to a sample and a key
#[derive(Debug, Clone)]
pub struct FileSampleSerie<S, K>
where
    S : FileSample<K>,
    K : SerieKey
{
    paths : Vec<String>,

    _key : std::marker::PhantomData<K>,
    _sample : std::marker::PhantomData<S>,
}

impl<S, K> FileSampleSerie<S, K>
where
    S : FileSample<K>,
    K : SerieKey
{
    /// Create a new serie of Sample
    pub fn new(paths : Vec<String>) -> Self {
        FileSampleSerie {
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

impl<S, K> IntoIterator for FileSampleSerie<S, K>
where
    S : FileSample<K>,
    K : SerieKey
{
    type Item = S;
    type IntoIter = FileSampleSerieIterator<S, K>;

    fn into_iter(self) -> Self::IntoIter {
        FileSampleSerieIterator::new(self.paths)
    }
}

// -----------------------------------------------------------------------------

/// An iterator over a serie of Sample
#[derive(Debug, Clone)]
pub struct FileSampleSerieIterator<S, K>
where
    S : FileSample<K>,
    K : SerieKey
{
    paths : Vec<String>,
    next_index : usize,

    current_sample : VecDeque<S>,

    _key : std::marker::PhantomData<K>,
}

impl <S, K> FileSampleSerieIterator<S, K>
where
    S : FileSample<K>,
    K : SerieKey
{
    /// Create a new iterator over a serie of Sample
    fn new(paths : Vec<String>) -> Self {
        FileSampleSerieIterator {
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
}

impl<S, K> Resetable for FileSampleSerieIterator<S, K>
where
    S : FileSample<K>,
    K : SerieKey
{
    fn reset(&mut self) {
        self.next_index = 0;
        self.current_sample = VecDeque::new();
    }
}

impl<S, K> Iterator for FileSampleSerieIterator<S, K>
where
    S : FileSample<K>,
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

            self.next()
               
        }else {
            // If we have a sample, we return the next point
            let sample = self.current_sample.pop_front().expect("Error while reading sample");
            
            Some(sample)
        }
    }
}