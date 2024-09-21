use leptos::*;
use leptos_animation::*;
use leptos_meta::*;
use leptos_router::ActionForm;
use leptos_use::*;

use crate::web::common::*;
use crate::web::landing::page::Newsletter;
use crate::web::plausible::components::{
    track_active_elements, EndPage, PageView, TrackElement, A,
};

/// Renders the home page of the app
#[allow(clippy::module_name_repetitions)]
#[component]
pub fn RulePage() -> impl IntoView {
    provide_meta_context();
    track_active_elements();

    view! {
      <Title text="How to PLAI"/>

      <PageTitle />

      <Objective />
      <Start />
      <Turn />
      <EndOfGame />

      <Newsletter/>

      <Statistics />
    }
}

#[component]
fn PageTitle() -> impl IntoView {
    view! {
      <div class="mx-auto px-4 sm:px-6 md:max-w-2xl md:px-4 lg:max-w-4xl lg:px-12 text-lg tracking-tight text-slate-700">
        <h1 class="text-8xl uppercase font-bold text-black">PLAI</h1>
        <h2 class="text-4xl uppercase font-bold text-black">THE TECH "WORKER'S" CARD GAME</h2>

        <p class="mt-4 text-orange-400 font-semibold">2 to 5 players</p>
        <p class="mt-1 text-orange-400 font-semibold">Ages: 19+</p>
        <p class="mt-1 text-orange-400 font-semibold">Playtime: 7-59 min</p>
      </div>
    }
}

#[component]
fn Objective() -> impl IntoView {
    view! {
    <section id="objective" aria-label="Objective" class="pt-10 md:pt-12 lg:py-16">
      <div class="mx-auto px-4 sm:px-6 md:max-w-2xl md:px-4 lg:max-w-4xl lg:px-12 text-lg tracking-tight text-slate-700">
      <div class="lg:pl-4">
          <h1 class="uppercase text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">"GAME'S OBJECTIVE"</h1>
          <p class="mt-6 mx-8 text-xl leading-8 text-gray-700">
            "Be the only startup alive and leave the others out of
business, or destroy all startups with your cooperative
Open Source Project."
          </p>
      </div>
      <p class="mt-4">"Lead your AI startup to dominate the competition, aiming to
become the ultimate greenwashing monopoly in this ever-
changing landscape. "<b>"The number of cards in your hand are
your company’s assets"</b>", they will allow you to mess with the
other startups. Do not let the number reach zero, but beware
if you have too many."</p>
      </div>
    </section>

    }
}

#[component]
fn Start() -> impl IntoView {
    view! {
    <section id="objective" aria-label="Objective" class="pt-4 md:pt-8 lg:py-10">
      <div class="mx-auto px-4 sm:px-6 md:max-w-2xl md:px-4 lg:max-w-4xl lg:px-12 text-lg tracking-tight text-slate-700">
      <div class="lg:pl-4">
          <h1 class="uppercase text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">"Starting the game"</h1>
      </div>

          <p class="mt-4">"The bottom left of some cards have roman numerals, depending on the number of players pick the following:"</p>

    <div class="bg-white py-8 sm:py-12">
      <div class="mx-auto max-w-7xl px-6 lg:px-8">
        <dl class="grid grid-cols-1 gap-x-8 gap-y-16 text-center lg:grid-cols-3">
          <div class="mx-auto flex max-w-xs flex-col gap-y-4">
            <dt class="text-base leading-7 text-gray-600">2 <i>plaiers</i></dt>
            <dd class="order-first text-xl font-semibold tracking-tight text-gray-900 sm:text-2xl">Numeral I</dd>
          </div>
          <div class="mx-auto flex max-w-xs flex-col gap-y-4">
            <dt class="text-base leading-7 text-gray-600">3-4 <i>plaiers</i></dt>
            <dd class="order-first text-xl font-semibold tracking-tight text-gray-900 sm:text-2xl">Numerals I & II</dd>
          </div>
          <div class="mx-auto flex max-w-xs flex-col gap-y-4">
            <dt class="text-base leading-7 text-gray-600">5 <i>plaiers</i></dt>
            <dd class="order-first text-xl font-semibold tracking-tight text-gray-900 sm:text-2xl">All cards</dd>
          </div>
        </dl>
      </div>
    </div>


    <p>"Remove Market Event cards from the deck. Shuffle, cut and
        deal 6 cards to each player. Add the Market Event cards back
        into the deck, shuffle and put the deck face down on the table."</p>

      </div>
    </section>


        }
}
#[component]
fn Turn() -> impl IntoView {
    view! {
    <section id="turn" aria-label="Player's turn" class="pt-4 md:pt-8 lg:py-10">
      <div class="mx-auto px-4 sm:px-6 md:max-w-2xl md:px-4 lg:max-w-4xl lg:px-12 text-lg tracking-tight text-slate-700 grid gap-y-4">
      <div class="lg:pl-4">
          <h1 class="uppercase text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">"Your turn"</h1>
      </div>


      <p class="mt-4">
        "During your turn you can do " <b>one</b> of the following actions:
      </p>

      <ul class="pl-8 list-none">
        <li><ListDecoration/> Do a funding</li>
        <li><ListDecoration/> Pl<i>ai</i> a special card</li>
        <li><ListDecoration/> Fight</li>
      </ul>

      <p class="mt-6">
      "By the end of turn, if any startup has run out of assets their
company is declared bankrupt and can't continue on the market.
Any eliminated player becomes an Open Source Maintainer."
      </p>

      <Funding />
      <Fight />
      <OpenSource />

      </div>
    </section>
    }
}
#[component]
fn Funding() -> impl IntoView {
    view! {
          <H3 txt="Funding"/>

          <ul class="mt-6 px-12 space-y-4">
            <li class="flex">
            <CardWireframe />
          <span class="ml-8">
          <p class="font-semibold">Family Funding</p>
          <p>Draw one card from the deck.</p>
          <p>Nobody can attack you</p>
          </span>
          </li>
            <li class="flex">
            <TwoCardWireframe />
          <span class="ml-8">
          <p class="font-semibold">Regional pitch</p>
          <p>Draw two cards from the deck.</p>
          <p>The next player can attack you</p>
          </span>
          </li>

            <li class="flex">

            <ThreeCardWireframe/>
          <span class="ml-8">
          <p class="font-semibold">Venture capital</p>
          <p>Draw three cards.</p>
          <p>Anyone can attack you</p>

          </span>
          </li>
          </ul>

          <p class="mt-6">
            During a Regional pitch or a Venture capital, do not look at
    the drawn cards right away, leave them face down in case
    another startup wants to attack you.
          </p>
          <p class="mt-4">
            If no one attacks, keep the assets raised during the funding.
          </p>
        }
}

