use leptos::*;
use leptos_meta::*;
use tailwind_fuse::*;

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

#[component]
pub fn ListDecoration() -> impl IntoView {
    view! {
           <svg viewBox="0 0 14 6" class="w-10 inline-flex fill-none stroke-1 stroke-orange hover:stroke-orange-700" >
        <path class="cls-1" d="M8.09,4.89c1.21,0,2.2-.98,2.2-2.2s-.98-2.2-2.2-2.2-2.2.98-2.2,2.2.98,2.2,2.2,2.2Z"/>
        <line class="cls-1" x1="5.5" y1="2.7" y2="2.7"/>
      </svg>
    }
}

#[component]
pub fn H3(#[prop(into)] txt: String) -> impl IntoView {
    view! {

      <div class="lg:pl-10 pl-4 mt-10 bg-orange">
        <h3 class=tw_merge!("uppercase","w-fit","px-4","bg-white","text-xl", "font-bold", "tracking-tight", "text-orange", "sm:text-2xl","leading-none")>
        {txt}
        </h3>
      </div>
    }
}

#[component]
pub fn H4(children: Children) -> impl IntoView {
    view! {
      <h4 class="pt-4 uppercase text-orange font-semibold">{children()}</h4>
    }
}

#[component]
pub fn CardWireframe() -> impl IntoView {
    view! {
            <svg class="w-24 h-26 inline-flex" viewBox="0 0 30 35">
              <rect class="stroke-1 stroke-gray fill-none" x=".38" y=".38" width="17.77" height="25.45"/>
            </svg>
    }
}
#[component]
pub fn TwoCardWireframe() -> impl IntoView {
    view! {
            <svg class="w-24 h-26 inline-flex" viewBox="0 0 30 35">
              <rect class="stroke-1 stroke-gray fill-none" x=".38" y=".38" width="17.77" height="25.45"/>
              <rect class="fill-white" x="-.94" y="6.34" width="25.45" height="17.77" transform="translate(-5.56 23.58) rotate(-78)"/>
              <rect class="cls-stroke-1 stroke-gray fill-none" x="-.94" y="6.34" width="25.45" height="17.77" transform="translate(-5.56 23.58) rotate(-78)"/>
            </svg>
    }
}

#[component]
pub fn ThreeCardWireframe() -> impl IntoView {
    view! {
            <svg class="w-24 h-26 inline-flex" viewBox="0 0 30 35">
              <rect class="stroke-1 stroke-gray fill-none" x=".38" y=".38" width="17.77" height="25.45"/>
              <rect class="fill-white" x="-.94" y="6.34" width="25.45" height="17.77" transform="translate(-5.56 23.58) rotate(-78)"/>
              <rect class="cls-stroke-1 stroke-gray fill-none" x="-.94" y="6.34" width="25.45" height="17.77" transform="translate(-5.56 23.58) rotate(-78)"/>
              <rect class="fill-white" x="1.78" y="9.18" width="25.45" height="17.77" transform="translate(-8.05 23.38) rotate(-64.5)"/>
              <rect class="stroke-1 stroke-gray fill-none" x="1.78" y="9.18" width="25.45" height="17.77" transform="translate(-8.05 23.38) rotate(-64.5)"/>
            </svg>
    }
}
