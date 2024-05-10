use charming::element::AxisType;
use leptos::*;
use leptos_charming::*;

#[component]
pub fn App() -> impl IntoView {
    let axis_type = AxisType::Category;
    let axis_data: MaybeSignal<Vec<String>> = MaybeSignal::derive(move || {
        vec![
            "Mon".to_string(),
            "Tue".to_string(),
            "Wed".to_string(),
            "Thu".to_string(),
            "Fri".to_string(),
            "Sat".to_string(),
            "Sun".to_string(),
        ]
    });

    view! {
        <Chart style="height: 400px">
            <Title text="My Chart"/>
            <XAxis type_=axis_type data=axis_data/>
            <YAxis type_=AxisType::Value/>
            <LineSeries/>
        </Chart>
    }
}
