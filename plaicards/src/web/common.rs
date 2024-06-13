use leptos::*;
use leptos_meta::*;
use leptos_router::A;

#[component]
pub fn Button(
    #[prop(into)] title: String,
    #[prop(into, default = "green".into())] color: String,
    #[prop(into, default = "".into())] class: String,
) -> impl IntoView {
    view! {

        <button class=format!("flex w-full justify-center rounded-md bg-{color}-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-{color}-500 disabled:opacity-75 {class}")>
            {title}
        </button>
    }
}

#[component]
pub fn ButtonDisablable(title: String, disabled: Signal<bool>) -> impl IntoView {
    view! {

        <button class="focus:shadow-outline rounded bg-orange-500 px-4 ml-2 py-2 font-bold text-white enabled:hover:bg-orange-700 focus:outline-none disabled:bg-orange-900 disabled:opacity-50"
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
        <a href={href} class=format!("flex w-full justify-center rounded-md bg-{color}-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-{color}-500")>
            {title}
        </a>
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
        <A href={href} target={target} class=format!("flex w-full justify-center rounded-md border-2 border-{color} px-3 py-1.5 text-sm text-color-{color} font-semibold leading-6 shadow-sm hover:border-{color}-900 hover:bg-{color}-100 my-2 {class}")>
            {title}
        </A>
    }
}
