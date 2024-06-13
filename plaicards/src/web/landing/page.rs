use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::prelude::*;

use leptos::*;
use leptos_meta::*;
use leptos_router::ActionForm;
use leptos_use::{use_interval, use_interval_fn, use_window_scroll, UseIntervalReturn};

use crate::web::common::ButtonLinkSecond;

/// Renders the home page of the app
#[component]
pub fn HomePage() -> impl IntoView {
    provide_meta_context();
    view! {

        <Title text="PLAI the board game for tech workers"/>
        <Meta charset="utf-8"/>
        <Meta name="description" content="Burned-out of your fast-paced environment? Too much workplace politics? Get ready for PLAI, the board game where you'll now be the CEO of your startup dedicated to... well, do AI things. And data. And also some Quantum stuff. I don't know really but actually your goal is to reign over your friends and fools! Muahahahah"/>
        <Meta name="keywords" content="plai, board game, tech industry, strategy game, cards, satire, kickstarter, ai, data, startup, comedy" />
        <Meta name="HandheldFriendly" content="True" />
        <Meta property="og:site_name" content="plAI card game"/>

            <Meta property="og:site_name" content="plAI card game"/>
    <Meta property="og:type" content="website"/>
    <Meta property="og:title" content="plAI the board game for (not only) tech workers"/>
    <Meta property="og:description" content="Burned-out of your fast-paced environment? Too much workplace politics? Get ready for PLAI, the board game where you'll now be the CEO of your startup dedicated to... well, do AI things. And data. And also some Quantum stuff. I don't know really but actually your goal is to reign over your friends and fools! Muahahahah"/>
    <Meta property="og:url" content="https://plai.cards/links/"/>
    <Meta property="og:image" content="https://plai.cards/content/images/2024/04/Portada.png"/>
    <Meta name="twitter:card" content="summary_large_image"/>
    <Meta name="twitter:title" content="plAI, the board game for (not only) tech workers"/>
    <Meta name="twitter:description" content="Burned-out of your fast-paced environment? Too much workplace politics? Get ready for PLAI, the board game where you'll now be the CEO of your startup dedicated to... well, do AI things. And data. And also some Quantum stuff. I don't know really but actually your goal is to reign over your friends and fools! Muahahahah"/>
    <Meta name="twitter:url" content="https://plai.cards/links/"/>
    <Meta name="twitter:image" content="https://plai.cards/content/images/2024/04/Portada.png"/>
    <Meta name="twitter:label1" content="Written by"/>
    <Meta name="twitter:data1" content="Marti"/>
    <Meta name="twitter:site" content="@plai_cards"/>
    <Meta property="og:image:width" content="1200"/>
    <Meta property="og:image:height" content="675"/>

        <Hero />
        <Testimonials />
        <CardTypes />
        <HeaderStats />
        <Features />
        //<LogoCloud />
        <Newsletter />
        <WordCloud />

    }
}

#[server(EmailAlert, "/api")]
pub async fn add_email_alert(email: String) -> Result<(), ServerFnError> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("data/crowdfunding-emails.txt")
        .unwrap();

    let date_as_string = Utc::now().to_string();
    if let Err(e) = writeln!(file, "{},{}", date_as_string, email) {
        eprintln!("Couldn't write to file: {}", e);
    }
    Ok(())
}

