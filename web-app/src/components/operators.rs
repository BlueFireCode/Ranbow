use reqwasm::http::Request;
use shared::model::operator::Operator;
use yew::{function_component, html, use_effect_with, use_state, Callback, Html, Properties};

use crate::environment;

async fn fetch_operators(side: u8) -> Vec<Operator> {
    let url = format!("{}/api/operators/{}", environment::ranbow_host_url(), side);
    Request::get(&url)
        .send().await.unwrap()
        .json().await.unwrap()
}

#[function_component(Operators)]
pub fn operators() -> Html {
    let state = use_state(|| None);
    {
        let state = state.clone();
        use_effect_with((), move |_| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_state = fetch_operators(1).await;
                state.set(Some(fetched_state));
            });
            || ()
        })
    }

    let atk_state = state.clone();
    let on_atk = Callback::from(move |_| {
        let state = atk_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_state = fetch_operators(1).await;
                state.set(Some(fetched_state));
            });
    });

    let def_state = state.clone();
    let on_def = Callback::from(move |_| {
        let state = def_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_state = fetch_operators(2).await;
                state.set(Some(fetched_state));
            });
    });

    html! {
        <div>
            <button
                type="button"
                style="width: 49%; float: left; margin-start: 5px;"
                class="btn btn-danger my-2"
                onclick={on_atk}>
                {"Attackers"}
            </button>
            <button
                type="button"
                style="width: 49%; float: right; margin-end: 5px;"
                class="btn btn-primary my-2"
                onclick={on_def}>
                {"Defenders"}
            </button>
            <table class="table table-striped mb-5">
                <thead>
                    <tr>
                        <th scope="col"></th>
                        <th scope="col">{"Name"}</th>
                        <th scope="col">{"Primaries"}</th>
                        <th scope="col">{"Secondaries"}</th>
                        <th scope="col">{"Gadgets"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        if let Some(state) = &*state {
                            state.into_iter().map(|operator| {
                                html!{
                                    <tr>
                                        <th style="vertical-align: middle;" scope="row" height="70" width="70"><img src={operator.icon_url.clone()} height="50" width="50"/></th>
                                        <td style="vertical-align: middle;" width="16%">
                                            <a href={format!{"{}/Operator/{}", environment::ranbow_host_url(), operator.id.to_string()}} class="link-body-emphasis">
                                                {operator.name.clone()}
                                            </a>
                                        </td>
                                        <td style="vertical-align: middle;" width="28%">
                                        {
                                            operator.primaries.clone().into_iter().map(|primary| {
                                                html! {
                                                    <div>
                                                        <img src={primary.icon_url.clone()} style="display: inline" height="40px" width="116px"/>
                                                        <p style="display: inline" class="ms-2">{primary.name.clone()}</p>
                                                    </div>
                                                }
                                            }).collect::<Html>()
                                        }
                                        </td>
                                        <td style="vertical-align: middle;" width="28%">
                                        {
                                            operator.secondaries.clone().into_iter().map(|secondary| {
                                                html! {
                                                    <div>
                                                        <img src={secondary.icon_url.clone()} style="display: inline" height="40px" width="116px"/>
                                                        <p style="display: inline" class="ms-2">{secondary.name.clone()}</p>
                                                    </div>
                                                }
                                            }).collect::<Html>()
                                        }
                                        </td>
                                        <td style="vertical-align: middle;" width="28%">
                                        {
                                            operator.gadgets.clone().into_iter().map(|gadget| {
                                                html! {
                                                    <div>
                                                        <img src={gadget.icon_url.clone()} style="display: inline" height="40px" width="116px"/>
                                                        <p style="display: inline" class="ms-2">{gadget.name.clone()}</p>
                                                    </div>
                                                }
                                            }).collect::<Html>()
                                        }
                                        </td>
                                    </tr>
                                }
                            }).collect::<Html>()
                        } else {
                            html! { <div>{"Loading..."}</div> }
                        }
                    }
                </tbody>
            </table>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String
}

#[function_component(OperatorView)]
pub fn operator_view(_props: &Props) -> Html {
    html! {
        <div class="ms-4 mt-3">
            <h2 class="text-warning">{"Under construction"}</h2>
        </div>
    }
}