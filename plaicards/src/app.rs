use crate::error_template::{AppError, ErrorTemplate};
use crate::web::board::Board;
use crate::web::common::BuiltWith;
use crate::web::landing::{HomePage, Showcase};
use crate::web::lobby::Lobby;
use crate::web::plausible::components::PageView;

use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::{expect_i18n, leptos_fluent, move_tr, tr};
use leptos_meta::*;
use leptos_router::*;

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

#[component]
#[must_use]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    leptos_fluent! {{
        locales: "./locales",
        // Static translations struct provided by fluent-templates.
        translations: TRANSLATIONS,
        // Check translations correctness in the specified files.
        //check_translations: "./src/web/**/*.rs",

        // Client side options
        // -------------------
        // Synchronize `<html lang="...">` attribute with the current
        // language using `leptos::create_effect`. By default, it is `false`.
        sync_html_tag_lang: true,
        // URL parameter name to use discovering the initial language
        // of the user. By default is `"lang"`.
        url_param: "lang",
        // Discover the initial language of the user from the URL.
        // By default, it is `false`.
        initial_language_from_url_param: true,
        // Set the discovered initial language of the user from
        // the URL in local storage. By default, it is `false`.
        initial_language_from_url_param_to_localstorage: true,
        // Update the language on URL parameter when using the method
        // `I18n.set_language`. By default, it is `false`.
        set_language_to_url_param: true,
        // Name of the field in local storage to get and set the
        // current language of the user. By default, it is `"lang"`.
        localstorage_key: "language",
        // Get the initial language from local storage if not found
        // in an URL param. By default, it is `false`.
        initial_language_from_localstorage: true,
        // Update the language on local storage when using the method
        // `I18n.set_language`. By default, it is `false`.
        set_language_to_localstorage: true,
        // Get the initial language from `navigator.languages` if not
        // found in the local storage. By default, it is `false`.
        initial_language_from_navigator: true,

    }};

    view! {
      <Stylesheet id="leptos" href="/pkg/plaicards.css"/>

      // sets the document title
      <Title text="PLAI - The baord game for tech workers"/>
      <Meta charset="utf-8"/>
      <Meta
        name="description"
        content="Burned-out of your fast-paced environment? Too much workplace politics? Get ready for PLAI, the board game where you'll now be the CEO of your startup dedicated to... well, do AI things. And data. And also some Quantum stuff. I don't know really but actually your goal is to reign over your friends and fools! Muahahahah"
      />
      <Meta
        name="keywords"
        content="plai, board game, tech industry, strategy game, cards, satire, kickstarter, ai, data, startup, comedy"
      />
      <Meta name="HandheldFriendly" content="True"/>
      <Meta property="og:site_name" content="plAI card game"/>

      <Meta property="og:site_name" content="plAI card game"/>
      <Meta property="og:type" content="website"/>
      <Meta property="og:title" content="plAI the board game for (not only) tech workers"/>
      <Meta
        property="og:description"
        content="Burned-out of your fast-paced environment? Too much workplace politics? Get ready for PLAI, the board game where you'll now be the CEO of your startup dedicated to... well, do AI things. And data. And also some Quantum stuff. I don't know really but actually your goal is to reign over your friends and fools! Muahahahah"
      />
      <Meta property="og:url" content="https://get.plai.cards/"/>
      <Meta name="twitter:card" content="summary_large_image"/>
      <Meta name="twitter:title" content="plAI, the board game for tech workers"/>
      <Meta
        name="twitter:description"
        content="Burned-out of your fast-paced environment? Too much workplace politics? Get ready for PLAI, the board game where you'll now be the CEO of your startup dedicated to... well, do AI things. And data. And also some Quantum stuff. I don't know really but actually your goal is to reign over your friends and fools! Muahahahah"
      />
      <Meta name="twitter:url" content="https://get.plai.cards/"/>
      <Meta name="twitter:image" content="https://get.plai.cards/img/portada.png"/>
      <Meta property="og:image" content="https://get.plai.cards/img/portada.png"/>
      <Meta name="twitter:label1" content="Written by"/>
      <Meta name="twitter:data1" content="Marti"/>
      <Meta name="twitter:site" content="@plai_cards"/>
      <Meta property="og:image:width" content="1552"/>
      <Meta property="og:image:height" content="873"/>

      <PageView/>
      <Router
        fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }

        trailing_slash=TrailingSlash::Redirect
      >
        <main>
          <Routes>
            <Route path="/" view=HomePage/>
            <Route path="/cards" view=Showcase/>
          // <Route path="/lobby/:id/:player_id" view=move || view!{ <Lobby/>} />
          // <Route path="/plai/:id/:player_id" view=move || view!{ <Board/>} />
          </Routes>
        </main>
        <BuiltWith/>
      // <script defer data-domain="get.plai.cards" src="https://frumentarii.8vi.cat/js/script.tagged-events.outbound-links.js"></script>
      </Router>
    }
}