#[component]
fn Hand() -> impl IntoView {
    let (x, y) = use_window_scroll();

    view! {
      <div class="-mt-55 md:mt-0 md:pt-3 lg:px-8 overflow-visible mb-10">

        <div class="grid grid-cols-5 place-content-center justify-items-center">
        <div class="z-10  hover:z-40 bg-card-hr bg-cover -rotate-12" style="width:20rem; height:27rem;border-radius:.5rem;border-color:#000; border-width:0.1rem;"
          style:margin-left=move || format!("{}rem", 25_f64.min(y() /25.))
          style:margin-top=move || format!("{}rem", 4.5_f64.min(y() /50.))
            style=("--tw-rotate", move || format!("-{}deg", 12. - y()/60.))
        />
        <div class="z-20  hover:z-40 mt-10 bg-card-aiarmy bg-cover -rotate-6" style="width:20rem; height:27rem;border-radius:.5rem;border-color:#000; border-width:0.1rem;"
          style:margin-left=move || format!("{}rem", 12_f64.min(y() /50.))
          style:margin-top=move || format!("{}rem", 4.6_f64.min(2.5 + y() /80.))
            style=("--tw-rotate", move || format!("-{}deg", 7. - y()/120.))
        />
        <div class="z-30  hover:z-40 mt-20 bg-card-dotcom bg-cover" style="width:20rem; height:27rem;border-radius:.5rem;border-color:#000; border-width:0.1rem;"></div>
        <div class="z-20  hover:z-40 mt-10 bg-card-moredata bg-cover rotate-6" style="width:20rem; height:27rem;border-radius:.5rem;border-color:#000; border-width:0.1rem;"
          style:margin-left=move || format!("-{}rem", 12_f64.min(y() /50.))
          style:margin-top=move || format!("{}rem", 4.6_f64.min(2.5 + y() /80.))
            style=("--tw-rotate", move || format!("{}deg", 7. - y()/120.))
        />
        <div class="z-10 hover:z-40 bg-card-toxic bg-cover rotate-12" style="width:20rem; height:27rem;border-radius:.5rem;border-color:#000; border-width:0.1rem;"
          style:margin-left=move || format!("-{}rem", 25_f64.min(y() /25.))
            style=("--tw-rotate", move || format!("{}deg", 12. - y()/60.))
          style:margin-top=move || format!("{}rem", 4.5_f64.min(y() /50.))
        />
        </div>

      </div>
    }
}

#[component]
fn Hero() -> impl IntoView {
    let add_email = create_server_action::<EmailAlert>();
    let value = add_email.value();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    let email_adjectives = [
        "gdpr-friendly",
        "best",
        "msn",
        "cool",
        "yahoo",
        "work",
        "tinet",
        "active",
        "friends'",
        "spam",
        "AOL",
        "test",
        "professional",
        "apple",
        "hotmail",
    ];

    let UseIntervalReturn { counter, .. } = use_interval(1500);

    view! {
    <div class="overflow-hidden bg-white pt-4 sm:py-8 lg:py-16 lg:h-svh content-center">
      <div class="mx-auto max-w-7xl px-6 lg:px-8">

        <div class="mx-auto place-items-center grid max-w-2xl grid-cols-1 md:gap-y-10 lg:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-2">
          <div class="z-50 lg:pr-8">
              <div class="lg:max-w-lg">
              <p class="text-lg leading-8 text-gray-600">Seize control of <mark>your career</mark></p>
              <p class="mt-2 text-3xl text-center font-bold tracking-tight text-gray-900 sm:text-4xl bg-gradient-to-r from-orange-700 via-blue-500 to-green-400 text-transparent bg-clip-text bg-300% animate-gradient">PAVE YOUR OWN PATH*</p>

              <p class="mt-10 text-lg leading-8 text-gray-600">Guide your <mark>tech startup</mark> to</p>
              <div class="bg-texture-paper bg-repeat  bg-clip-text bg-fixed bg-parallax">
              <p class="mt-2 text-5xl flex bg-clip-text text-center font-black tracking-tight sm:text-6xl text-transparent">MARKET DOMINANCE</p>
              </div>

              <div class="mt-8 sm:mt-20 mb-6">
                <p class="text-lg text-gray-600 leading-8">
                  PLAI, the board game for tech practitioners
                </p>
                <p class="text-lg text-gray-600 leading-8">
                  <mark>Next sprint</mark> on Kickstarter
                </p>

            <ActionForm action=add_email class="plausible-DOESNOTWORK-event-name=Subscribe+Top">
            <div class="mt-6 grid grid-rows-2 px-10 gap-4">
              <label for="email-address" class="sr-only">Email address</label>
              <input id="email-address" name="email" type="email" autocomplete="email" required class="text-center min-w-0 flex-auto rounded-md border-0 bg-white opacity-90 px-3.5 py-2 shadow-sm ring-1 ring-inset ring-green/10 focus:ring-2 focus:ring-inset focus:ring-green-700 sm:text-lg sm:leading-6" placeholder={move || format!("Enter your {} email", email_adjectives[(counter()) as usize % email_adjectives.len()])} />
              <button type="submit" class="flex-none rounded-md bg-green-700 px-3.5 py-2.5 text-lg font-semibold text-white shadow-sm hover:bg-green-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-emerald-500 ">
              Get your copy
            </button>

            </div>
            </ActionForm>
            <div class="px-10 my-6">
              <ButtonLinkSecond
                title="See the cards"
                class="plausible-event-name=LandingSeeCards"
                href="/cards" />
            </div>
              </div>
              </div>
          </div>

          <Hand />
        </div>
      </div>
      // Scroll down icon
      <div class="z-50 absolute inset-x-0 bottom-0 flex justify-center">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-16 text-blue motion-safe:animate-bounce">
        <path stroke-linecap="round" stroke-linejoin="round" d="m9 12.75 3 3m0 0 3-3m-3 3v-7.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
      </svg>
      </div>
    </div>


    }
}

