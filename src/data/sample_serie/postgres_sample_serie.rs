use std::collections::VecDeque;

use postgres::{Client, Portal, Transaction};

use crate::data::plottable::Plottable;
use crate::data::sample::key::SerieKey;
use crate::data::sample::postgres_sample::PostgresSample;

/// represent a serie of Sample, linked to a sample and a key
pub struct PostgresSampleSerie<S, K>
where
    S : PostgresSample<K>,
    K : SerieKey
{
    conn : Client,

    _key : std::marker::PhantomData<K>,
    _sample : std::marker::PhantomData<S>,
}

impl<S, K> PostgresSampleSerie<S, K>
where
    S : PostgresSample<K>,
    K : SerieKey
{
    /// Create a new serie of Sample
    /// -args: conn: the connection string to the database
    /// -args: sub: the arg to pass to the construction of the query
    pub fn new(conn_string : &str) -> Self {
        PostgresSampleSerie {
            conn : Client::connect(conn_string, postgres::NoTls).expect("Error while connecting to the database"),
            _key : std::marker::PhantomData,
            _sample : std::marker::PhantomData,
        }
    }
}

impl<'it, S, K> IntoIterator for &'it mut PostgresSampleSerie<S, K>
where
    S : PostgresSample<K>,
    K : SerieKey
{
    type Item = S;
    type IntoIter = PostgresSampleSerieIntoIterator<'it, S, K>;

    fn into_iter(self) -> Self::IntoIter {
        PostgresSampleSerieIntoIterator::new(self.conn.transaction().expect("Error while starting transaction"))
    }
}

impl<S, K> Plottable<S, K> for PostgresSampleSerie<S, K>
where
    S : PostgresSample<K>,
    K : SerieKey
{}

// -----------------------------------------------------------------------------

/// An iterator over a serie of Sample
pub struct PostgresSampleSerieIntoIterator<'a, S, K>
where
    S : PostgresSample<K>,
    K : SerieKey
{
    transaction : Transaction<'a>,
    portal : Portal,

    current_sample : VecDeque<S>,

    _key : std::marker::PhantomData<K>,
}

impl <'a, S, K> PostgresSampleSerieIntoIterator<'a, S, K>
where
    S : PostgresSample<K>,
    K : SerieKey
{
    /// Create a new iterator over a serie of Sample
    /// -args: sub: the arg to pass to the construction of the query
    pub fn new(mut conn : Transaction<'a>) -> Self {
        let query = S::get_postgres_select_query();

        let portal = conn.bind(query.as_str(), &[]).expect("Error while binding query");

        PostgresSampleSerieIntoIterator {
            transaction : conn,
            portal,
            current_sample : VecDeque::new(),
            _key : std::marker::PhantomData,
        }
    }
}

impl<'a, S, K> Iterator for PostgresSampleSerieIntoIterator<'a, S, K>
where
    S : PostgresSample<K>,
    K : SerieKey
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        // If we have no more sample to read, we try the load the next file
        if self.current_sample.is_empty() {
            // We try to load the next sample
            let rows = self.transaction.query_portal(&self.portal, 1).expect("Error while querying portal");

            let sample = S::new_from_row(&rows[0]).unwrap();
            self.current_sample = sample.into_iter().collect();
                
            self.next()
               
        }else {
            // If we have a sample, we return the next point
            let sample = self.current_sample.pop_front().expect("Error while reading sample");
            
            Some(sample)
        }
    }
}