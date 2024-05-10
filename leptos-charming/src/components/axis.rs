use charming::element::AxisType;
use leptos::*;

use crate::chart;

#[allow(non_snake_case)]
#[component(transparent)]
pub fn XAxis(
    #[prop(into)] type_: AxisType,
    #[prop(optional)] data: MaybeSignal<Vec<String>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let chart_context =
        use_context::<chart::CharmingChartContext>().expect("chart context not found");

    let data_for_effect = data.clone();

    create_effect(move |_| {
        if let Some(chart) = chart_context.chart() {
            let axis = charming::component::Axis::new()
                .type_(type_.clone())
                .data(data_for_effect.get_untracked());

            chart.x_axis(axis);
        }
    });

    children.map(|child| child())
}

#[allow(non_snake_case)]
#[component(transparent)]
pub fn YAxis(
    #[prop(into)] type_: AxisType,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let chart_context =
        use_context::<chart::CharmingChartContext>().expect("chart context not found");

    create_effect(move |_| {
        if let Some(chart) = chart_context.chart() {
            let axis = charming::component::Axis::new().type_(type_.clone());

            chart.y_axis(axis);
        }
    });

    children.map(|child| child())
}
