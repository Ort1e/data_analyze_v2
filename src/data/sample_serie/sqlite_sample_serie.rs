use std::collections::VecDeque;

use sqlite::{Connection, State, Statement};

use crate::data::sample::key::SerieKey;
use crate::data::sample::sqlite_sample::SqliteSample;

/// represent a serie of Sample, linked to a sample and a key
#[derive(Clone)]
pub struct SqliteSampleSerie<'a, S, K, Sub>
where
    S : SqliteSample<K, Sub>,
    K : SerieKey
{
    conn : &'a Connection,
    sub : Sub,

    _key : std::marker::PhantomData<K>,
    _sample : std::marker::PhantomData<S>,
}

impl<'a, S, K, Sub> SqliteSampleSerie<'a, S, K, Sub>
where
    S : SqliteSample<K, Sub>,
    K : SerieKey
{
    /// Create a new serie of Sample
    /// -args: conn: the connection to the database
    /// -args: sub: the arg to pass to the construction of the query
    pub fn new(conn : &'a Connection, sub : Sub) -> Self {
        SqliteSampleSerie {
            conn,
            sub,
            _key : std::marker::PhantomData,
            _sample : std::marker::PhantomData,
        }
    }

    pub fn get_connection(&self) -> &'a Connection {
        self.conn
    }
}

impl<'a, S, K, Sub> IntoIterator for SqliteSampleSerie<'a, S, K, Sub>
where
    S : SqliteSample<K, Sub>,
    K : SerieKey
{
    type Item = S;
    type IntoIter = SqliteSampleSerieIterator<'a, S, K, Sub>;

    fn into_iter(self) -> Self::IntoIter {
        SqliteSampleSerieIterator::new(self.conn, &self.sub)
    }
}

// -----------------------------------------------------------------------------

/// An iterator over a serie of Sample
pub struct SqliteSampleSerieIterator<'a, S, K, Sub>
where
    S : SqliteSample<K, Sub>,
    K : SerieKey
{
    stmt : Statement<'a>,

    current_sample : VecDeque<S>,

    _key : std::marker::PhantomData<K>,
    _sub : std::marker::PhantomData<Sub>,
}

impl <'a, S, K, Sub> SqliteSampleSerieIterator<'a, S, K, Sub>
where
    S : SqliteSample<K, Sub>,
    K : SerieKey
{
    /// Create a new iterator over a serie of Sample
    /// -args: sub: the arg to pass to the construction of the query
    pub fn new(conn : &'a Connection, sub : &Sub) -> Self {
        let query = S::get_sqlite_select_query(sub);
        let stmt = conn.prepare(query).expect("Error while preparing statement");


        SqliteSampleSerieIterator {
            stmt,
            current_sample : VecDeque::new(),
            _key : std::marker::PhantomData,
            _sub : std::marker::PhantomData,
        }
    }
}

impl<'a, S, K, Sub> Iterator for SqliteSampleSerieIterator<'a, S, K, Sub>
where
    S : SqliteSample<K, Sub>,
    K : SerieKey
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        // If we have no more sample to read, we try the load the next file
        if self.current_sample.is_empty() {
            // We try to load the next sample
            let state = self.stmt.next();

            match state {
                Ok(State::Row) => {
                    let sample = S::new_from_row(&self.stmt).expect("Error while reading sample");
                    self.current_sample = sample.into_iter().collect();
                },
                Ok(State::Done) => return None,
                Err(e) => panic!("Error while reading sample: {:?}", e),
            }


            self.next()
               
        }else {
            // If we have a sample, we return the next point
            let sample = self.current_sample.pop_front().expect("Error while reading sample");
            
            Some(sample)
        }
    }
}