
use crate::data::sample::key::SerieKey;
use crate::data::sample::Sample;

use super::Operator;



#[derive(Debug, Clone, PartialEq)]
pub struct DisplayNumberFilter<K> 
where
    K : SerieKey
{
    key : K,
    operator : Operator,
    value : f32,
}


impl <K> DisplayNumberFilter<K>
where
    K : SerieKey
{
    pub fn new(key : K, operator : Operator, value : f32) -> Self {
        if !key.is_numeric() {
            panic!("The key {} is not numeric", key);
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
    
    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn apply<S>(&self, sample : &S) -> bool
    where
        S : Sample<K>,
    {
        let value = sample.get_numeric_value(&self.key);
        self.operator.compare_number(value, self.value)
    }
}