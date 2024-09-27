use std::collections::HashMap;

use plotters::backend::BitMapBackend;
use plotters::chart::{ChartBuilder, SeriesLabelPosition};
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::element::Circle;
use plotters::style::{Color, Palette, PaletteColor, RGBColor, BLACK, WHITE};

use crate::data::sample::key::SerieKey;




pub(crate) struct CustomPalette;


impl Palette for CustomPalette {
    const COLORS: &'static [(u8, u8, u8)] = &[
        (230, 25, 75),
        (60, 180, 75),
        (255, 225, 25),
        (0, 130, 200),
        (245, 130, 48),
        (145, 30, 180),
        (70, 240, 240),
        (240, 50, 230),
        (210, 245, 60),
        (250, 190, 190),
        (0, 128, 128),
        (230, 190, 255),
        (170, 110, 40),
        (255, 250, 200),
        (128, 0, 0),
        (170, 255, 195),
        (128, 128, 0),
        (255, 215, 180),
        (0, 0, 128),
        (128, 128, 128),
        (0, 0, 0),
    ];
}





/// draw the legend on the given drawing area
pub(crate) fn write_legend<Key> (
    label_drawing_area: &DrawingArea<BitMapBackend<'_>, Shift>,
    legend_to_color : &HashMap<String, PaletteColor<CustomPalette>>,
    legend_serie_key : &Option<Key>
) -> Result<(), Box<dyn std::error::Error>>
where 
    Key : SerieKey,
{
    // draw the legend on a fantome chart
    let mut label_chart = 
    ChartBuilder::on(label_drawing_area)
    .margin(5)
    .build_cartesian_2d(0..1, 0..1)?;

    label_chart
        .configure_mesh()
        .disable_mesh()
        .draw()?;




    let dummy_data : Vec<(i32, i32)> = Vec::new();

    // draw phantome serie to get the legend

    // begin by the first one indicating the key used
    if let Some(legend_serie_key) = legend_serie_key {
        let color = RGBColor(255, 255, 255).mix(0.0);
        
        let serie_unlabellized = label_chart
                .draw_series(
                    dummy_data.iter()
                        .map(|(x, y)| Circle::new((*x, *y), 2, color.filled())),
            )?;
        serie_unlabellized.label(legend_serie_key.get_display_name()).legend(move |(x, y)| {
            Circle::new((x, y), 5, color.filled())
        });
    }

    
    let mut unique_legends = legend_to_color.keys().collect::<Vec<_>>();
    if legend_serie_key.is_some() && legend_serie_key.unwrap().is_numeric(){
        unique_legends.sort_by(|a, b| a.parse::<f32>().unwrap().partial_cmp(&b.parse::<f32>().unwrap()).unwrap());
    }

    for legend in unique_legends.iter() {
        // skip empty legend
        if legend == &"" {
            continue;
        }
        let color = legend_to_color.get(*legend).unwrap();
        let serie_unlabellized = label_chart
                .draw_series(
                    dummy_data.iter()
                        .map(|(x, y)| Circle::new((*x, *y), 2, color.filled())),
            )?;
            serie_unlabellized.label(legend.to_string()).legend(move |(x, y)| {
                Circle::new((x, y), 5, color.filled())
            });
    }

    label_chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .position(SeriesLabelPosition::MiddleMiddle)
        .draw()?;

    Ok(())
}


pub(crate) fn format_number_f32(n: &f32) -> String {
    format_number(*n as f64)
}

/// Format a number to a string
pub fn format_number(n: f64) -> String {
    if n == 0.0 {
        return "0".to_string();
    }

    let abs_n = n.abs();
    let sign = if n < 0.0 { "-" } else { "" };

    if abs_n >= 0.001 && abs_n < 10000.0 {
        // For numbers that don't need scientific notation
        
        let formatted = format!("{:.4}", abs_n);
        let limited = formatted.chars().take(5).collect::<String>(); // take only 5 characters (4 digits + 1 decimal point)
        let trimmed = limited.trim_end_matches('0').trim_end_matches('.');
        format!("{}{}", sign, trimmed)
    } else {
        // For numbers that require scientific notation
        let formatted = format!("{:.3e}", abs_n);
        let parts: Vec<&str> = formatted.split('e').collect();
        let mantissa = parts[0].trim_end_matches('0').trim_end_matches('.');
        let exponent = parts[1]; // Keep the exponent part as-is
        format!("{}{}e{}", sign, mantissa, exponent)
    }
}

