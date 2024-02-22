use gloo_storage::{LocalStorage, Storage};
use rand::{seq::SliceRandom, Rng};
use shared::model::{operator::Operator, operator_display::OperatorDisplay, weapon::Weapon};

pub fn get_random_id(attacker: bool) -> Option<String> {
    let val: Vec<OperatorDisplay> = LocalStorage::get("ranbow_selected_operators").unwrap_or(Vec::<OperatorDisplay>::new());
    let selected: Vec<OperatorDisplay> = val.into_iter().filter(|e: &OperatorDisplay| e.attacker == attacker && e.selected.unwrap_or(true)).collect();
    let selected = selected.choose(&mut rand::thread_rng());
    if let Some(selected) = selected {
        Some(selected.id.to_string())
    } else {
        None
    }
}

pub fn randomize_operator(op: Operator) -> Option<Operator> {
    let mut op = op.clone();
    let mut rng = rand::thread_rng();

    let selected_primary = op.primaries.choose(&mut rng);
    op.selected_primary = match selected_primary {
        Some(prim) => randomize_weapon(prim.clone()),
        None => return None
    };

    let selected_secondary = op.secondaries.choose(&mut rng);
    op.selected_secondary = match selected_secondary {
        Some(sec) => randomize_weapon(sec.clone()),
        None => return None
    };

    let selected_gadget = op.gadgets.choose(&mut rng);
    op.selected_gadget = match selected_gadget {
        Some(gadget) => Some(gadget.clone()),
        None => return None
    };

    Some(op)
}

pub fn randomize_weapon(weapon: Weapon) -> Option<Weapon> {
    let mut weapon = weapon.clone();
    let mut rng = rand::thread_rng();

    if weapon.attachment_options.sight {
        weapon.attachments.selected_sight = Some(rng.gen_range(0..=weapon.attachments.sight));
    }

    if weapon.attachment_options.muzzle {
        let selected_muzzle = weapon.attachments.muzzle.choose(&mut rng);
        weapon.attachments.selected_muzzle = match selected_muzzle {
            Some(muzzle) => Some(muzzle.clone()),
            None => return None
        }
    }

    if weapon.attachment_options.grip {
        let selected_grip = weapon.attachments.grip.choose(&mut rng);
        weapon.attachments.selected_grip = match selected_grip {
            Some(grip) => Some(grip.clone()),
            None => return None
        }
    }

    if weapon.attachment_options.laser {
        weapon.attachments.laser_selected = Some(rng.gen_bool(0.5));
    }

    Some(weapon)
}