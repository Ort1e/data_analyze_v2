
use crate::data::sample::key::SerieKey;
use crate::data::sample::Sample;

use super::Operator;



#[derive(Debug, Clone, PartialEq)]
pub struct DisplayStringFilter<K> 
where
    K : SerieKey
{
    key : K,
    operator : Operator,
    value : String,
}


impl <K> DisplayStringFilter<K>
where
    K : SerieKey
{
    pub fn new(key : K, operator : Operator, value : String) -> Self {
        if !key.is_string() {
            panic!("The key {} is not string", key);
        }

        Self {
            key,
            operator,
            value,
        }
    }
    
    pub fn get_key(&self) -> K {
        self.key
    }
    
    pub fn get_operator(&self) -> &Operator {
        &self.operator
    }
    
    pub fn get_value(&self) -> &str {
        self.value.as_str()
    }

    pub fn apply<S>(&self, sample : &S) -> bool
    where
        S : Sample<K>,
    {
        let value = sample.get_string_value(&self.key);
        self.operator.compare_string(value.as_str(), self.value.as_str())
    }
}