#[component]
fn Testimonials() -> impl IntoView {
    view! {

    //<!-- This example requires Tailwind CSS v2.0+ -->
    <section class="bg-gray-900">
      <div class="max-w-7xl mx-auto md:grid md:grid-cols-3 md:px-6 lg:px-8">
        <Testimonial
          comment="I thought working in tech was intense, but this game takes it to a whole new level. At least in PLAI, I can be the CEO."
          name="Casey C."
          title="Data Unicorn, Nat20 Corp"
        />

        <Testimonial
          comment="Who needs real-life success when you can dominate the tech world on your living room table?"
          name="Mark Z."
          title="Head of Data Manipulation, TrustMe Inc."
        />
        <Testimonial
          comment="Finally, a way to experience startup stress without the paycheck!"
          name="Esther L."
          title="Prompt Engineer, AItomate"
        />


      </div>
    </section>


      }
}

#[component]
fn Testimonial(comment: &'static str, name: &'static str, title: &'static str) -> impl IntoView {
    view! {
          <div class="py-12 px-4 sm:px-6 md:flex md:flex-col md:py-16 md:pl-0 md:pr-10 lg:pr-16">
            <blockquote class="mt-6 md:flex-grow md:flex md:flex-col">
              <div class="relative text-lg font-medium text-white md:flex-grow">
                <svg class="absolute top-0 left-0 transform -translate-x-3 -translate-y-2 h-8 w-8 text-blue-700" fill="currentColor" viewBox="0 0 32 32" aria-hidden="true">
                  <path d="M9.352 4C4.456 7.456 1 13.12 1 19.36c0 5.088 3.072 8.064 6.624 8.064 3.36 0 5.856-2.688 5.856-5.856 0-3.168-2.208-5.472-5.088-5.472-.576 0-1.344.096-1.536.192.48-3.264 3.552-7.104 6.624-9.024L9.352 4zm16.512 0c-4.8 3.456-8.256 9.12-8.256 15.36 0 5.088 3.072 8.064 6.624 8.064 3.264 0 5.856-2.688 5.856-5.856 0-3.168-2.304-5.472-5.184-5.472-.576 0-1.248.096-1.44.192.48-3.264 3.456-7.104 6.528-9.024L25.864 4z" />
                </svg>
                <p class="relative">{comment}</p>
              </div>
              <div class="mt-8">
                <div class="flex items-start">
                  <div class="ml-4">
                    <div class="text-base font-medium text-white">{name}</div>
                    <div class="text-base font-medium text-gray-200">{title}</div>
                  </div>
                </div>
              </div>
            </blockquote>
          </div>
    }
}

