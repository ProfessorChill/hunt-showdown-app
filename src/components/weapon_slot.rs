use yew::prelude::*;

use crate::content::{generic_item::CustomAmmo, GenericItem};
use crate::TRANSPARENT_B64;

#[derive(PartialEq, Properties)]
pub struct WeaponSlotProps {
    pub weapon: Option<GenericItem>,
    pub dual_wield: bool,
    pub locked: bool,
    pub ammo_types: Vec<CustomAmmo>,
    pub on_weapon_slot_clicked: Callback<MouseEvent>,
    pub on_weapon_toggle_lock: Callback<MouseEvent>,
    pub on_weapon_delete: Callback<MouseEvent>,
    pub on_weapon_randomize: Callback<MouseEvent>,
    pub on_ammo_slot_clicked: Callback<usize>,
}

#[function_component]
pub fn WeaponSlot(props: &WeaponSlotProps) -> Html {
    let WeaponSlotProps {
        weapon,
        dual_wield,
        locked,
        ammo_types,
        on_weapon_slot_clicked,
        on_weapon_toggle_lock,
        on_weapon_delete,
        on_weapon_randomize,
        on_ammo_slot_clicked,
    } = props;

    let (lock_src, lock_alt) = if *locked {
        ("/images/icons/Lock.svg", "Locked")
    } else {
        ("/images/icons/Unlock.svg", "Unlocked")
    };

    weapon.as_ref().map_or_else(|| html! {
        <div class={classes!("weapon-slot")}>
            <div class={classes!("is-flex")}>
                <div class={classes!("item-actions")}>
                    <img src={lock_src} alt={lock_alt} onclick={on_weapon_toggle_lock} />
                    <img src="/images/icons/TrashCan.svg" alt="Delete" onclick={on_weapon_delete} />
                    <img src="/images/icons/Dice.svg" alt="Randomize" onclick={on_weapon_randomize} />
                </div>
            </div>
            <img class={classes!("weapon-img")} src={TRANSPARENT_B64} alt={""} onclick={on_weapon_slot_clicked} />
        </div>
    }, |weapon| {
        let weapon = weapon.clone();

        // Shoot takes priority, if there is none return html!{}.
        let usage_type_html = ammo_types
            .iter()
            .enumerate()
            .map(|(slot, ammo_type)| {
                let on_ammo_slot_clicked = on_ammo_slot_clicked.clone();
                let on_ammo_slot_clicked_cb = Callback::from(move |_| {
                    on_ammo_slot_clicked.emit(slot);
                });

                let (src, alt) = match (&ammo_type.0, &ammo_type.1) {
                    (bullet_size, Some(bullet_variant)) => (
                        bullet_variant.to_svg_path(Some(&weapon), bullet_size),
                        format!("{bullet_size} {bullet_variant}"),
                    ),
                    (bullet_size, None) => (
                        bullet_size.to_svg_path(Some(&weapon)),
                        format!("{bullet_size}"),
                    ),
                };

                html! {
                    <img
                        class={classes!("ammo-img")}
                        src={src}
                        alt={alt}
                        onclick={on_ammo_slot_clicked_cb}
                    />
                }
            })
            .collect::<Html>();

        html! {
            <div class={classes!("weapon-slot")}>
                <div class={classes!("is-flex")}>
                    <div class={classes!("is-flex-grow-1", "item-actions")}>
                        <img src={lock_src} alt={lock_alt} onclick={on_weapon_toggle_lock} />
                        <img
                            src="/images/icons/TrashCan.svg"
                            alt="Delete"
                            onclick={on_weapon_delete}
                        />
                        <img src="/images/icons/Dice.svg" alt="Randomize" onclick={on_weapon_randomize} />
                    </div>

                    if *dual_wield {
                        <h1 class={classes!("is-flex-grow-1")}>{"DUAL WIELD"}</h1>
                    }

                    <div>
                        {usage_type_html}
                    </div>
                </div>
                <img
                    class={classes!("weapon-img", "item-selected")}
                    src={weapon.to_weapon_path()}
                    alt={weapon.name.clone()}
                    onclick={on_weapon_slot_clicked}
                />
                <span class={classes!("weapon-name")}>{weapon.to_full_name()}</span>
            </div>
        }
    })
}
