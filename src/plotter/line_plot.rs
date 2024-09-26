use std::collections::HashMap;
use std::path::Path;

use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::series::LineSeries;
use plotters::style::{Color, IntoFont, Palette, PaletteColor, WHITE};

use crate::data::filtering::Filters;
use crate::data::plottable::Plottable;
use crate::data::sample::key::SerieKey;
use crate::data::sample::Sample;
use crate::params::{FIGURE_CAPTION_FONT_SIZE, LABEL_HORIZONTAL_SIZE, ONE_FIG_SIZE};
use crate::stat::stats_serie::MetricName;

use super::layout::Layout;
use super::plot_data::PlotData;
use super::utils::{axe_number_formater, write_legend, CustomPalette};







/// plot the given data as a line
/// take a list of series to plot, to the format (x_serie_key, y_serie_key, filter)
/// If filter is Some, the data will be filtered by the given key and the given function (true to keep the data))
/// NOTE : the number of series to plot must be equal to the number of subplots
/// NOTE : If remove_outliers is Some, the outliers will be removed from the data with the given key
/// NOTE : The aggregation_metrics is the metric used to aggregate the data with the same x value
pub fn line_plot<'plot_lt, S, Key, Plot>(
    data : &'plot_lt Plot, 
    legend_serie_key : Option<Key>,
    save_path : &str,
    layout : &Layout,

    series : Vec<(Key, Option<Key>, Option<&'plot_lt Filters<Key>>)>,
    
    remove_outlier : bool,
    aggregation_metric : MetricName,
) -> Result<(), Box<dyn std::error::Error>> 
where
    Key : SerieKey,
    S : Sample<Key>,
    Plot : Plottable<S, Key>,
    for<'a> &'a Plot: IntoIterator<Item = S>,
{
    if series.len() != layout.get_nb_of_subplots() {
        panic!("The number of series to plot ({}) is not equal to the number of subplots ({})", series.len(), layout.get_nb_of_subplots());
    }

    // initialise the plotter
    let image_path_o = Path::new(save_path);
    // (w, h)
    let global_size = (layout.width as u32 * ONE_FIG_SIZE.0 + LABEL_HORIZONTAL_SIZE, layout.height as u32 * ONE_FIG_SIZE.1);

    // global drawing
    let root_drawing_area = BitMapBackend::new(image_path_o, global_size).into_drawing_area();
    root_drawing_area.fill(&WHITE)?;
    // isolate the label area
    let (chart_drawing_area, label_drawing_area) = 
        root_drawing_area.split_horizontally(global_size.0 - LABEL_HORIZONTAL_SIZE);

    // get the drawing area for each subplot (row, col)
    let child_drawing_areas = chart_drawing_area.split_evenly(layout.get_plotter_layout());
    
    // associate each legend to a color
    let mut legend_to_color : HashMap<String, PaletteColor<CustomPalette>> = HashMap::new();
    let mut legend_index = 0;

    // plot each serie
    for (_, (
            (x_serie_key, y_serie_key, filters), root)
        )
    in series.into_iter().zip(child_drawing_areas.iter()).enumerate() {
        
        
        // get the data
        let data_it = data.into_iter_with_filter(
            (x_serie_key, y_serie_key), 
            legend_serie_key.clone(), 
            filters
        );
        let plot_data = PlotData::from_it(data_it, Some(aggregation_metric.clone()), remove_outlier);

        // define the chart
        let (range_x, range_y) = plot_data.get_range();

        let y_serie_name = if let Some(y_serie_key) = y_serie_key {
            y_serie_key.get_display_name()
        } else {
            "count".to_string()
        };

        let caption = format!("{} per {}", y_serie_name, x_serie_key.get_display_name());
        let mut chart = ChartBuilder::on(&root)
            .caption(caption.as_str(), ("sans-serif", FIGURE_CAPTION_FONT_SIZE).into_font())
            .margin(5)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(range_x, range_y)?;

        chart.configure_mesh()
            .x_desc(x_serie_key.get_display_name().as_str())
            .y_desc(y_serie_name.as_str())
            .x_label_formatter(&axe_number_formater)
            .y_label_formatter(&axe_number_formater)
            .draw()?;


        // plot the data
        for (legend, data_for_legend) in plot_data.into_iter() {
            let color = 
                legend_to_color.entry(legend.to_string())
                    .or_insert_with(|| { 
                        legend_index += 1; 
                        CustomPalette::pick(legend_index - 1) 
                    });

            chart
                .draw_series(
                    LineSeries::new(
                        data_for_legend.iter().map(|(x, y)| (*x, *y)),
                        color.filled().stroke_width(1),
                    )
                )?;
        }
    }// end of for each serie

    write_legend(&label_drawing_area, &legend_to_color, &legend_serie_key)?;

    root_drawing_area.present()?;


    Ok(())
}