use yew::prelude::*;

use crate::content::GenericItem;
use crate::TRANSPARENT_B64;

#[derive(PartialEq, Properties)]
pub struct ConsumableSlotProps {
    pub consumable: Option<GenericItem>,
    pub locked: bool,
    pub id: usize,
    pub on_consumable_slot_clicked: Callback<usize>,
    pub on_consumable_toggle_lock: Callback<usize>,
    pub on_consumable_delete: Callback<usize>,
    pub on_consumable_randomize: Callback<usize>,
}

#[function_component]
pub fn ConsumableSlot(props: &ConsumableSlotProps) -> Html {
    let ConsumableSlotProps {
        consumable,
        locked,
        id,
        on_consumable_slot_clicked,
        on_consumable_toggle_lock,
        on_consumable_delete,
        on_consumable_randomize,
    } = props;

    let id = *id;

    let (lock_src, lock_alt) = if *locked {
        ("/images/icons/Lock.svg", "Locked")
    } else {
        ("/images/icons/Unlock.svg", "Unlocked")
    };

    let on_consumable_clicked_handle = {
        let on_consumable_slot_clicked = on_consumable_slot_clicked.clone();

        move |_: MouseEvent| {
            on_consumable_slot_clicked.emit(id);
        }
    };

    let on_consumable_toggle_lock_handle = {
        let on_consumable_toggle_lock = on_consumable_toggle_lock.clone();

        move |_: MouseEvent| {
            on_consumable_toggle_lock.emit(id);
        }
    };

    let on_consumable_delete_handle = {
        let on_consumable_delete = on_consumable_delete.clone();

        move |_: MouseEvent| {
            on_consumable_delete.emit(id);
        }
    };

    let on_consumable_randomize_handle = {
        let on_consumable_randomize = on_consumable_randomize.clone();

        move |_: MouseEvent| {
            on_consumable_randomize.emit(id);
        }
    };

    if let Some(consumable) = consumable {
        let consumable = consumable.clone();

        html! {
            <div class={classes!("consumable-slot")}>
                <div class={classes!("item-actions")}>
                    <img src={lock_src} alt={lock_alt} onclick={on_consumable_toggle_lock_handle} />
                    <img src="/images/icons/TrashCan.svg" alt="Delete" onclick={on_consumable_delete_handle} />
                    <img src="/images/icons/Dice.svg" alt="Randomize" onclick={on_consumable_randomize_handle} />
                </div>
                <img
                    class={classes!("consumable-img", "item-selected")}
                    src={consumable.to_consumable_path()}
                    alt={consumable.name.clone()}
                    onclick={on_consumable_clicked_handle}
                />
                <span class={classes!("consumable-name")}>{consumable.name}</span>
            </div>
        }
    } else {
        html! {
            <div class={classes!("consumable-slot")}>
                <div class={classes!("item-actions")}>
                    <img src={lock_src} alt={lock_alt} onclick={on_consumable_toggle_lock_handle} />
                    <img src="/images/icons/TrashCan.svg" alt="Delete" onclick={on_consumable_delete_handle} />
                    <img src="/images/icons/Dice.svg" alt="Randomize" onclick={on_consumable_randomize_handle} />
                </div>
                <img
                    class={classes!("consumable-img")}
                    src={TRANSPARENT_B64}
                    alt={""}
                    onclick={on_consumable_clicked_handle}
                />
            </div>
        }
    }
}
