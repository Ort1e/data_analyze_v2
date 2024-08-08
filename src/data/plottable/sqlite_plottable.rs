use sqlite::Connection;

use crate::data::sample::key::SerieKey;
use crate::data::sample::sqlite_sample::SqliteSample;
use crate::data::sample_serie::sqlite_sample_serie::SqliteSampleSerieIterator;

use super::Plottable;






/// Define a plottable serie (legend associated with points)
pub struct SqlitePlottable<S, K>
where
    S : SqliteSample<K>,
    K : SerieKey
{
    conn : Connection,
    query : String,
    _key : std::marker::PhantomData<K>,
    _sample : std::marker::PhantomData<S>,
}


impl<S, K> SqlitePlottable<S, K>
where
    S : SqliteSample<K>,
    K : SerieKey,
{
    pub fn new(conn : Connection, query : &str) -> Self {
        SqlitePlottable {
            conn,
            query : query.to_string(),
            _key : std::marker::PhantomData,
            _sample : std::marker::PhantomData,
        }
    }
}

impl<'a, S, K> Plottable<'a, S, K, SqliteSampleSerieIterator<'a, S, K>> for SqlitePlottable<S, K> 
where 
    S : SqliteSample<K>,
    K : SerieKey,
{
    fn into_sample_iter(&'a self) -> SqliteSampleSerieIterator<'a, S, K> {
        SqliteSampleSerieIterator::new(&self.conn, &self.query).into_iter()
    }
}

