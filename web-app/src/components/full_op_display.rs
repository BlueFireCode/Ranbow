use shared::model::{operator::Operator, weapon::Weapon};
use yew::prelude::*;

use crate::logic::helpers::u8_to_sight_name;

#[derive(Properties, PartialEq)]
pub struct FullDisplayProps {
    pub operator: Option<Operator>,
    pub tdm: bool
}

#[function_component(FullOpDisplay)]
pub fn full_op_display(props: &FullDisplayProps) -> Html {
    match props.operator.clone() {
        Some(op) => {
            let selected_primary = op.selected_primary.unwrap();
            let selected_secondary = op.selected_secondary.unwrap();
            let selected_gadget = op.selected_gadget.unwrap();
            html! {
                <div>
                    <h3 class="op-name-panel">
                        <img src={op.icon_url.clone()} class="me-2" height="65px" width="65px"/>
                        <p style="display: inline">{op.name.clone()}</p>
                    </h3>
                    <h4>
                        <img src={selected_primary.icon_url.clone()} height="100px" width="290px"/>
                        <p class="op-weapon-panel">{selected_primary.name.clone()}</p>
                    </h4>
                    <WeaponAttachmentDisplay weapon={Some(selected_primary)}/>
                    <h4>
                        <img src={selected_secondary.icon_url.clone()} height="100px" width="290px"/>
                        <p class="op-weapon-panel">{selected_secondary.name.clone()}</p>
                    </h4>
                    <WeaponAttachmentDisplay weapon={Some(selected_secondary)}/>
                    if !props.tdm {
                        <h4>
                            <img src={selected_gadget.icon_url.clone()} height="100px" width="290px"/>
                            <p class="op-weapon-panel">{selected_gadget.name.clone()}</p>
                        </h4>
                    }
                </div>
            }
        },
        None => html! { }
    }
}

#[derive(Properties, PartialEq)]
pub struct WeaponDisplayProps {
    pub weapon: Option<Weapon>
}

#[function_component(WeaponAttachmentDisplay)]
pub fn weapon_attachment_display(props: &WeaponDisplayProps) -> Html {
    match props.weapon.clone() {
        Some(weapon) => {
            html! {
                <div>
                    {
                        if weapon.attachment_options.sight {
                            html! { <p class="ms-4">{"Sight: "}{u8_to_sight_name(weapon.attachments.selected_sight.clone().unwrap())}</p> }
                        } else { html! {} }
                    }
                    {
                        if weapon.attachment_options.muzzle {
                            html! { <p class="ms-4">{"Muzzle: "}{weapon.attachments.selected_muzzle.clone().unwrap()}</p> }
                        } else { html! {} }
                    }
                    {
                        if weapon.attachment_options.grip {
                            html! { <p class="ms-4">{"Grip: "}{weapon.attachments.selected_grip.clone().unwrap()}</p> }
                        } else { html! {} }
                    }
                    {
                        if weapon.attachment_options.laser {
                            html! { <p class="ms-4">{"Laser: "}{if weapon.attachments.laser_selected.clone().unwrap() { "Yes" } else { "No" }}</p> }
                        } else { html!{} }
                    }
                </div>
            }
        },
        None => html! { }
    }
}
