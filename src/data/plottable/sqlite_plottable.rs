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

impl<'it_lt, S, K> Plottable<'it_lt, S, K,  SqliteSampleSerieIterator<'it_lt, S, K>> for SqlitePlottable<S, K> 
where 
    S : SqliteSample<K> + 'it_lt,
    K : SerieKey +'it_lt,
{
    fn into_sample_iter<'a>(&'a self) -> SqliteSampleSerieIterator<'it_lt, S, K> where 'a : 'it_lt {
        SqliteSampleSerieIterator::new(&self.conn, &self.query).into_iter()
    }
}
