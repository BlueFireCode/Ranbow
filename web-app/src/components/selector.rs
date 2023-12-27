use reqwasm::http::Request;
use shared::model::operator_display::OperatorDisplay;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use log::info;

async fn fetch_operator_displays() -> Vec<OperatorDisplay> {
    Request::get("http://192.168.0.146:8080/api/operator_displays/0").
        send().await.unwrap().
        json().await.unwrap()
}

#[function_component(Selector)]
pub fn selector() -> Html {
    let state = use_state(|| None);
    {
        let state = state.clone();
        use_effect_with((), move |_| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_state = fetch_operator_displays().await;
                state.set(Some(fetched_state));
            });
            || ()
        })
    }

    let on_change = Callback::from(move |value: OperatorDisplay| {
        match value.selected {
            Some(selected) => info!("on_change event called. OpId: {} - OpName: {} - Selected: {}", value.id, value.name, selected),
            None => {},
        };
    });

    if let Some(state) = &*state {
        html! {
            <div style="float: left;">
                <input class="collapsable-input" type="checkbox" id="collapsable" checked={true}/>
                <label class="collapsable-label ms-4 mt-3" for="collapsable">
                    <svg aria-hidden="true" focusable="false" role="img" class="mb-1" viewBox="0 0 16 16" width="16" height="16" fill="currentColor" style="display: inline-block; user-select: none; vertical-align: text-bottom; overflow: visible;">
                        <path d="m4.177 7.823 2.396-2.396A.25.25 0 0 1 7 5.604v4.792a.25.25 0 0 1-.427.177L4.177 8.177a.25.25 0 0 1 0-.354Z"></path>
                        <path d="M0 1.75C0 .784.784 0 1.75 0h12.5C15.216 0 16 .784 16 1.75v12.5A1.75 1.75 0 0 1 14.25 16H1.75A1.75 1.75 0 0 1 0 14.25Zm1.75-.25a.25.25 0 0 0-.25.25v12.5c0 .138.112.25.25.25H9.5v-13Zm12.5 13a.25.25 0 0 0 .25-.25V1.75a.25.25 0 0 0-.25-.25H11v13Z"></path>
                    </svg>
                </label>
                <div class="collapsable-content border-top border-end border-bottom border-secondary mt-3 p-2 w-25 selector-list"
                    data-bs-smooth-scroll="true">
                    <table class="table table-borderless">
                        <tbody>
                            {
                                state.into_iter().map(|operator_display| {
                                    html! {
                                        <OperatorCheckBox operator_display={operator_display.clone()} on_change={on_change.clone()}/>
                                    }
                                }).collect::<Html>()
                                }
                        </tbody>
                    </table>
                </div>
            </div>
        }
    } else {
        html! {
            <div>{"Loading..."}</div>
        }
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub operator_display: OperatorDisplay,
    pub on_change: Callback<OperatorDisplay>
}

fn get_value_from_change_event(e: Event, mut op: OperatorDisplay) -> OperatorDisplay {
    let event_target = e.target().unwrap();
    let target:HtmlInputElement = event_target.dyn_into().unwrap();
    op.selected = Some(target.checked());
    op
}

#[function_component(OperatorCheckBox)]
pub fn operator_check_box(props: &Props) -> Html {
    let props = props.clone();
    let op_display = props.operator_display.clone();
    let onchange = Callback::from(move |change_event: Event| {
        props.on_change.emit(get_value_from_change_event(change_event, op_display.clone()));
    });

    html!{
        <tr style="min-height: 60px;">
            <td class="form-check-label" style="background-color: transparent;">
                <input class="form-check-input custom-checkbox-input" type="checkbox" id={String::from("operator_check_") + props.operator_display.id.to_string().as_str()} checked={true} {onchange}/>
                <label class="custom-checkbox-content" for={String::from("operator_check_") + props.operator_display.id.to_string().as_str()}>
                    <img height="50" width="50" src={props.operator_display.icon_url.clone()}/>
                    <span width="100" class="mt-2">{props.operator_display.name.clone()}</span>
                </label>
            </td>
        </tr>
    }
}