use leptos::*;
use leptos_meta::*;

use crate::web::plausible::components::A;

#[component]
pub fn Button(
    #[prop(into)] title: String,
    #[prop(into, default = "green".into())] color: String,
    #[prop(into, default = "".into())] class: String,
) -> impl IntoView {
    view! {
      <button class=format!(
          "flex w-full justify-center rounded-md bg-{color}-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-{color}-500 disabled:opacity-75 {class}",
      )>{title}</button>
    }
}

#[component]
pub fn ButtonDisablable(title: String, disabled: Signal<bool>) -> impl IntoView {
    view! {
      <button
        class="py-2 px-4 ml-2 font-bold text-white bg-orange-500 rounded focus:outline-none disabled:bg-orange-900 disabled:opacity-50 enabled:hover:bg-orange-700 focus:shadow-outline"
        class=("cursor-not-allowed", disabled)
        disabled=disabled
      >
        {title}
      </button>
    }
}

#[component]
pub fn ButtonLink(
    #[prop(into)] title: String,
    #[prop(into)] href: String,
    #[prop(into, default = "green".into())] color: String,
) -> impl IntoView {
    view! {
      <A
        href=href
        class=format!(
            "flex w-full justify-center rounded-md bg-{color}-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-{color}-500",
        )
      >

        {title}
      </A>
    }
}

#[component]
pub fn ButtonLinkSecond(
    #[prop(into)] title: String,
    #[prop(into)] href: String,
    #[prop(into, default = "_self".into())] target: String,
    #[prop(into, default = "green".into())] color: String,
    #[prop(into, default = "".into())] class: String,
) -> impl IntoView {
    view! {
      <A
        href=href
        target=target
        class=format!(
            "flex w-full justify-center rounded-md border-2 border-{color} px-3 py-1.5 text-sm text-color-{color} font-semibold leading-6 shadow-sm hover:border-{color}-900 hover:bg-{color}-100 my-2 {class}",
        )
      >

        {title}
      </A>
    }
}

#[component]
pub fn BuiltWith() -> impl IntoView {
    view! {
      <footer>
        <div class="my-4 text-center text-gray-700">
          "Built with 💜, linux, vim, 🦀, leptos and tailwindCSS"
        </div>
      </footer>
    }
}