/// Format a time in seconds to a string
pub fn format_duration(seconds: f64) -> String {
    // Total number of seconds in a day, hour, and minute
    const SECONDS_IN_A_MINUTE: f64 = 60.0;
    const SECONDS_IN_AN_HOUR: f64 = SECONDS_IN_A_MINUTE * 60.0;
    const SECONDS_IN_A_DAY: f64 = SECONDS_IN_AN_HOUR * 24.0;

    // Calculate days, hours, minutes, and seconds
    let days = (seconds / SECONDS_IN_A_DAY).floor();
    let remaining_after_days = seconds % SECONDS_IN_A_DAY;

    let hours = (remaining_after_days / SECONDS_IN_AN_HOUR).floor();
    let remaining_after_hours = remaining_after_days % SECONDS_IN_AN_HOUR;

    let minutes = (remaining_after_hours / SECONDS_IN_A_MINUTE).floor();
    let remaining_seconds = remaining_after_hours % SECONDS_IN_A_MINUTE;

    // Build the formatted string conditionally
    let mut result = String::new();

    if days > 0.0 {
        result.push_str(&format!("{}d", days as u32));
    }

    if hours > 0.0 || days > 0.0 {
        if !result.is_empty() {
            result.push_str(" ");
        }
        result.push_str(&format!("{}h", hours as u32));
    }

    if minutes > 0.0 || hours > 0.0 || days > 0.0 {
        if !result.is_empty() {
            result.push_str(" ");
        }
        result.push_str(&format!("{}m", minutes as u32));
    }

    if !result.is_empty() {
        result.push_str(" ");
    }

    result.push_str(&format!("{}s", format_number(remaining_seconds)));

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        // Test for zero
        assert_eq!(format_number(0.0), "0");

        // Test small numbers
        assert_eq!(format_number(0.123), "0.123");
        assert_eq!(format_number(0.0123), "0.012");
        assert_eq!(format_number(0.00123), "0.001");
        assert_eq!(format_number(0.00012345), "1.234e-4");

        // Test numbers in tens and hundreds
        assert_eq!(format_number(10.0), "10");
        assert_eq!(format_number(100.0), "100");
        assert_eq!(format_number(999.09), "999");
        assert_eq!(format_number(99.9), "99.9");
        assert_eq!(format_number(999.99), "999.9");

        // Test numbers in thousands
        assert_eq!(format_number(1000.0), "1000");
        assert_eq!(format_number(10000.0), "1e4");
        // assert_eq!(format_number(999999.0), "9.999e5"); -> error of precision

        // Test large numbers in scientific notation
        assert_eq!(format_number(1e6), "1e6");
        assert_eq!(format_number(1.234e9), "1.234e9");
        assert_eq!(format_number(1.2345e9), "1.234e9");

        // Test negative numbers
        assert_eq!(format_number(-123.45), "-123.4");
        assert_eq!(format_number(-0.000123), "-1.23e-4");
    }

    #[test]
    fn test_format_duration() {
        // Test for whole days
        assert_eq!(format_duration(86400.0), "1d 0h 0m 0s");

        // Test for days and hours
        assert_eq!(format_duration(90000.0), "1d 1h 0m 0s");

        // Test for hours and minutes
        assert_eq!(format_duration(3661.0), "1h 1m 1s");

        // Test for seconds only
        assert_eq!(format_duration(59.5), "59.5s");

        // Test for fractional seconds
        assert_eq!(format_duration(0.123), "0.123s");

        // Test for small time in milliseconds
        assert_eq!(format_duration(0.001), "0.001s");

        // Test for multiple days and hours
        assert_eq!(format_duration(172800.0), "2d 0h 0m 0s");
        assert_eq!(format_duration(172872.0), "2d 0h 1m 12s");
    }
}