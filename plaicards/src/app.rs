use crate::error_template::{AppError, ErrorTemplate};
use crate::web::board::Board;
use crate::web::common::BuiltWith;
use crate::web::landing::{HomePage, Showcase};
use crate::web::lobby::Lobby;
use crate::web::plausible::components::{provide_plausible_context, EndPage, PageView};

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
    provide_meta_context();
    provide_plausible_context();
    leptos_fluent! {{
        locales: "./locales",
        translations: TRANSLATIONS,

        // Client side options
        // -------------------
        sync_html_tag_lang: true,
        initial_language_from_url_param: true,
        initial_language_from_url_param_to_localstorage: true,
        set_language_to_url_param: true,
        initial_language_from_localstorage: true,
        set_language_to_localstorage: true,
        initial_language_from_navigator: true,

    }};

    view! {
      <Stylesheet id="leptos" href="/pkg/plaicards.css"/>
      <MetaInfo />

      <Router
        fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }

        trailing_slash=TrailingSlash::Redirect
      >
        <main>
      <PageView/>
          <Routes>
            <Route path="/" view=HomePage/>
            <Route path="/cards" view=Showcase/>
          // <Route path="/lobby/:id/:player_id" view=move || view!{ <Lobby/>} />
          // <Route path="/plai/:id/:player_id" view=move || view!{ <Board/>} />
          </Routes>
        </main>
        <BuiltWith/>
        <EndPage />
      </Router>
    }
}

#[component]
fn MetaInfo() -> impl IntoView {
    let description = "Burned-out of your fast-paced environment? Too much workplace politics? Get ready for PLAI, the board game where you'll now be the CEO of your startup dedicated to... well, do AI things. And data. And also some Quantum stuff. I don't know really but actually your goal is to reign over your friends and fools! Muahahahah";
    let img = "https://get.plai.cards/img/portada.png";
    let keywords = "plai, board game, tech industry, strategy game, cards, satire, kickstarter, ai, data, startup, comedy";
    let url = "https://get.plai.cards/";
    let title = "PLAI - the Board game for tech workers";
    view! {

      <Title text={title}/>

        <Meta charset="utf-8"/>
        <Meta
          name="description"
          content={description} />
        <Meta
          name="keywords"
          content={keywords}
        />
        <Meta name="HandheldFriendly" content="True"/>
        <Meta property="og:site_name" content="plAI card game"/>

        <Meta property="og:site_name" content="plAI card game"/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:title" content={title}/>
        <Meta
          property="og:description"
          content={description}
        />
        <Meta property="og:url" content={url}/>
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content={title}/>
        <Meta
          name="twitter:description"
          content={description}
        />
        <Meta name="twitter:url" content={url}/>
        <Meta name="twitter:image" content={img}/>
        <Meta property="og:image" content={img}/>
        <Meta name="twitter:label1" content="Written by"/>
        <Meta name="twitter:data1" content="Marti"/>
        <Meta name="twitter:site" content="@plai_cards"/>
        <Meta property="og:image:width" content="1552"/>
        <Meta property="og:image:height" content="873"/>
    }
}
