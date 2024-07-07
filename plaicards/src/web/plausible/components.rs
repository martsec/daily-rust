use leptos::ev::{keydown, load, visibilitychange};
use wasm_bindgen::JsCast;
use web_sys::{window, Event, HtmlAnchorElement, MouseEvent};

use leptos::*;
use leptos_use::*;

use crate::web::plausible::Plausible;

use leptos::html::{Div, Input};
use leptos::logging::{debug_warn, log};
use leptos_router::A as ARouter;
use leptos_use::use_active_element;

use super::experiments::{use_experiment_props, Experiment, ExperimentCtx};

/// Sets the plausible context. It should usually be somewhere near the
/// root of your application (Similar to the `<Router />` component).
pub fn provide_plausible_context() {
    let tracking = Plausible::new_private("get.plai.cards", "https://frumentarii.8vi.cat");
    provide_context(tracking);
}

/// Retrieves plausible context
#[must_use]
pub fn expect_plausible_context() -> Plausible {
    expect_context::<Plausible>()
}

/// Sends an event if the user focused on an item with an ID starting with `plausible-`
///
/// It will send an `active_element` event with the property `data-id` with the value
/// without `plausible-`
///
/// For example `plausible-email` becomes `email`
pub fn track_active_elements() {
    let active_element = use_active_element();

    create_effect(move |_| {
        let id = active_element()
            .map(|el| el.dataset().get("id"))
            .unwrap_or_default();
        debug_warn!("Selecting item {:?}", id);

        if let Some(id) = id {
            if id.starts_with("plausible-") {
                expect_plausible_context()
                    .event("active_element")
                    .prop("id", id.replace("plausible-", "").into())
                    .send_local();
            }
        }
    });
}
/// Track a standard page view event.
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

/// Send a custom event when a given part of a webpage is within
/// the viewport of the user.
///
/// Use it to track when somenone has "viewed"/reached a given part of
/// your website.
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

/// Send and `endpage` event. Similar to page view but to know when a
/// visitor reached the end of the page.
#[must_use]
#[component]
pub fn EndPage() -> impl IntoView {
    view! {<TrackElement name="endpage" />}
}

/// Substitute for `<a>` and `<A>` that tracks the links to plausible
// TODO implement id and attributes
// FIXME it does not correctly find the experiment context
#[must_use]
#[component]
pub fn A(
    #[prop(into)] href: String,
    #[prop(into, default = "_self".into())] target: String,
    #[prop(optional, into)] class: Option<AttributeValue>,
    /// The nodes or elements to be shown inside the link.
    children: Children,
) -> impl IntoView {
    // Work around to provide experiment context. Complains of using it outside
    // Suspense despite being inside one!
    use super::experiments::use_experiment;
    let exp = use_experiment();

    let handle = move |ev: MouseEvent| {
        ev.prevent_default();

        if let Some(target) = ev.target() {
            if let Some(anchor) = target.dyn_ref::<HtmlAnchorElement>() {
                let url = anchor.href();

                expect_plausible_context()
                    .link_click(&url)
                    .set_experiment(exp)
                    .send_local();

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
        //id=id
        on:click=handle
      >
        {children()}
      </ARouter>

    }
}
