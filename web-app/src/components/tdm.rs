use reqwasm::http::Request;
use shared::model::operator::Operator;
use yew::prelude::*;

use crate::logic::randomizer;
use crate::components::{full_op_display::FullOpDisplay, selector::Selector};

async fn fetch_operator(id: &str) -> Operator {
    let url = format!("/api/operator/{}", id);
    Request::get(&url)
        .send().await.unwrap()
        .json().await.unwrap()
}

#[function_component(TDM)]
pub fn tdm() -> Html {
    let state: UseStateHandle<Option<Operator>> = use_state(|| None);

    let gen_state = state.clone();
    let on_gen = Callback::from(move |_| {
        let state = gen_state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let id = randomizer::get_random_id(true, None);
            if let Some(id) = id {
                let operator = fetch_operator(&id).await;
                state.set(randomizer::randomize_operator(true, operator));
            } else {
                log::error!("Failed to randomly select operator.");
            }
        });
    });

    html! {
        <div class="default-content-div">
            <Selector/>
            <div style="flex-grow: 1;" class="ms-4 mt-3">
                <p class="mb-0">{"Generate operators for Team Deathmatch and Free for all modes."}</p>
                <p class="mb-4">{"The app will pick a random Operator from any side, shields are disabled."}</p>
                <div class="mb-3" style="text-align: center; width: 290px; height: 40px;">
                    <p>
                        <button
                            type="button"
                            style="width:75%;"
                            class="btn btn-danger"
                            onclick={on_gen}>
                            {"Generate new"}
                        </button>
                    </p>
                </div>
                <div>
                    {
                        if let Some(state) = &*state {
                            html! { <FullOpDisplay tdm={true} operator={Some(state.clone())}/> }
                        } else {
                            html! { <FullOpDisplay tdm={true} operator={None}/> }
                        }
                    }
                </div>
            </div>
        </div>
    }
}