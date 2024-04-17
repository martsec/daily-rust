use leptos::*;
use leptos_meta::*;

#[component]
pub fn Button(title: String) -> impl IntoView {
    view! {

        <button class="flex w-full justify-center rounded-md bg-emerald-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-emerald-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
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
