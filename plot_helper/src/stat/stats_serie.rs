use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[cfg(feature = "parrallelize")]
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

/// represent a serie with its stats
#[derive(Debug, Clone)]
pub struct StatsSerie {
    pub serie : Vec<f32>,
    pub stats : HashMap<MetricName, MetricValue>,
}

impl StatsSerie {
    pub fn new(serie : &Vec<f32>) -> Self {
        let mut stats = HashMap::new();

        if serie.len() == 0 {
            stats.insert(MetricName::Mean, MetricName::Mean.with_value(f64::NAN));
            stats.insert(MetricName::Median, MetricName::Median.with_value(f64::NAN));
            stats.insert(MetricName::Additive, MetricName::Additive.with_value(f64::NAN));
            stats.insert(MetricName::NbValues, MetricName::NbValues.with_value(0.0));
            stats.insert(MetricName::StandardDeviation, MetricName::StandardDeviation.with_value(f64::NAN));
            stats.insert(MetricName::Min, MetricName::Min.with_value(f64::NAN));
            stats.insert(MetricName::Max, MetricName::Max.with_value(f64::NAN));
            

            return Self {
                serie : serie.clone(),
                stats,
            }
        }
        let nb_value = serie.len() as u64;

        let mut additive = 0.0;
        let mut min = f32::MAX;
        let mut max = f32::MIN;

        for value in serie.iter() {
            additive += *value as f64;
            if *value < min {
                min = *value;
            }
            if *value > max {
                max = *value;
            }
        }
                
        stats.insert(
            MetricName::Mean, 
            MetricName::Mean.with_value(additive / serie.len() as f64)
        );

        let sorted_serie = {
            let mut sorted_serie = serie.clone();
            sorted_serie.sort_by(|a, b| a.partial_cmp(b).unwrap());
            sorted_serie
        };
        if serie.len() % 2 == 0 {
            stats.insert(MetricName::Median, MetricName::Median.with_value((sorted_serie[serie.len() / 2] as f64 + sorted_serie[serie.len() / 2 - 1] as f64) / 2.0));
        }else{
            stats.insert(MetricName::Median, MetricName::Median.with_value(sorted_serie[serie.len() / 2] as f64));
        }

        stats.insert(MetricName::Additive, MetricName::Additive.with_value(additive));
        stats.insert(MetricName::Min, MetricName::Min.with_value(min as f64));
        stats.insert(MetricName::Max, MetricName::Max.with_value(max as f64));

        stats.insert(MetricName::NbValues, MetricName::NbValues.with_value(nb_value as f64));

        let standard_deviation = if serie.len() == 1 {
            0.0
        }else{
            let mean = stats.get(&MetricName::Mean).unwrap().value;
            let sum = serie.iter().map(|f| (*f as f64 - mean).powi(2)).sum::<f64>();
            (sum / (serie.len() as f64 - 1.0)).sqrt()
        };

        stats.insert(MetricName::StandardDeviation, MetricName::StandardDeviation.with_value(standard_deviation));

        Self {
            serie : serie.clone(),
            stats,
        }
    }

    pub fn get_stats(&self, metric : MetricName) -> MetricValue {
        self.stats.get(&metric).unwrap().clone()
    }
}

impl Display for StatsSerie {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        let mut keys = self.stats.keys().collect::<Vec<&MetricName>>();
        keys.sort();
        for key in keys {
            let value = self.stats.get(key).unwrap();
            str.push_str(&format!("{} ", value));
        }

        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MetricValue {
    pub name : MetricName,
    pub value : f64,
}

impl Display for MetricValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:.2}", self.name, self.value)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum MetricName {
    Mean,
    Median,
    Additive,
    NbValues,
    StandardDeviation,
    Min,
    Max,
}

impl Display for MetricName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}


impl MetricName {
    pub fn get_name(&self) -> String {
        match self {
            MetricName::Mean => "mean".to_string(),
            MetricName::Median => "median".to_string(),
            MetricName::Additive => "additive".to_string(),
            MetricName::NbValues => "nb_values".to_string(),
            MetricName::StandardDeviation => "standard_deviation".to_string(),
            MetricName::Min => "min".to_string(),
            MetricName::Max => "max".to_string(),
        }
    }

    pub fn get_all() -> Vec<MetricName> {
        vec![
            MetricName::Mean,
            MetricName::Median,
            MetricName::Additive,
            MetricName::NbValues,
            MetricName::StandardDeviation,
            MetricName::Min,
            MetricName::Max,
        ]
    }

    pub fn with_value(self, value : f64) -> MetricValue {
        MetricValue {
            name : self,
            value,
        }
    }
}