//! The sizes Read Mode lays its columns out at, which come from the body text's measure: the app reaches the text
//! layout only inside a component's `measure`, so a zero-size probe component reads it there.

use craft::behavior::{Input, Metrics, Sense};
use craft::pages::ui::reading::ReadingMetrics;
use craft::ui::contract::{Common, Component, Cx, MeasureCx, NoValue, Outcome, Props, Role, Show as _};
use craft::ui::primitives::ReadingMeasure;
use craft::ui::style::{ComponentStyle, StyleCx, TypeRole};
use egui::{Response, Ui};

/// Measures the body text's reading widths; shows nothing.
#[derive(Component, Props)]
#[component(canvas_control)]
struct ReadingProbe {
    #[common]
    common: Common,
    measured: Option<ReadingMetrics>,
}

/// What the probe measured this pass.
struct ReadingProbeOutput {
    response: Response,
    metrics: Option<ReadingMetrics>,
}

impl Outcome for ReadingProbeOutput {
    type Value = NoValue;

    fn response(&self) -> &Response {
        &self.response
    }
}

/// The gaps the metrics take from the tokens.
#[derive(Clone, Debug, PartialEq, ComponentStyle)]
struct ReadingProbeStyle {
    #[style(metrics)]
    metrics: Metrics,
    column_gap: f32,
    nav_bar: f32,
}

impl Component for ReadingProbe {
    const ROLE: Role = Role::None;
    const PARTS: bool = false;
    const STOPS: bool = false;
    const INPUT: Input = Input::Display;
    const SHOWS_LABEL: bool = false;
    const OVERLAYS: bool = false;
    type Style = ReadingProbeStyle;
    type Output = ReadingProbeOutput;

    fn style(cx: &StyleCx<'_>, _: ReadingProbeStyleKey) -> ReadingProbeStyle {
        ReadingProbeStyle { metrics: Metrics::new(), column_gap: cx.tokens.space.gap.section, nav_bar: cx.tokens.size.panel_header }
    }

    fn measure(&mut self, cx: &MeasureCx<'_, Self>) -> egui::Vec2 {
        let measure = ReadingMeasure::of(TypeRole::Body, |run| cx.layout(run));
        self.measured = Some(metrics(cx.style.column_gap, cx.style.nav_bar, measure));
        egui::Vec2::ZERO
    }

    fn sense(&self) -> Sense {
        Sense::empty()
    }

    fn finish(self, cx: Cx<'_, Self>) -> ReadingProbeOutput {
        ReadingProbeOutput { response: cx.response.clone(), metrics: self.measured }
    }
}

/// Column widths Read Mode has always used (about one column per 295 screen points, none under 192), as v0.19.1 laid out 4
/// columns at 1440 screen points. craft's body-text measure (66 characters) gives 2 there, so only the gaps come from it.
const COLUMN_TARGET: f32 = 295.0;
const COLUMN_MIN: f32 = 192.0;

fn metrics(column_gap: f32, nav_bar: f32, measure: ReadingMeasure) -> ReadingMetrics {
    let mut m = ReadingMetrics::new(column_gap, nav_bar, measure);
    // lean: fixed px, not the text size; follow the body text again if Read Mode should scale with it.
    m.column_target = COLUMN_TARGET;
    m.column_min = COLUMN_MIN;
    m
}

/// The reading metrics under the current theme, or `None` when the probe did not measure this pass.
pub(super) fn read(ui: &mut Ui) -> Option<ReadingMetrics> {
    ReadingProbe { common: Common::new(), measured: None }.label("Reading metrics").show(ui).metrics
}
