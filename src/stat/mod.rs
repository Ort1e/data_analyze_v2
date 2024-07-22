
pub mod stats_serie;
pub mod compression;
pub mod linspace;

/// get the outliers of the given data (Mask)
/// return the vector of bool, true if the corresponding data is an outlier
pub fn remove_outliers<X>(
    mut data_to_filter: Vec<(X, f32)>,
) -> Vec<(X, f32)> {
    let q1_q3 = calculate_q1_q3(&mut data_to_filter);
    let (lower_bound, upper_bound) = calculate_bounds(&q1_q3);

    data_to_filter.into_iter().filter(|point| point.1 >= lower_bound && point.1 <= upper_bound).collect()
}

/// use the inverted_cdf method to get the q1 and q3
fn calculate_q1_q3<X>(data: &mut Vec<(X, f32)>) -> (f32, f32) {
    if data.len() == 0 {
        return (f32::NAN, f32::NAN);
    }

    // Sort the data first
    data.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let freq_q1 = (data.len() as f32) / 4.0;
    let freq_q3 = (data.len() as f32) * 3.0 / 4.0;

    let q1_index = freq_q1.ceil() as usize - 1;

        
    let q3_index =  freq_q3.ceil() as usize - 1;

    
    let q1 = data[q1_index].1;
    let q3 = data[q3_index].1;

    (q1, q3)
}


fn calculate_bounds(q1_q3: &(f32, f32)) -> (f32, f32)
{
    // Calculate the lower and upper bounds for outliers
    let iqr = q1_q3.1.clone() - q1_q3.0.clone();
    let lower_bound = q1_q3.0.clone() - (1.5 * iqr);
    let upper_bound = q1_q3.1.clone() + (1.5 * iqr);

    (lower_bound, upper_bound)
}