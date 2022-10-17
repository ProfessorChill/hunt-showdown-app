use yew::prelude::*;

use crate::content::{BulletSize, BulletVariant, GenericItem, Slot};
use crate::TRANSPARENT_B64;

#[derive(PartialEq, Properties)]
pub struct WeaponSlotProps {
    pub weapon: Option<GenericItem>,
    pub dual_wield: bool,
    pub locked: bool,
    pub ammo_types: Vec<(BulletSize, Option<BulletVariant>, u16)>,
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

    if let Some(weapon) = weapon {
        let weapon = weapon.clone();
        let weapon_slot_class = match weapon.slot {
            Some(Slot::Small) => "slot-small".to_string(),
            Some(Slot::Medium) => "slot-medium".to_string(),
            Some(Slot::Large) => "slot-large".to_string(),
            None => "slot-large".to_string(),
        };

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
                        format!("{} {}", bullet_size, bullet_variant),
                    ),
                    (bullet_size, None) => (
                        bullet_size.to_svg_path(Some(&weapon)),
                        format!("{}", bullet_size),
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
            <div class={classes!("weapon-slot", weapon_slot_class)}>
                <img
                    class={classes!("weapon-img", "item-selected")}
                    src={weapon.to_weapon_path()}
                    alt={weapon.name.clone()}
                    onclick={on_weapon_slot_clicked}
                />
                <div class={classes!("weapon-ammo-types")}>
                    {usage_type_html}
                </div>
                <div class={classes!("item-actions")}>
                    <img src={lock_src} alt={lock_alt} onclick={on_weapon_toggle_lock} />
                    <img
                        src="/images/icons/TrashCan.svg"
                        alt="Delete"
                        onclick={on_weapon_delete}
                    />
                    <img src="/images/icons/Dice.svg" alt="Randomize" onclick={on_weapon_randomize} />
                </div>
                {if *dual_wield {
                    Some(html! { <h1>{"DUAL WIELD"}</h1> })
                } else {
                    None
                }}
                <span class={classes!("weapon-name")}>{weapon.to_full_name()}</span>
            </div>
        }
    } else {
        html! {
            <div class={classes!("weapon-slot", "slot-large")}>
                <img class={classes!("weapon-img")} src={TRANSPARENT_B64} alt={""} onclick={on_weapon_slot_clicked} />
                <div class={classes!("item-actions")}>
                    <img src={lock_src} alt={lock_alt} onclick={on_weapon_toggle_lock} />
                    <img src="/images/icons/TrashCan.svg" alt="Delete" onclick={on_weapon_delete} />
                    <img src="/images/icons/Dice.svg" alt="Randomize" onclick={on_weapon_randomize} />
                </div>
            </div>
        }
    }
}