#[component]
fn Features() -> impl IntoView {
    view! {


          <div class="overflow-hidden bg-white py-24 sm:py-32">
            <div class="flex justify-center mx-auto max-w-7xl px-6 lg:px-8">

              <dl class="max-w-xl text-base leading-7 text-gray-600 lg:max-w-none grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-10">
                      <div class="relative pl-9">
                        <dt class="inline font-semibold text-gray-900">
                          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="absolute left-1 top-1 size-5 text-emerald-600">
                            <path fill-rule="evenodd" d="M3.5 2A1.5 1.5 0 0 0 2 3.5V5c0 1.149.15 2.263.43 3.326a13.022 13.022 0 0 0 9.244 9.244c1.063.28 2.177.43 3.326.43h1.5a1.5 1.5 0 0 0 1.5-1.5v-1.148a1.5 1.5 0 0 0-1.175-1.465l-3.223-.716a1.5 1.5 0 0 0-1.767 1.052l-.267.933c-.117.41-.555.643-.95.48a11.542 11.542 0 0 1-6.254-6.254c-.163-.395.07-.833.48-.95l.933-.267a1.5 1.5 0 0 0 1.052-1.767l-.716-3.223A1.5 1.5 0 0 0 4.648 2H3.5Zm9.78.22a.75.75 0 1 0-1.06 1.06L13.94 5l-1.72 1.72a.75.75 0 0 0 1.06 1.06L15 6.06l1.72 1.72a.75.75 0 1 0 1.06-1.06L16.06 5l1.72-1.72a.75.75 0 0 0-1.06-1.06L15 3.94l-1.72-1.72Z" clip-rule="evenodd" />
                          </svg>
                          Board game.
                        </dt>
                        <dd class="inline">"Let's leave the phones home and go back to play like in Mesopotamia. Because analog is the new retro."</dd>
                      </div>



                      <div class="relative pl-9">
                        <dt class="inline font-semibold text-gray-900">
                          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="absolute left-1 top-1 size-5 text-emerald-600">
        <path fill-rule="evenodd" d="M8.25 6.75a3.75 3.75 0 1 1 7.5 0 3.75 3.75 0 0 1-7.5 0ZM15.75 9.75a3 3 0 1 1 6 0 3 3 0 0 1-6 0ZM2.25 9.75a3 3 0 1 1 6 0 3 3 0 0 1-6 0ZM6.31 15.117A6.745 6.745 0 0 1 12 12a6.745 6.745 0 0 1 6.709 7.498.75.75 0 0 1-.372.568A12.696 12.696 0 0 1 12 21.75c-2.305 0-4.47-.612-6.337-1.684a.75.75 0 0 1-.372-.568 6.787 6.787 0 0 1 1.019-4.38Z" clip-rule="evenodd" />
        <path d="M5.082 14.254a8.287 8.287 0 0 0-1.308 5.135 9.687 9.687 0 0 1-1.764-.44l-.115-.04a.563.563 0 0 1-.373-.487l-.01-.121a3.75 3.75 0 0 1 3.57-4.047ZM20.226 19.389a8.287 8.287 0 0 0-1.308-5.135 3.75 3.75 0 0 1 3.57 4.047l-.01.121a.563.563 0 0 1-.373.486l-.115.04c-.567.2-1.156.349-1.764.441Z" />
      </svg>

                          With friends.
                        </dt>
                        <dd class="inline">"Play with friends and work colleagues while you have them."</dd>
                      </div>


                      <div class="relative pl-9">
                        <dt class="inline font-semibold text-gray-900">
                          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="absolute left-1 top-1 size-5 text-emerald-600">
    <path fill-rule="evenodd" d="M5.166 2.621v.858c-1.035.148-2.059.33-3.071.543a.75.75 0 0 0-.584.859 6.753 6.753 0 0 0 6.138 5.6 6.73 6.73 0 0 0 2.743 1.346A6.707 6.707 0 0 1 9.279 15H8.54c-1.036 0-1.875.84-1.875 1.875V19.5h-.75a2.25 2.25 0 0 0-2.25 2.25c0 .414.336.75.75.75h15a.75.75 0 0 0 .75-.75 2.25 2.25 0 0 0-2.25-2.25h-.75v-2.625c0-1.036-.84-1.875-1.875-1.875h-.739a6.706 6.706 0 0 1-1.112-3.173 6.73 6.73 0 0 0 2.743-1.347 6.753 6.753 0 0 0 6.139-5.6.75.75 0 0 0-.585-.858 47.077 47.077 0 0 0-3.07-.543V2.62a.75.75 0 0 0-.658-.744 49.22 49.22 0 0 0-6.093-.377c-2.063 0-4.096.128-6.093.377a.75.75 0 0 0-.657.744Zm0 2.629c0 1.196.312 2.32.857 3.294A5.266 5.266 0 0 1 3.16 5.337a45.6 45.6 0 0 1 2.006-.343v.256Zm13.5 0v-.256c.674.1 1.343.214 2.006.343a5.265 5.265 0 0 1-2.863 3.207 6.72 6.72 0 0 0 .857-3.294Z" clip-rule="evenodd" />

      </svg>

                          Win.
                        </dt>
                        <dd class="inline">"Nothing else matters. It's you against the market. Don't let them have any funding and hang them dry."</dd>
                      </div>

                      <div class="relative pl-9">
                        <dt class="inline font-semibold text-gray-900">
                          <svg class="absolute left-1 top-1 h-5 w-5 text-emerald-600" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                            <path fill-rule="evenodd" d="M5.5 17a4.5 4.5 0 01-1.44-8.765 4.5 4.5 0 018.302-3.046 3.5 3.5 0 014.504 4.272A4 4 0 0115 17H5.5zm3.75-2.75a.75.75 0 001.5 0V9.66l1.95 2.1a.75.75 0 101.1-1.02l-3.25-3.5a.75.75 0 00-1.1 0l-3.25 3.5a.75.75 0 101.1 1.02l1.95-2.1v4.59z" clip-rule="evenodd" />
                          </svg>
                          Cloud agnostic.
                        </dt>
                        <dd class="inline">Do you use one of the big clouds? None of them? No worries, PLAI is designed to be local-first and resillient to cloud downtimes.</dd>
                      </div>


                      <div class="relative pl-9">
                        <dt class="inline font-semibold text-gray-900">
                          <svg class="absolute left-1 top-1 h-5 w-5 text-emerald-600" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                            <path fill-rule="evenodd" d="M10 1a4.5 4.5 0 00-4.5 4.5V9H5a2 2 0 00-2 2v6a2 2 0 002 2h10a2 2 0 002-2v-6a2 2 0 00-2-2h-.5V5.5A4.5 4.5 0 0010 1zm3 8V5.5a3 3 0 10-6 0V9h6z" clip-rule="evenodd" />
                          </svg>
                          Privacy.
                        </dt>
                        <dd class="inline">"It's your game so you decide how to plai. Our cards respect your privacy and have no trackers within."</dd>
                      </div>


                      <div class="relative pl-9">
                        <dt class="inline font-semibold text-gray-900">
                          <svg class="absolute left-1 top-1 h-5 w-5 text-emerald-600" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                            <path d="M4.632 3.533A2 2 0 016.577 2h6.846a2 2 0 011.945 1.533l1.976 8.234A3.489 3.489 0 0016 11.5H4c-.476 0-.93.095-1.344.267l1.976-8.234z" />
                            <path fill-rule="evenodd" d="M4 13a2 2 0 100 4h12a2 2 0 100-4H4zm11.24 2a.75.75 0 01.75-.75H16a.75.75 0 01.75.75v.01a.75.75 0 01-.75.75h-.01a.75.75 0 01-.75-.75V15zm-2.25-.75a.75.75 0 00-.75.75v.01c0 .414.336.75.75.75H13a.75.75 0 00.75-.75V15a.75.75 0 00-.75-.75h-.01z" clip-rule="evenodd" />
                          </svg>
                          A box.
                        </dt>
                        <dd class="inline">"Unlike laptops, computers and servers, we provide a box you'll want to use most of the time to store the cards away."</dd>
                      </div>



              </dl>
            </div>
          </div>


          }
}

