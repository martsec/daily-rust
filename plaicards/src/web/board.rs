use crate::web::common::Button;
use crate::web::common::ButtonLink;
use data_encoding::BASE64URL_NOPAD;
use leptos::*;
use leptos_meta::*;
use leptos_router::use_params_map;
use leptos_router::*;
use leptos_use::{
    use_websocket, use_websocket_with_options, UseWebSocketOptions, UseWebsocketReturn,
};

#[component]
pub fn Board() -> impl IntoView {
    view! {
    <div class="h-screen bg-gray-200">
    <Nav />


    //<!-- Player Drawers -->
    <div class="mt-0.5 flex justify-around">
        <HandHorizontal />
        <HandHorizontal />
    </div>
    <HandVertical left=false />
    <HandVertical left=true />

    <MiddleBoard />

    <PlayerDrawer />
    </div>
    }
}

#[component]
fn Nav() -> impl IntoView {
    view! {
     <nav class="flex justify-center">
      <div class="fixed top-2 content-center w-11/12 bg-white/30 backdrop-blur-md py-2 z-50 rounded-2xl">
         <div class="container mx-auto px-4 grid grid-cols-3 justify-items-center items-center text-white">
           <div class="justify-self-start">
             <h1>Rounds: 12</h1>
           </div>

           //<!-- Game Title -->
           <div class="justify-self-center">
             <h1 class="text-2xl">PLAI</h1>
           </div>

           //<!-- Players' Icons -->
           <div class="justify-self-end flex">
             <div class="mx-1 h-6 w-6 rounded-full bg-blue"></div>
             <div class="mx-1 h-6 w-6 rounded-full bg-green"></div>
             <div class="mx-1 h-6 w-6 rounded-full bg-orange"></div>
             <div class="mx-1 h-6 w-6 rounded-full bg-yellow"></div>
             <div class="mx-1 h-6 w-6 rounded-full bg-gray-illustration"></div>
           </div>
         </div>
       </div>
    </nav>
       }
}

#[component]
fn MiddleBoard() -> impl IntoView {
    view! {
    //<!-- Middle Area for Decks -->
    <div class="my-8 flex justify-center space-x-4">
      <div class="h-32 w-24 bg-gray-700"></div>
      <div class="h-32 w-24 bg-gray-500"></div>
    </div>
      }
}

#[component]
fn HandVertical(left: bool) -> impl IntoView {
    view! {
    <div class="absolute left-5 top-1/4">
      <div class="rounded bg-white p-3">
        <h2>Player 1</h2>
        <div class="drawer-container">
          <div class="card-vertical will-change-transform"></div>
          <div class="card-vertical will-change-transform"></div>
          <div class="card-vertical will-change-transform"></div>
          <div class="card-vertical will-change-transform"></div>
          <div class="card-vertical will-change-transform"></div>
        </div>
      </div>
    </div>
    }
}

#[component]
fn HandHorizontal() -> impl IntoView {
    view! {
    <div class="rounded bg-white p-2">
      <div class="card-container p-2">
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
      </div>
      <h2>Player 3</h2>
    </div>
    }
}

#[component]
fn PlayerDrawer() -> impl IntoView {
    view! {
    //<!-- Bottom Drawer for Player's Cards -->
    <div id="playersDrawer" class="fixed bottom-0 left-0 right-0 rounded-t-lg p-4 text-white  bg-green-700/20 backdrop-blur-md">

      <div class="grid grid-cols-3 justify-items-center">
        <div class="justify-self-start">
        <button onclick="toggleDrawer()" class="focus:shadow-outline rounded bg-red-500 px-4 py-2 font-bold text-white hover:bg-red-700 focus:outline-none">Hide</button>
        </div>
        <div>
          <h2>Your Cards</h2>
        </div>
        <div>
        </div>
      </div>

      <div class="grid justify-center">
      <div class="card-container pt-4">
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
        <div class="card will-change-transform"></div>
      </div>
    </div>
    </div>
    }
}
