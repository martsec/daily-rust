use leptos::*;
use leptos_meta::*;

#[component]
pub fn Button(title: String) -> impl IntoView {
    view! {

        <button class="flex w-full justify-center rounded-md bg-emerald-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-emerald-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 disabled:opacity-75">
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
pub fn ButtonLink(title: String, href: String) -> impl IntoView {
    view! {
        <a href={href} class="flex w-full justify-center rounded-md bg-emerald-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-emerald-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
            {title}
        </a>
    }
}