#[component]
fn HeaderStats() -> impl IntoView {
    view! {
      <div class="relative isolate overflow-hidden bg-gray-900 py-24 sm:py-32">
        <img src="img/home_cards.jpg" alt="" class="absolute inset-0 -z-10 h-full w-full object-cover object-right md:object-center" />
        <div class="hidden sm:absolute sm:-top-10 sm:right-1/2 sm:-z-10 sm:mr-10 sm:block sm:transform-gpu sm:blur-3xl" aria-hidden="true">
          <div class="aspect-[1097/845] w-[68.5625rem] bg-gradient-to-tr from-green to-orange opacity-20" style="clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)"></div>
        </div>
        <div class="absolute -top-52 left-1/2 -z-10 -translate-x-1/2 transform-gpu blur-3xl sm:top-[-28rem] sm:ml-16 sm:translate-x-0 sm:transform-gpu" aria-hidden="true">
          <div class="aspect-[1097/845] w-[68.5625rem] bg-gradient-to-tr from-orange to-blue opacity-20" style="clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)"></div>
        </div>
        <div class="mx-auto max-w-7xl px-6 lg:px-8">
          <div class="mx-auto max-w-2xl lg:mx-0">
            <h2 class="text-4xl font-bold tracking-tight text-white sm:text-6xl flex items-center">
    <span class="relative ml-3 h-[1em] w-40 overflow-hidden">
        <span class="absolute h-full w-full -translate-y-full animate-slide leading-none text-green-300">
        Board
        </span>
      <span
        class="absolute h-full w-full -translate-y-full animate-slide leading-none text-orange-300 [animation-delay:3s]"
      >
        Field
      </span>
      <span
        class="absolute h-full w-full -translate-y-full animate-slide leading-none text-blue-300 line-through [animation-delay:6s]"
      >
        AI
      </span>
      </span>

            tested</h2>
            <p class="mt-6 text-lg leading-8 text-gray-300">"PLAI is a ready to play board game. Developed over a couple years, we've done multiple playtests with people like you and different than you so you can enjoy the thrill of building the next tech giant."</p>
          </div>
          <div class="mx-auto mt-10 mb-4 max-w-2xl lg:mx-0 lg:max-w-none">
            //<div class="grid grid-cols-1 gap-x-8 gap-y-6 text-base font-semibold leading-7 text-white sm:grid-cols-2 md:flex lg:gap-x-10">
            //  <a href="#">Open roles <span aria-hidden="true">&rarr;</span></a>
            //  <a href="#">Internship program <span aria-hidden="true">&rarr;</span></a>
            //  <a href="#">Our values <span aria-hidden="true">&rarr;</span></a>
            //  <a href="#">Meet our leadership <span aria-hidden="true">&rarr;</span></a>
            //</div>
            <dl class="mt-16 grid grid-cols-1 gap-8 sm:mt-20 sm:items-center sm:grid-cols-2 lg:grid-cols-5">
              <div class="flex flex-col-reverse">
                <dt class="text-base leading-7 text-gray-300">Players</dt>
                <dd class="text-4xl font-bold leading-9 tracking-tight text-white">"2-5"</dd>
              </div>
              <div class="flex flex-col-reverse">
                <dt class="text-base leading-7 text-gray-300">Avg. game time</dt>
                <dd class="text-4xl font-bold leading-9 tracking-tight text-white">"~25 min"</dd>
              </div>
              <div class="flex flex-col-reverse">
                <dt class="text-base leading-7 text-gray-300">Revisions</dt>
                <dd class="text-4xl font-bold leading-9 tracking-tight text-white">9</dd>
              </div>
              <div class="flex flex-col-reverse">
                <dt class="text-base leading-7 text-gray-300">Beta testers</dt>
                <dd class="text-4xl font-bold leading-9 tracking-tight text-white">100+</dd>
              </div>
              <div class="flex flex-col-reverse">
                <dt class="text-base leading-7 text-gray-300">Satire potential</dt>
                <dd class="text-4xl font-bold leading-9 tracking-tight text-white">Unlimited</dd>
              </div>
            </dl>
          </div>
        </div>
      </div>

          }
}

