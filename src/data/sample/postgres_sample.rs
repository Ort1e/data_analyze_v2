use postgres::Row;

use super::key::SerieKey;
use super::Sample;








/// Define a sample linked to a key from a postgres row
pub trait PostgresSample<K> : Sample<K>
where 
    Self : Clone + Sized + Send + Sync,
    K : SerieKey
{
    /// Load samples from a row. The row is garantea to be without error, and the information 
    /// can be extracted from the row like this:
    /// ```
    /// let id : i64 = row.get("id");
    /// ```
    fn new_from_row(row : &Row) -> Result<Vec<Self>, Box<dyn std::error::Error>>;
    
    /// Get the select query to get the samples from the database
    /// - args: the substitution to apply to the query
    fn get_postgres_select_query() -> String;
}
