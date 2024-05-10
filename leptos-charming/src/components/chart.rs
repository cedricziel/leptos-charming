use charming::{Chart, WasmRenderer};
use leptos::{create_effect, html::Div, *};
use web_sys::HtmlDivElement;

use web_sys::wasm_bindgen::JsCast;

#[derive(Debug, Clone, Copy)]
pub struct CharmingChartContext {
    chart: RwSignal<Option<charming::Chart>>,
}

impl CharmingChartContext {
    pub fn new() -> Self {
        Self {
            chart: create_rw_signal(None),
        }
    }

    pub fn set_chart(&self, chart: &charming::Chart) {
        self.chart.set(Some(chart.clone()));
    }

    pub fn chart(&self) -> Option<charming::Chart> {
        self.chart.get()
    }

    pub fn chart_signal(&self) -> ReadSignal<Option<charming::Chart>> {
        self.chart.read_only()
    }
}

impl Default for CharmingChartContext {
    fn default() -> Self {
        Self::new()
    }
}

pub fn provide_charming_chart_context() -> CharmingChartContext {
    let ctx = CharmingChartContext::new();
    provide_context(ctx);

    ctx
}

#[allow(non_snake_case)]
#[component]
pub fn Chart(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(into, optional)] style: MaybeSignal<String>,
    #[prop(optional)] chart: Option<WriteSignal<Option<Chart>>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let chart_ref = create_node_ref::<Div>();
    let chart_ctx = provide_charming_chart_context();

    let chart_load = chart_ref;
    chart_load.on_load(move |chart_div| {
        let html_node = chart_div.unchecked_ref::<HtmlDivElement>();

        if html_node.id().is_empty() {
            let id = format!("chart-{}", rand::random::<u64>());
            _ = chart_div.clone().id(id);
        }

        let renderer = WasmRenderer::new(600, 400);

        _ = chart_div.on_mount(move |chart_div| {
            let chart_div = chart_div.unchecked_ref::<HtmlDivElement>();

            let charming_chart = Chart::new();

            renderer
                .render(&chart_div.id().to_string(), &charming_chart)
                .unwrap();

            chart_ctx.set_chart(&charming_chart);
            if let Some(chart) = chart {
                chart.set(Some(charming_chart));
            }
        });
    });

    view! {
        <div class=move || class.get() node_ref=chart_ref style=move || style.get()>
            {children.map(|child| child())}
        </div>
    }
}

#[allow(non_snake_case)]
#[component(transparent)]
pub fn Title(#[prop(into)] text: String) -> impl IntoView {
    let chart_context = use_context::<CharmingChartContext>().expect("chart context not found");

    create_effect(move |_| {
        if let Some(chart) = chart_context.chart() {
            let title = charming::component::Title::new().text(&text);

            chart.title(title);
        }
    });
}
