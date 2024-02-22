use reqwasm::http::Request;
use shared::model::{operator::Operator, weapon::Weapon};
use yew::{function_component, html, use_effect_with, use_state, Callback, Html, Properties};

use crate::logic::helpers::u8_to_sight_name;

async fn fetch_operators(side: u8) -> Vec<Operator> {
    let url = format!("/api/operators/{}", side);
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
                class="btn btn-danger my-2 operators-button-left"
                onclick={on_atk}>
                {"Attackers"}
            </button>
            <button
                type="button"
                class="btn btn-primary my-2 operators-button-right"
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
                                            <a href={format!{"/Operator/{}", operator.id.to_string()}} class="link-body-emphasis">
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

async fn fetch_operator(id: String) -> Option<Operator> {
    let url = format!("/api/operator/{}", id);
    match Request::get(&url)
        .send().await.unwrap()
        .json().await {
        Ok(res) => Some(res),
        Err(err) => {
            log::error!("{}", err);
            None
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String
}

#[function_component(OperatorView)]
pub fn operator_view(props: &Props) -> Html {
    let id = props.id.clone();
    let state = use_state(|| None);
    {
        let state = state.clone();
        use_effect_with((), move |_| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_state = fetch_operator(id).await;
                state.set(Some(fetched_state));
            });
            || ()
        })
    }

    html! {
        <div class="ms-4 mt-3">
            {
                if let Some(state) = &*state {
                    if let Some(state) = state {
                        html! {
                            <div class="op-weapon-table-row">
                                <h3 class="op-name-panel">
                                    <img src={state.icon_url.clone()} class="me-2" height="65px" width="65px"/>
                                    <p style="display: inline">{state.name.clone()}</p>
                                </h3>
                                <hr class="op-panel-divider"/>
                                <div class="op-weapon-table-row">
                                {
                                    state.primaries.clone().into_iter().map(|primary| {
                                        html! {
                                            <WeaponDisplay weapon={primary}/>
                                        }
                                    }).collect::<Html>()
                                }
                                </div>
                                <hr class="op-panel-divider"/>
                                <div class="op-weapon-table-row">
                                {
                                    state.secondaries.clone().into_iter().map(|secondary| {
                                        html! {
                                            <WeaponDisplay weapon={secondary}/>
                                        }
                                    }).collect::<Html>()
                                }
                                </div>
                                <hr class="op-panel-divider"/>
                                <div class="op-weapon-table-row">
                                {
                                    state.gadgets.clone().into_iter().map(|gadget| {
                                        html! {
                                            <h4 class="op-weapon-table-cell">
                                                <img src={gadget.icon_url.clone()} height="100px" width="290px"/>
                                                <p class="op-weapon-panel">{gadget.name.clone()}</p>
                                            </h4>
                                        }
                                    }).collect::<Html>()
                                }
                                </div>
                            </div>
                        }
                    } else {
                        html! {
                            <div><img class="rounded mx-auto mt-5 d-block" src={"https://http.cat/404"}/></div>
                        }
                    }
                } else {
                    html! {
                        <div>{"Loading..."}</div>
                    }
                }
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct WeaponDisplayProps {
    pub weapon: Weapon
}

#[function_component(WeaponDisplay)]
pub fn weapon_display(props: &WeaponDisplayProps) -> Html {
    let weapon = props.weapon.clone();
    html! {
        <div class="weapon-display-panel op-weapon-table-cell">
            <h4>
                <button type="button" class="btn btn-link link-body-emphasis weapon-collapsable-button" data-bs-toggle="collapse" data-bs-target={format!("#{}", weapon.name.replace(" ", "_"))}>
                    <img src={weapon.icon_url.clone()} height="100px" width="290px"/>
                    {weapon.name.clone()}
                </button>
            </h4>
            <div class="collapse border border-light rounded p-2 mx-2" id={weapon.name.replace(" ", "_")}>
            {
                if weapon.attachment_options.sight ||
                    weapon.attachment_options.muzzle ||
                    weapon.attachment_options.grip ||
                    weapon.attachment_options.laser {
                    html! {
                        <div>
                        {
                            if weapon.attachment_options.sight {
                                html! {
                                    <ul> {"Sights:"}
                                    {
                                        (0..=weapon.attachments.sight).collect::<Vec<_>>().into_iter().map(|sight| {
                                            html! { <li class="ms-4">{u8_to_sight_name(sight)}</li> }
                                        }).collect::<Html>()
                                    }
                                    </ul>
                                }
                            } else { html! {} }
                        }
                        {
                            if weapon.attachment_options.muzzle {
                                html! {
                                    <ul> {"Muzzles:"}
                                    {
                                        weapon.attachments.muzzle.into_iter().map(|muzzle| {
                                            html! { <li class="ms-4">{muzzle.clone()}</li> }
                                        }).collect::<Html>()
                                    }
                                    </ul>
                                }
                            } else { html! {} }
                        }
                        {
                            if weapon.attachment_options.grip {
                                html! {
                                    <ul> {"Grips:"}
                                    {
                                        weapon.attachments.grip.into_iter().map(|grip| {
                                            html! { <li class="ms-4">{grip.clone()}</li> }
                                        }).collect::<Html>()
                                    }
                                    </ul>
                                }
                            } else { html! {} }
                        }
                        {
                            if weapon.attachment_options.laser {
                                html! {
                                    <ul> {"Underbarrel:"}
                                        <li class="ms-4">{"None"}</li>
                                        <li class="ms-4">{"Laser"}</li>
                                    </ul>
                                }
                            } else { html! {} }
                        }
                        </div>
                    }
                } else {
                    html! {
                        <div>{"Idk, your skin maybe..."}</div>
                    }
                }
            }
            </div>
        </div>
    }
}