#[component]
fn Fight() -> impl IntoView {
    view! {
          <H3 txt="Fight"/>
          <p>Steal assets from other startups.</p>

              <ul class="pl-8 pt-4 list-none">
                <li><ListDecoration/><b>Hostile Takeover</b>: if you have 9 or more cards, you
    can challenge any player at your turn.</li>
                <li><ListDecoration/><b>During a Funding</b>: you or another startup does a
    Regional pitch or a Venture capital</li>
              </ul>

            <div class="bg-gray tracking-tight p-1 pl-4 font-bold text-black">
              You can play up to four battle cards face down.
            </div>

            <p>Use Adversaries if you are attacking and Use Cases if you
    are on the defensive. In both cases you can add Buzzwords to
    increase the number of your attack or defense.</p>

          <p>Specific special cards can be played in the middle of battle,
            such as cards that negate other "card's" effects. These Special
            cards do not need to be in the four picked cards at the
            beginning of the battle.
          </p>

          <p>You can bluff and only attack or defend yourself with
          Buzzword cards. However, if the other startup has Use Cases
          or Adversary cards you will have lost the battle.</p>

            <div class="bg-gray tracking-tight p-1 pl-4 font-bold text-black">
             You can always bribe other players for help.
            </div>

          <H4>Order of events during a fight</H4>

          <ul class="ml-8 grid gap-y-6 list-decimal">
            <li>The attacker(s) chooses their cards to play and places
    them face down. The defender decides if they want to
    defend, and if so, put their cards face down.</li>
            <li>Defense cards are revealed and their effects are applied. Then, do the same for the "attacker's" cards.</li>
            <li>Add up the stregnth of each played card. Whoever
    has more points wins, or in the event of a draw, the
    defense is the winner.</li>
            <li>The winner takes the drawn cards if it is a Funding, or
    the entire losing hand if it is a Hostile Takeover.</li>
            <li>Discard all the played cards.</li>
          </ul>
        }
}

#[component]
fn OpenSource() -> impl IntoView {
    view! {
          <H3 txt="Open Source Project"/>

          <p class="mt-6">
            Your startup is bankrupt, but its not your end. You morph
    into an Open Source Maintainer and can do one of the
    following actions during your next turns:
          </p>

          <ul class="pl-8 pt-4 list-none">
            <li><ListDecoration/> Family funding</li>
            <li><ListDecoration/> Pl<i>ai</i> a special card</li>
            <li><ListDecoration/> Help fight other startups</li>
          </ul>
          <p class="mt-6">
          Open Source Maintainers must act in collaboration and
    talk in public. They can only have a maximum of 6 assets
    between them and any additional ones must be discarded.
          </p>
        }
}

#[component]
fn EndOfGame() -> impl IntoView {
    view! {
        <section id="end-game" aria-label="End of the game" class="pt-10 md:pt-12 lg:py-16 pb-10">
          <div class="mx-auto px-4 sm:px-6 md:max-w-2xl md:px-4 lg:max-w-4xl lg:px-12 text-lg tracking-tight text-slate-700">
          <div class="lg:pl-4">
              <h1 class="uppercase text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">End of game</h1>
          </div>



          <div class="mt-6 grid lg:grid-cols-3  grid-cols-1 gap-y-2 gap-x-4">

              <div><H4>Trough of Disillusionment</H4></div>
              <div class="lg:col-span-2">There are no more cards available in the draw pile. No one will remember you.</div>
              <div class="h-1 col-span-full"><hr class="h-px bg-orange border-0"/></div>

              <div><H4>Monopoly</H4></div>
              <div class="lg:col-span-2">When one startup is left, it must resist
    a coordinated attack from the Open
    Source maintainers. If the company wins,
    congrats, you are now a corporation</div>

              <div class="h-1 col-span-full"><hr class="h-px bg-orange border-0"/></div>
              <div><H4>Teamwork Makes the Dreamwork</H4></div>
              <div class="lg:col-span-2">The Open Source Project won the final battle against the only startup left.
    "It's" a collective win for all (except for the losing player).</div>

           </div>

          </div>
        </section>

        }
}

#[component]
fn Statistics() -> impl IntoView {
    view! {}
}
