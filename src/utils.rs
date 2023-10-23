use anyhow::Error;
use num_traits::Num;
use plotters::prelude::*;
use std::convert::From;

use crate::genetic_algorithm::GeneticAlgorithmResultData;

const DEFAULT_FONT: &str = "sans-serif";
const DEFAULT_FONT_SIZE: i32 = 30;
const X_DESC: &str = "Generation";
const Y_DESC: &str = "Score";
const LEGEND_LABEL: &str = "Score for generation";
const OPTIMAL_LEGEND_LABEL: &str = "Optimal score";

pub struct GraphData<'a> {
    pub out_file: &'a str,
    pub graph_size: (u32, u32),
    pub title: &'a str,
    pub margin: u32,
    pub x_label_area_size: u32,
    pub y_label_area_size: u32,
    pub y_max_value: Option<f32>,
    pub optimal_value_line: Option<f32>,
}

impl<'a> Default for GraphData<'a> {
    fn default() -> GraphData<'a> {
        GraphData {
            out_file: "out.png",
            graph_size: (640, 480),
            title: "",
            margin: 5,
            x_label_area_size: 30,
            y_label_area_size: 50,
            y_max_value: None,
            optimal_value_line: None,
        }
    }
}

pub fn plot_graph<T>(
    data: &GeneticAlgorithmResultData<T>,
    graph_data: &GraphData,
) -> Result<(), Error>
where
    T: Num + Clone + Ord + Default,
    f64: From<T>,
{
    let min_y = f32::min(
        0.0,
        to_f32(data.score_per_generation.iter().min().unwrap().clone()),
    );
    let max_y = graph_data
        .y_max_value
        .unwrap_or(1.2 * to_f32(data.score_per_generation.iter().max().unwrap().clone()));

    let root = BitMapBackend::new(&graph_data.out_file, graph_data.graph_size).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            graph_data.title,
            (DEFAULT_FONT, DEFAULT_FONT_SIZE).into_font(),
        )
        .margin(graph_data.margin)
        .x_label_area_size(graph_data.x_label_area_size)
        .y_label_area_size(graph_data.y_label_area_size)
        .build_cartesian_2d(0.0f32..data.score_per_generation.len() as f32, min_y..max_y)?;

    chart
        .configure_mesh()
        .x_desc(X_DESC)
        .y_desc(Y_DESC)
        .draw()?;

    if let Some(optimal_score) = graph_data.optimal_value_line {
        chart
            .draw_series(LineSeries::new(
                (0..data.score_per_generation.len()).map(|x| (x as f32, optimal_score)),
                &GREEN,
            ))?
            .label(OPTIMAL_LEGEND_LABEL)
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
    }

    chart
        .draw_series(LineSeries::new(
            data.score_per_generation
                .iter()
                .enumerate()
                .map(|(x, y)| (x as f32, to_f32((*y).clone()))),
            &RED,
        ))?
        .label(LEGEND_LABEL)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

fn to_f32<T>(value: T) -> f32
where
    f64: From<T>,
{
    f64::from(value) as f32
}
