use leptos::ev::MouseEvent;
use leptos::Callback;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
fn MyComponent(#[prop(into)] render_number: Callback<i32, String>) -> impl IntoView {
    view! {
        <div>
            {render_number.call(42)}
        </div>
    }
}

fn test() -> impl IntoView {
    view! {
        <MyComponent render_number=move |x: i32| x.to_string()/>
    }
}

#[component]
fn Button(title: String) -> impl IntoView {
    view! {
        <button
            class="bg-amber-600 hover:bg-amber-800 mx-3 px-5 py-3 text-white rounded-lg"
            >
            {title}
        </button>
    }
}

#[component]
pub fn Card(title: String) -> impl IntoView {
    view! {
        <div class="card text-center shadow-lg shadow-gray-200 bg-white hover:-translate-y-2">
            {title}
        </div>
    }
}

#[component]
pub fn PlayerHand(cards: ReadSignal<Vec<String>>) -> impl IntoView {
    let cards = move || {
        cards
            .get()
            .into_iter()
            .map(|d| {
                view! {<Card title={d} />}
            })
            .collect_view()
    };
    view! {
        <div class="grid grid-flow-row text-neutral-600 sm:grid-cols-1 md:grid-cols-9 lg:grid-cols-5 xl:grid-cols-9">
            {cards}
        </div>
    }
}

#[component]
pub fn PlayerDrawer() -> impl IntoView {
    let plain_cards: Vec<String> = (0..5).map(|n| format!("Card {n}")).collect();
    let (cards, set_cards) = create_signal(plain_cards);
    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`,
    // so it's super easy to move them into closures
    let increment = move |_| set_cards.update(|cs| cs.push("NewCard".to_string()));
    let decrement = move |_| {
        set_cards.update(|cs| {
            cs.pop();
        })
    };

    view! {

    //<!-- drawer init and toggle -->
    <div class="text-center">
          <h4 class="inline-flex items-center text-base text-gray-500 dark:text-gray-400 font-medium">"Player's Hand"</h4>

               <Button title="Draw card".to_string() on:click=increment/>
               <Button title="Plai card".to_string() on:click=decrement/>

                   <div class="mx-8 my-2">
                    <PlayerHand cards={cards}/>
                   </div>
    </div>

        }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {

        //<Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move || view! { <Home/> }/>
            </Routes>
        </Router>
        <script data-trunk scr="./pkg/flowbite.min.js"></script>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to PLAI"</h2>
            <p class="px-10 pb-10">"✨Become the artificial intelligence monopoly you deserve✨"</p>

            <PlayerDrawer />
        </div>


    }
}
