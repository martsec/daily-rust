use rand::prelude::*;
use std::collections::HashSet;

use leptos::*;
use leptos_fluent::{expect_i18n, leptos_fluent, move_tr, tr};
use leptos_meta::*;

use super::page::Newsletter;
use crate::web::common::{Button, ButtonLinkSecond};

// Shows a random card
#[component]
pub fn Showcase() -> impl IntoView {
    let (card, set_card) = create_signal(0);
    let seen_cards = create_rw_signal(HashSet::<String>::new());
    provide_context(seen_cards);
    provide_meta_context();
    let total_cards = 7;

    view! {
      <Title text="PLAI - our cards"/>

      <div class="overflow-hidden bg-white py-8 sm:py-8 lg:py-16 md:h-[60rem] content-center">
        <div class="mx-auto max-w-7xl px-6 lg:px-8">

          <div class="mx-auto place-items-center grid max-w-2xl grid-cols-1 gap-y-10 lg:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-2">
            <Suspense fallback=|| ()>
              <Card card=card/>
            </Suspense>
          </div>
        </div>
        <div class="m-8 px-4 mx-auto w-96">
          <p class="m-4 text-center text-gray-400 text-lg">
            {move_tr!(
                "showcase-seen", { "cardsCount" => seen_cards().len(), "total" => total_cards }
            )}

          </p>
          <Button
            title="Draw card"
            color="orange"
            class="plausible-event-name=LandingDrawCard"
            on:click=move |_| {
                let mut rng = rand::thread_rng();
                let num_card: u8 = rng.gen();
                set_card(num_card % total_cards + 1);
            }
          />

        </div>

      </div>

      <Newsletter/>
    }
}

#[component]
fn Deck() -> impl IntoView {
    view! {
      <div class="deck">
        <div class="sized-card card-border bg-card-back bg-cover"></div>
        <div class="sized-card card-border bg-card-back bg-cover"></div>
        <div class="sized-card card-border bg-card-back bg-cover"></div>
        <div class="sized-card card-border bg-card-back bg-cover"></div>
        <div class="sized-card card-border bg-card-back bg-cover"></div>
        <div class="sized-card card-border bg-card-back bg-cover"></div>
      </div>
      <div class="lg:max-w-lg">
        <p class="mt-6 text-xl text-center text-gray-600">{move || tr!("showcase-intro")}</p>
      </div>
    }
}

#[component]
fn Card(#[prop(into)] card: Signal<u8>) -> impl IntoView {
    view! {
      {move || match card() {
          0 => view! { <Deck/> },
          1 => view! { <Daily/> },
          2 => view! { <Data/> },
          3 => view! { <Antitrust/> },
          4 => view! { <Dotcom/> },
          5 => view! { <Criminals/> },
          6 => view! { <ArtCompetitions/> },
          _ => view! { <Hr/> },
      }}
    }
}

#[component]
fn FlipCard(
    #[prop(into)] image: String,
    #[prop(into, default="".into())] class: String,
) -> impl IntoView {
    view! {
      <div class=format!("sized-card flip-card {class}")>
        <div class=format!("sized-card card-border flip-card-front bg-cover bg-{image}")></div>
        <div class="sized-card card-border flip-card-back bg-card-back bg-cover"></div>
      </div>
    }
}

fn update_seen_cards(name: &str) {
    let cards = use_context::<RwSignal<HashSet<String>>>().expect("ERR with card storage");
    cards.update(|cs| {
        cs.insert(name.into());
    });
}

#[component]
fn Daily() -> impl IntoView {
    view! {
      {move || update_seen_cards("daily")}
      <FlipCard image="card-daily" class="animate-fade-slide-in-right"/>
      <div class="lg:max-w-lg animate-fade-slide-in-left">
        <p class="mt-2 text-3xl text-center uppercase text-black">
          {move || tr!("showcase-daily")}
        </p>
        <p class="mt-6 text-xl text-center text-gray-600">
          {move || tr!("showcase-daily.description")}
        </p>
        <p class="mt-6 text-xl text-center text-gray-600">
          {move || tr!("showcase-daily.punchline")}
        </p>
      </div>
    }
}

#[component]
fn Antitrust() -> impl IntoView {
    view! {
      {move || update_seen_cards("antitrust")}
      <FlipCard image="card-antitrust" class="animate-fade-slide-in-right"/>
      <div class="lg:max-w-lg animate-fade-slide-in-left">
        <p class="mt-2 text-3xl text-center uppercase text-black">
          {move || tr!("showcase-antitrust")}
        </p>
        <p class="mt-6 text-xl text-center text-gray-600">
          {move || tr!("showcase-antitrust.description")}
        </p>
        <p class="m-6 text-xl text-center text-gray-600">
          {move || tr!("showcase-antitrust.challenge")}
        </p>
        <ButtonLinkSecond
          title="Ex-FTC Head Joins Silicon Valley Firm"
          href="https://news.bloomberglaw.com/business-and-practice/big-tech-in-spotlight-as-ex-ftc-head-joins-silicon-valley-firm"
          target="_blank"
          color="grey"
        />

        <ButtonLinkSecond
          title="Amazon vs US FTC lawsuit"
          href="https://www.reuters.com/legal/amazon-has-deep-bench-defense-lawyers-fight-us-ftc-lawsuit-2023-09-26/"
          target="_blank"
          color="grey"
        />
      </div>
    }
}

