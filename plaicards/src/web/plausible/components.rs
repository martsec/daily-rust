use leptos::ev::{keydown, load, visibilitychange};
use wasm_bindgen::JsCast;
use web_sys::{window, Event, HtmlAnchorElement, MouseEvent};

use leptos::*;
use leptos_use::*;

use crate::web::plausible::Plausible;

use leptos::html::{Div, Input};
use leptos::logging::{debug_warn, log};
use leptos_router::A as ARouter;

use super::experiments::{use_experiment_props, Experiment, ExperimentCtx};

pub fn provide_plausible_context() {
    let tracking = Plausible::new_private("test", "https://frumentarii.8vi.cat");
    provide_context(tracking);
}

#[must_use]
pub fn expect_plausible_context() -> Plausible {
    expect_context::<Plausible>()
}

#[must_use]
#[component]
pub fn PageView() -> impl IntoView {
    let el = create_node_ref::<Div>();
    let is_visible = use_element_visibility(el);
    let triggered_pageview = create_rw_signal(false);

    create_effect(move |_| {
        if is_visible() && !triggered_pageview() {
            expect_plausible_context().pageview().send_local();
            triggered_pageview.set(true);
        }
    });

    view! { <div node_ref=el></div> }
}

#[must_use]
#[component]
pub fn TrackElement(
    #[prop(into)] name: String,
    #[prop(into, default = false)] allow_duplicates: bool,
) -> impl IntoView {
    let el = create_node_ref::<Div>();
    let is_visible = use_element_visibility(el);
    let triggered = create_rw_signal(false);

    let n = name.clone();
    create_effect(move |_| {
        if is_visible() && !triggered() {
            let nam = name.clone();
            expect_plausible_context().event(&nam).send_local();
            triggered.set(true);
        }
    });

    view! { <div node_ref=el></div> }
}

/// Substitute for `<a>` and `<A>` that tracks the links to plausible
#[must_use]
#[cfg_attr(
    any(debug_assertions, feature = "ssr"),
    tracing::instrument(level = "trace", skip_all,)
)]
#[component]
pub fn A(
    #[prop(into)] href: String,
    #[prop(into, default = "_self".into())] target: String,
    #[prop(optional, into)] class: Option<AttributeValue>,
    #[prop(optional, into)] id: Option<Oco<'static, str>>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    /// The nodes or elements to be shown inside the link.
    children: Children,
) -> impl IntoView {
    let handle = move |ev: MouseEvent| {
        ev.prevent_default();

        if let Some(target) = ev.target() {
            if let Some(anchor) = target.dyn_ref::<HtmlAnchorElement>() {
                let url = anchor.href();

                // FIXME it does not find the experiment context
                expect_plausible_context().link_click(&url).send_local();

                let target = anchor.target();

                // Navigate to the new URL
                if let Some(window) = window() {
                    match target.as_str() {
                        "_blank" => {
                            window
                                .open_with_url(&url)
                                .expect("Failed to open in a new tab");
                        }
                        _ => {
                            window
                                .location()
                                .set_href(&url)
                                .expect("Failed to navigate");
                        }
                    };
                }
            }
        }
    };

    view! {
      <ARouter
        href=href
        target=target
        class=class
        // id=id
        on:click=handle
      >
        {children()}
      </ARouter>
    }
}
