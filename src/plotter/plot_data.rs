use std::collections::HashMap;
use std::mem;
use std::ops::Range;

use crate::data::rangeable::Rangeable;
use crate::stat::compression::compress_data_serie;
use crate::stat::remove_outliers;
use crate::stat::stats_serie::{MetricName, StatsSerie};






/// a (x, y) point
type Point = (f32, f32);

/// represent pluggable data, indexed by a legend, for one graph
#[derive(Debug, Clone)]
pub struct PlotData {
    data:  HashMap<String, Vec<Point>>,
    x_range: Range<f32>,
    y_range: Range<f32>,
}

impl PlotData {
    /// create a new PlotData from an iterator of (String, Point) and a metric to aggregate the data
    /// Also compress the data to accelerate the plotting
    pub fn from_it<It>(mut data : It, aggregation_metric : Option<MetricName>, remove_outlier : bool) -> Self
    where
        It : Iterator<Item = (String, Point)> + Rangeable
    {
        let mut data_collected = HashMap::new();
        while let Some((key, point)) = data.next() {
            data_collected.entry(key).or_insert_with(Vec::new).push(point);
        }

        if remove_outlier {
            for (_, serie) in data_collected.iter_mut() {
                *serie = remove_outliers(mem::take(serie));
            }
        }

        let (mut x_range, y_range) = data.get_range().unwrap_or((0.0..1.0, 0.0..1.0));

        if x_range.start == x_range.end {
            x_range = x_range.start - 0.5..x_range.end + 0.5;
        }


        let mut self_ = Self {
            data : data_collected,
            x_range,
            y_range,
        };

        if let Some(metric) = aggregation_metric {
            self_ = self_.apply_aggregator(metric).unwrap();
        }

        self_.compress();

        self_
    }


    /// compress the data to accelerate the plotting
    fn compress(&mut self) -> &mut Self{
        let (range_x, range_y) = self.get_range();
        let original_data = mem::replace(&mut self.data, HashMap::new()); // take out the map
        // Transform the data.
        self.data = original_data.into_iter().map(|(key, serie)| {
            // Now you can avoid cloning the key, as `key` is owned here due to `into_iter()`.
            let compressed_serie = compress_data_serie(serie, &range_x, &range_y);
            (key, compressed_serie) // No need to clone the key.
        }).collect();

        return self
    }

    /// aggregate the data and combine the value with the same x value with a specified metric
    fn apply_aggregator(self, aggregator : MetricName) -> Result<PlotData, Box<dyn std::error::Error>> {
        let mut aggregated_data = HashMap::new();
        for (key, mut serie) in self.data.into_iter() {
            serie.sort_by(|(x1, _), (x2, _)| x1.partial_cmp(x2).unwrap());
            let mut aggregated_serie = Vec::new(); // new serie
            let mut current_x = f32::MIN; // current x value for the aggregation
            let mut current_y = Vec::new(); // all the y values for the current x value
            for (x, y) in serie.into_iter() {
                if x == current_x { // if the x value is the same as the current one, add the y value to the current y values
                    current_y.push(y);
                } else { // if the x value is different, calculate the metrics and reset the current x and y values
                    if current_y.len() != 0 {
                        // calculate the metrics
                        let stats = StatsSerie::new(&current_y).get_stats(aggregator);
                        aggregated_serie.push((current_x, stats.value as f32));
                    }
                    current_x = x;
                    current_y = Vec::new();
                    current_y.push(y);
                }
            }
            if current_y.len() != 0 {
                // calculate the metrics
                let stats = StatsSerie::new(&current_y).get_stats(aggregator);
                aggregated_serie.push((current_x, stats.value as f32));

            }
            // replace the serie with the aggregated one
            aggregated_data.insert(key.clone(), aggregated_serie);
        }
        Ok(aggregated_data.into())
    }

    fn get_range_from_hashmap(data : &HashMap<String, Vec<Point>>) -> (Range<f32>, Range<f32>) {
        let mut y_min = f32::MAX;
        let mut y_max = f32::MIN;
    
        let mut x_min = f32::MAX;
        let mut x_max = f32::MIN;
    
        // parcour the data and get the min and max of each axis
        for (_, serie) in data.iter() {
            for (x, y) in serie.iter() {
                if *x < x_min {
                    x_min = *x;
                }
                if *x > x_max {
                    x_max = *x;
                }
    
                if *y < y_min {
                    y_min = *y;
                }
                if *y > y_max {
                    y_max = *y;
                }
            }
        }
    
        if y_min == f32::MAX || y_max == f32::MIN {// if the data is empty
            y_min = 0.0;
            y_max = 1.0;
        }
    
        if x_min == f32::MAX || x_max == f32::MIN {// if the data is empty
            x_min = 0.0;
            x_max = 1.0;
        }
    
        if x_min == x_max {
            x_min -= 0.5;
            x_max += 0.5;
        }
    
        (x_min..x_max, y_min..y_max)
    }

    pub fn get_data(&self) -> &HashMap<String, Vec<Point>> {
        &self.data
    }

    pub fn get_range(&self) -> (Range<f32>, Range<f32>) {
        (self.x_range.clone(), self.y_range.clone())
    }
}

impl From<HashMap<String, Vec<Point>>> for PlotData {
    fn from(data : HashMap<String, Vec<Point>>) -> Self {
        let (x_range, y_range) = PlotData::get_range_from_hashmap(&data);
        Self {
            data,
            x_range,
            y_range,
        }
    }
}

impl Into<HashMap<String, Vec<Point>>> for PlotData {
    fn into(self) -> HashMap<String, Vec<Point>> {
        self.data
    }
}

impl IntoIterator for PlotData {
    type Item = (String, Vec<Point>);
    type IntoIter = std::collections::hash_map::IntoIter<String, Vec<Point>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}