use leptos::*;

use crate::ui::app::{Button, ButtonLink};

#[component]
pub fn Setup(players: RwSignal<Vec<(i32, RwSignal<String>)>>) -> impl IntoView {
    let mut player_id = 2;
    let add_player = move |_| {
        players.update(|ps| {
            ps.push((player_id, create_rw_signal("New Player".to_string())));
            player_id += 1;
        });
    };

    let player_list = move || {
        players
            .get()
            .into_iter()
            .map(|(id, n)| {
                view! {
                    <li class="py-2">
                        <input
                            type="text"
                            id=id
                            class="rounded-lg"
                            on:input=move |ev| { n.set(event_target_value(&ev)) }
                            prop:value=n
                        />

                        <Button
                            title="Remove 📉".to_string()
                            on:click=move |_| {
                                players
                                    .update(|ps| {
                                        ps.retain(|(pid, _)| pid != &id);
                                    });
                            }
                        />
                    </li>
                }
            })
            .collect_view()
    };

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
             <img src="img/logo.png" alt="PLAI logo" class="mt-6 mx-auto w-56 h-56"/>

            <h2 class="p-6 text-4xl">"Welcome to PLAI"</h2>
            <p class="px-10 pb-10">
                "✨Become the artificial intelligence monopoly you deserve✨"
            </p>

            <div>
                <Button title="Add Player".to_string() on:click=add_player/>
                <ul class="my-4">{player_list}</ul>

                <div class="my-4">
                    <Show
                        when=move || { 2 <= players.get().len() && players.get().len() <= 6 }
                        fallback=|| view! { "Choose between 1 to 6 players." }
                    >
                        <ButtonLink title="👩🏾‍💼 Start Game 👨🏾‍💼".to_string() href="/plai".to_string()/>
                    </Show>
                </div>
            </div>
        </div>
    }
}
