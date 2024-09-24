use sqlite::Connection;

use crate::data::sample::key::SerieKey;
use crate::data::sample::sqlite_sample::SqliteSample;
use crate::data::sample_serie::sqlite_sample_serie::SqliteSampleSerieIterator;

use super::Plottable;






/// Define a plottable serie (legend associated with points)
pub struct SqlitePlottable<S, K, Sub>
where
    S : SqliteSample<K, Sub>,
    K : SerieKey
{
    conn : Connection,
    sub : Sub,
    _key : std::marker::PhantomData<K>,
    _sample : std::marker::PhantomData<S>,
}


impl<S, K, Sub> SqlitePlottable<S, K, Sub>
where
    S : SqliteSample<K, Sub>,
    K : SerieKey,
{   
    /// Create a new plottable serie
    /// -args: conn: the connection to the database
    /// -args: sub: the arg to pass to the construction of the query
    pub fn new(conn : Connection, sub : Sub) -> Self {
        SqlitePlottable {
            conn,
            sub,
            _key : std::marker::PhantomData,
            _sample : std::marker::PhantomData,
        }
    }

    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }
}

impl<'it_lt, S, K, Sub> Plottable<'it_lt, S, K,  SqliteSampleSerieIterator<'it_lt, S, K, Sub>> for SqlitePlottable<S, K, Sub> 
where 
    Sub : 'it_lt,
    S : SqliteSample<K, Sub> + 'it_lt,
    K : SerieKey +'it_lt,
{
    fn into_sample_iter<'a>(&'a self) -> SqliteSampleSerieIterator<'it_lt, S, K, Sub> where 'a : 'it_lt {
        SqliteSampleSerieIterator::new(&self.conn, &self.sub).into_iter()
    }
}

