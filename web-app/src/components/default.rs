use yew::prelude::*;

use crate::components::selector::Selector;

#[function_component(Default)]
pub fn default() -> Html {
    html! {
        <div>
            <Selector/>
        </div>
    }
}