#[component]
fn CardTypes() -> impl IntoView {
    view! {
     <div class="w-full my-6 lg:my-12 inline-flex flex-nowrap overflow-hidden [mask-image:_linear-gradient(to_right,transparent_0,_black_32px,_black_calc(100%-60px),transparent_100%)] md:[mask-image:_linear-gradient(to_right,transparent_0,_black_128px,_black_calc(100%-200px),transparent_100%)]">
        <ul class="flex items-center justify-center md:justify-start [&_li]:mx-8 [&_img]:max-w-none [&_img]:max-h-40 animate-infinite-scroll">
            <li>
                <img src="./img/TheCards-1.png" alt="Adversary" />
            </li>
            <li>
                <img src="./img/TheCards-Title-1.png" alt="Attack Cards" />
            </li>
            <li>
                <img src="./img/TheCards-Title-2.png" alt="Defense Cards" />
            </li>
            <li>
                <img src="./img/TheCards-2.png" alt="Use Case" />
            </li>
            <li>
                <img src="./img/TheCards-3.png" alt="Buzzwords" />
            </li>
            <li>
                <img src="./img/TheCards-Title-3.png" alt="Support Cards" />
            </li>
            <li>
                <img src="./img/TheCards-Title-4.png" alt="Special" />
            </li>
            <li>
                <img src="./img/TheCards-4.png" alt="Game changing" />
            </li>
            <li>
                <img src="./img/TheCards-5.png" alt="Market Events" />
            </li>
            <li>
                <img src="./img/TheCards-Title-5.png" alt="Immediate Use" />
            </li>
        </ul>
        <ul class="flex items-center justify-center md:justify-start [&_li]:mx-8 [&_img]:max-w-none [&_img]:max-h-40 animate-infinite-scroll" aria-hidden="true">
            <li>
                <img src="./img/TheCards-1.png" alt="Adversary" />
            </li>
            <li>
                <img src="./img/TheCards-Title-1.png" alt="Attack Cards" />
            </li>
            <li>
                <img src="./img/TheCards-Title-2.png" alt="Defense Cards" />
            </li>
            <li>
                <img src="./img/TheCards-2.png" alt="Use Case" />
            </li>
            <li>
                <img src="./img/TheCards-3.png" alt="Buzzwords" />
            </li>
            <li>
                <img src="./img/TheCards-Title-3.png" alt="Support Cards" />
            </li>
            <li>
                <img src="./img/TheCards-Title-4.png" alt="Special" />
            </li>
            <li>
                <img src="./img/TheCards-4.png" alt="Game changing" />
            </li>
            <li>
                <img src="./img/TheCards-5.png" alt="Market Events" />
            </li>
            <li>
                <img src="./img/TheCards-Title-5.png" alt="Immediate Use" />
            </li>
        </ul>
    </div>
      }
}
#[component]
fn WordCloud() -> impl IntoView {
    view! {

    <div class="overflow-hidden">
    <div class="w-full inline-flex flex-nowrap text-13xl font-mono bg-card-antitrust bg-clip-text bg-fixed bg-parallax bg-repeat opacity-50">
        <ul class="text-transparent flex items-center justify-center md:justify-start [&_li]:mx-8 [&_img]:max-w-none animate-scrollRight uppercase font-black" style="-webkit-text-stroke: 1px black;">
            <li>PLAI</li>
            <li>GAME</li>
            <li>Startup</li>
            <li>CEO</li>
            <li>Selling</li>
            <li>GDPR</li>
            <li>Data</li>
        </ul>
    </div>
    <div class="w-full py-2 inline-flex flex-nowrap text-13xl font-mono bg-card-back bg-clip-text bg-fixed bg-parallax bg-repeat opacity-50">
        <ul class="text-transparent flex items-center justify-center md:justify-start [&_li]:mx-8 [&_img]:max-w-none animate-scrollLeft uppercase font-black">
          <li>PYTHON</li>
          <li>Artificial</li>
          <li>Intelligence</li>
          <li>Neural</li>
          <li>Network</li>
          <li>Rust</li>
        </ul>
    </div>
    </div>
      }
}

