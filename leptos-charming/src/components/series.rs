use charming::series::Line;
use leptos::*;

use crate::chart::CharmingChartContext;

#[component(transparent)]
pub fn LineSeries() -> impl IntoView {
    let chart_context = use_context::<CharmingChartContext>().expect("chart context not found");

    create_effect(move |_| {
        if let Some(chart) = chart_context.chart() {
            let series = Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]);

            chart.series(series);
        }
    });
}
