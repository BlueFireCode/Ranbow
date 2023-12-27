use yew::prelude::*;

use crate::components::selector::Selector;

#[function_component(Default)]
pub fn default() -> Html {
    html! {
        <div style="display: flex;flex-direction: row;width: 100%">
            <Selector/>
            <div style="flex-grow: 1;" class="ms-4">
                <h1 class="mt-3">{"Default"}</h1>
                <p>{"asdf"}</p>
                <p>{"asdf"}</p>
                <p>{"asdf"}</p>
                <p>{"asdf"}</p>
                <p>{"asdf"}</p>
                <p>{"asdf"}</p>
                <p>{"asdf"}</p>
                <p>{"asdf"}</p>
                <p>{"asdf"}</p>
            </div>
        </div>
    }
}