#[component]
fn ArtCompetitions() -> impl IntoView {
    view! {
      {move || update_seen_cards("winart")}
      <FlipCard image="card-winart" class="animate-fade-slide-in-right"/>
      <div class="lg:max-w-lg animate-fade-slide-in-left">
        <p class="mt-2 text-3xl text-center uppercase text-black">
          {move || tr!("showcase-winart")}
        </p>
        <p class="mt-6 text-xl text-center text-gray-600">
          {move || tr!("showcase-winart.description")}
        </p>
        <p class="m-6 text-xl text-center text-gray-600">
          {move || tr!("showcase-winart.challenge")}
        </p>
        <ButtonLinkSecond
          title="Glaze: Protect your art from AI"
          href="https://glaze.cs.uchicago.edu/aboutus.html"
          target="_blank"
          color="green"
        />

        <ButtonLinkSecond
          title="Cara: art social network with Glaze"
          href="https://cara.app/about"
          target="_blank"
          color="green"
        />
      </div>
    }
}
#[component]
fn Criminals() -> impl IntoView {
    view! {
      {move || update_seen_cards("criminals")}
      <FlipCard image="card-criminals" class="animate-fade-slide-in-right"/>
      <div class="lg:max-w-lg animate-fade-slide-in-left">
        <p class="mt-2 text-3xl text-center uppercase text-black">
          {move || tr!("showcase-antitrust")}
        </p>
        <p class="mt-6 text-xl text-center text-gray-600">
          {move || tr!("showcase-criminals.description")}
        </p>
        <p class="m-6 text-xl text-center text-gray-600">
          {move || tr!("showcase-criminals.challenge")}
        </p>
        <ButtonLinkSecond
          title="S. XIX pseudo-science"
          href="https://www.nbcnews.com/think/opinion/racist-police-practices-mug-shots-normalize-criminalization-black-americans-ncna1235694"
          target="_blank"
          color="green"
        />

        <ButtonLinkSecond
          title="Against Face Analysis"
          href="https://kcimc.medium.com/against-face-analysis-55066903535b"
          target="_blank"
          color="green"
        />
        <ButtonLinkSecond
          title="Predicting people's character from their appearance"
          href="https://www.nytimes.com/2019/07/10/opinion/facial-recognition-race.html"
          target="_blank"
          color="green"
        />
        <ButtonLinkSecond
          title="It's sadly still being investigated"
          href="https://www.technologyreview.com/2016/11/22/107128/neural-network-learns-to-identify-criminals-by-their-faces/"
          target="_blank"
          color="green"
        />
      // https://en.wikipedia.org/wiki/Cesare_Lombroso
      </div>
    }
}

#[component]
fn Dotcom() -> impl IntoView {
    view! {
      {move || update_seen_cards("dotcom")}
      <FlipCard image="card-dotcom" class="animate-fade-slide-in-right"/>
      <div class="lg:max-w-lg animate-fade-slide-in-left">
        <p class="mt-2 text-3xl text-center uppercase text-black">
          {move || tr!("showcase-dotcom")}
        </p>
        <p class="mt-6 text-xl text-center text-gray-600">
          {move || tr!("showcase-dotcom.description")}
        </p>
      </div>
    }
}

#[component]
fn Data() -> impl IntoView {
    view! {
      {move || update_seen_cards("data")}
      <div class="lg:max-w-lg animate-fade-slide-in-right">
        <p class="mt-2 text-3xl text-center uppercase text-black">
          {move || tr!("showcase-more-data")}
        </p>
        <p class="mt-6 text-xl text-center text-gray-600">
          {move || tr!("showcase-more-data.description")}
        </p>
        <p class="my-4 text-xl text-center text-gray-600">
          {move || tr!("showcase-more-data.challenge")}
        </p>

        <ButtonLinkSecond
          title="Interview: Shoshana Zuboff"
          href="https://nymag.com/intelligencer/2019/02/shoshana-zuboff-q-and-a-the-age-of-surveillance-capital.html"
          target="_blank"
          color="yellow"
        />

        <ButtonLinkSecond
          title="The data labelers"
          href="https://www.theverge.com/features/23764584/ai-artificial-intelligence-data-notation-labor-scale-surge-remotasks-openai-chatbots"
          target="_blank"
          color="yellow"
        />

      </div>
      <FlipCard image="card-moredata" class="animate-fade-slide-in-left"/>
    }
}
#[component]
fn Hr() -> impl IntoView {
    view! {
      {move || update_seen_cards("hr")}
      <div class="lg:max-w-lg animate-fade-slide-in-right">
        <p class="mt-2 text-3xl text-center uppercase text-black">
          {move || tr!("showcase-select-employees")}
        </p>
        <p class="mt-6 text-xl text-center text-gray-600">
          {move || tr!("showcase-select-employees.description")}
        </p>
        <p class="my-4 text-xl text-center text-gray-600">
          {move || tr!("showcase-select-employees.challenge")}
        </p>

        <ButtonLinkSecond
          title="BBC article"
          href="https://www.bbc.com/worklife/article/20240214-ai-recruiting-hiring-software-bias-discrimination"
          target="_blank"
        />

        <ButtonLinkSecond
          title="Mystery AI Hype post"
          href="https://buttondown.email/maiht3k/archive/in-praise-of-the-ephemeral/"
          target="_blank"
        />

      </div>
      <FlipCard image="card-hr" class="animate-fade-slide-in-left"/>
    }
}