#[component]
fn LogoCloud() -> impl IntoView {
    view! {
    <div class="bg-white py-24 sm:py-32">
      <div class="mx-auto max-w-7xl px-6 lg:px-8">
        <h2 class="text-center text-lg font-semibold leading-8 text-gray-900">Trusted by the worlds most innovative teams</h2>
        <div class="mx-auto mt-10 grid max-w-lg grid-cols-4 items-center gap-x-8 gap-y-10 sm:max-w-xl sm:grid-cols-6 sm:gap-x-10 lg:mx-0 lg:max-w-none lg:grid-cols-5">
          <img class="col-span-2 max-h-12 w-full object-contain lg:col-span-1" src="https://tailwindui.com/img/logos/158x48/transistor-logo-gray-900.svg" alt="Transistor" width="158" height="48" />
          <img class="col-span-2 max-h-12 w-full object-contain lg:col-span-1" src="https://tailwindui.com/img/logos/158x48/reform-logo-gray-900.svg" alt="Reform" width="158" height="48" />
          <img class="col-span-2 max-h-12 w-full object-contain lg:col-span-1" src="https://tailwindui.com/img/logos/158x48/tuple-logo-gray-900.svg" alt="Tuple" width="158" height="48" />
          <img class="col-span-2 max-h-12 w-full object-contain sm:col-start-2 lg:col-span-1" src="https://tailwindui.com/img/logos/158x48/savvycal-logo-gray-900.svg" alt="SavvyCal" width="158" height="48" />
          <img class="col-span-2 col-start-2 max-h-12 w-full object-contain sm:col-start-auto lg:col-span-1" src="https://tailwindui.com/img/logos/158x48/statamic-logo-gray-900.svg" alt="Statamic" width="158" height="48" />
        </div>
      </div>
    </div>
        }
}

