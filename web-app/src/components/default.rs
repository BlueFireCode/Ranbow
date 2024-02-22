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

#[function_component(Default)]
pub fn default() -> Html {
    let state: UseStateHandle<Option<Operator>> = use_state(|| None);

    let atk_state = state.clone();
    let on_atk = Callback::from(move |_| {
        let state = atk_state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let id = randomizer::get_random_id(true);
            if let Some(id) = id {
                let operator = fetch_operator(&id).await;
                state.set(randomizer::randomize_operator(operator));
            } else {
                log::error!("Failed to randomly select operator.");
            }
        });
    });

    let def_state = state.clone();
    let on_def = Callback::from(move |_| {
        let state = def_state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let id = randomizer::get_random_id(false);
            if let Some(id) = id {
                let operator = fetch_operator(&id).await;
                state.set(randomizer::randomize_operator(operator));
            } else {
                log::error!("Failed to randomly select operator.");
            }
        });
    });

    html! {
        <div class="default-content-div">
            <Selector/>

            <div style="flex-grow: 1;" class="ms-4 mt-3">
                <h4 class="mb-3">{"Generate new"}</h4>
                <div class="mb-3" style="width: 290px; height: 40px;">
                    <button
                        type="button"
                        style="width:48%; float: left;"
                        class="btn btn-danger"
                        onclick={on_atk}>
                        {"Attacker"}</button>
                    <button
                        type="button"
                        style="width:48%; float: right;"
                        class="btn btn-primary"
                        onclick={on_def}>
                        {"Defender"}</button>
                </div>
                <div>
                    {
                        if let Some(state) = &*state {
                            html! { <FullOpDisplay operator={Some(state.clone())}/> }
                        } else {
                            html! { <FullOpDisplay operator={None}/> }
                        }
                    }
                </div>
            </div>
        </div>
    }
}
