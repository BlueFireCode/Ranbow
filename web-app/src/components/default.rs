// use gloo_storage::{LocalStorage, Storage};
// use shared::model::operator_display::OperatorDisplay;
use yew::prelude::*;

use crate::components::selector::Selector;

#[function_component(Default)]
pub fn default() -> Html {
    // let val: Vec<OperatorDisplay> = LocalStorage::get("ranbow_selected_operators").unwrap_or(Vec::<OperatorDisplay>::new());

    html! {
        <div style="display: flex;flex-direction: row;width: 100%">
            <Selector/>
            // <div style="flex-grow: 1;" class="ms-4">
            //     <h1 class="mt-3">{"Default"}</h1>
            //     {
            //         val.into_iter().map(|e| {
            //             html! {
            //                 <p>{e.name.clone()}</p>
            //             }
            //         }).collect::<Html>()
            //     }
            // </div>
        </div>
    }
}