#[component]
pub fn Newsletter() -> impl IntoView {
    let add_email = create_server_action::<EmailAlert>();
    let value = add_email.value();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));
    view! {
    <div class="relative isolate overflow-hidden bg-gray-900 py-16 sm:py-24 lg:py-32">
      <div class="mx-auto max-w-7xl px-6 lg:px-8">
        <div class="mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 lg:max-w-none lg:grid-cols-2">
          <div class="max-w-xl lg:max-w-lg">
            <h2 class="text-3xl font-bold tracking-tight text-white sm:text-4xl">Help us!</h2>
            <p class="mt-4 text-lg leading-8 text-gray-300">We want to release PLAI to production, but we need enough mass to print them. </p>
            <ActionForm action=add_email>
              <div class="mt-6 flex max-w-md gap-x-4">
                <label for="email-address" class="sr-only">Email address</label>
                <input id="email-address" name="email" type="email" autocomplete="email" required class="min-w-0 flex-auto rounded-md border-0 bg-white/5 px-3.5 py-2 text-white shadow-sm ring-1 ring-inset ring-white/10 focus:ring-2 focus:ring-inset focus:ring-green-500 sm:text-sm sm:leading-6" placeholder="Enter your best email" />
                <button type="submit" class="flex-none rounded-md bg-green-500 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-green-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-green-500 plausible-event-name=Subscribe+Bottom">I want them!</button>
              </div>
            </ActionForm>
          </div>
          <dl class="grid grid-cols-1 gap-x-8 gap-y-10 sm:grid-cols-2 lg:pt-2">
            <div class="flex flex-col items-start">
              <div class="rounded-md bg-white/5 p-2 ring-1 ring-white/10">
                <svg class="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5m-9-6h.008v.008H12v-.008zM12 15h.008v.008H12V15zm0 2.25h.008v.008H12v-.008zM9.75 15h.008v.008H9.75V15zm0 2.25h.008v.008H9.75v-.008zM7.5 15h.008v.008H7.5V15zm0 2.25h.008v.008H7.5v-.008zm6.75-4.5h.008v.008h-.008v-.008zm0 2.25h.008v.008h-.008V15zm0 2.25h.008v.008h-.008v-.008zm2.25-4.5h.008v.008H16.5v-.008zm0 2.25h.008v.008H16.5V15z" />
                </svg>
              </div>
              <dt class="mt-4 font-semibold text-white">"Don't miss it"</dt>
              <dd class="mt-2 leading-7 text-gray-400">Get an alert when the crowdfunding campaign starts</dd>
            </div>
            <div class="flex flex-col items-start">
              <div class="rounded-md bg-white/5 p-2 ring-1 ring-white/10">
                <svg class="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M10.05 4.575a1.575 1.575 0 10-3.15 0v3m3.15-3v-1.5a1.575 1.575 0 013.15 0v1.5m-3.15 0l.075 5.925m3.075.75V4.575m0 0a1.575 1.575 0 013.15 0V15M6.9 7.575a1.575 1.575 0 10-3.15 0v8.175a6.75 6.75 0 006.75 6.75h2.018a5.25 5.25 0 003.712-1.538l1.732-1.732a5.25 5.25 0 001.538-3.712l.003-2.024a.668.668 0 01.198-.471 1.575 1.575 0 10-2.228-2.228 3.818 3.818 0 00-1.12 2.687M6.9 7.575V12m6.27 4.318A4.49 4.49 0 0116.35 15m.002 0h-.002" />
                </svg>
              </div>
              <dt class="mt-4 font-semibold text-white">No spam</dt>
              <dd class="mt-2 leading-7 text-gray-400">"I'll just use your email 1-3 times to send you campaign related activities. For spam and seeign how cool the game is, follow our instagram at " <a href="https://www.instagram.com/plai_cards/" target="_blank" class="underline decoration-green underline-offset-4 text-white font-bold hover:decoration-blue hover:underline-offset-8 decoration-2">@plai_cards</a></dd>
            </div>
          </dl>
        </div>
      </div>
      <div class="absolute left-1/2 top-0 -z-10 -translate-x-1/2 blur-3xl xl:-top-6" aria-hidden="true">
        <div class="aspect-[1155/678] w-[72.1875rem] bg-gradient-to-tr from-green to-blue opacity-30" style="clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)"></div>
      </div>
    </div>

        }
}
