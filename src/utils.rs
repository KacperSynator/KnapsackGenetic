use anyhow::Error;
use num_traits::Num;
use plotters::prelude::*;
use std::cmp::min;
use std::convert::From;

use crate::genetic_algorithm::GeneticAlgorithmResultData;

pub struct GraphData<'a> {
    pub out_file: &'a str,
    pub graph_size: (u32, u32),
    pub title: &'a str,
    pub margin: u32,
    pub x_label_area_size: u32,
    pub y_label_area_size: u32,
}

impl<'a> Default for GraphData<'a> {
    fn default() -> GraphData<'a> {
        GraphData {
            out_file: "out.png",
            graph_size: (640, 480),
            title: "",
            margin: 5,
            x_label_area_size: 30,
            y_label_area_size: 30,
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
    let min_y = min(
        T::default(),
        data.score_per_generation.iter().min().unwrap().clone(),
    );
    let max_y = data.score_per_generation.iter().max().unwrap().clone();

    let root = BitMapBackend::new(&graph_data.out_file, graph_data.graph_size).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(graph_data.title, ("sans-serif", 50).into_font())
        .margin(graph_data.margin)
        .x_label_area_size(graph_data.x_label_area_size)
        .y_label_area_size(graph_data.y_label_area_size)
        .build_cartesian_2d(
            0.0f32..data.score_per_generation.len() as f32,
            f64::from(min_y) as f32..(f64::from(max_y) * 1.2) as f32,
        )?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        data.score_per_generation
            .iter()
            .enumerate()
            .map(|(x, y)| (x as f32, f64::from((*y).clone()) as f32)),
        &RED,
    ))?;

    root.present()?;

    Ok(())
}
