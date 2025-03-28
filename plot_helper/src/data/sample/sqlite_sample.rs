use sqlite::Statement;

use super::key::SerieKey;
use super::Sample;








/// Define a sample linked to a key from a sqlite row
pub trait SqliteSample<K, Sub> : Sample<K>
where 
    Self : Clone + Sized + Send + Sync,
    K : SerieKey,
{
    /// Load samples from a satement. The statement is garantea to be without error, and the information 
    /// can be extracted from the statement like this:
    /// ```
    /// let value = row.read::<i64, _>(0);
    /// ```
    fn new_from_row(row : &Statement) -> Result<Vec<Self>, Box<dyn std::error::Error>>;
    
    /// Get the select query to get the samples from the database
    /// - args: the substitution to apply to the query
    fn get_sqlite_select_query(substitution : &Sub) -> String;
}
