use leptos::*;
use leptos_meta::*;

#[component]
pub fn Button(title: String) -> impl IntoView {
    view! {
        <button class="bg-amber-600 hover:bg-amber-800 mx-3 px-5 py-3 text-white rounded-lg">
            {title}
        </button>
    }
}

#[component]
pub fn ButtonLink(title: String, href: String) -> impl IntoView {
    view! {
        <a href={href} class="bg-amber-600 hover:bg-amber-800 mx-3 px-5 py-3 text-white rounded-lg">
            {title}
        </a>
    }
}
