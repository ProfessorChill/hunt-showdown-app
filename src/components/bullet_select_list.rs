use yew::prelude::*;

use crate::content::generic_item::CustomAmmo;
use crate::content::{BulletSize, BulletVariant, GenericItem};

#[derive(PartialEq, Properties)]
pub struct BulletDisplayProps {
    pub weapon: GenericItem,
    pub bullet: (BulletSize, Option<BulletVariant>, u16),
    pub on_bullet_clicked: Callback<(BulletSize, Option<BulletVariant>, u16)>,
}

#[function_component]
pub fn BulletDisplay(props: &BulletDisplayProps) -> Html {
    let BulletDisplayProps {
        weapon,
        bullet,
        on_bullet_clicked,
    } = props;

    let on_bullet_clicked_cb = {
        let on_bullet_clicked_handle = on_bullet_clicked.clone();
        let bullet_clone = bullet.clone();

        move |_| {
            on_bullet_clicked_handle.emit(bullet_clone.clone());
        }
    };

    let bullet_svg_path = if let Some(bullet_variant) = &bullet.1 {
        bullet_variant.to_svg_path(Some(weapon), &bullet.0)
    } else {
        bullet.0.to_svg_path(Some(weapon))
    };

    if let Some(bullet) = &bullet.1 {
        html! {
            <div class={classes!("item-display")} onclick={on_bullet_clicked_cb}>
                <img class={classes!("item-img")} src={bullet_svg_path} alt={format!("{}", bullet)} />
                <span class={classes!("item-name")}>{format!("{}", bullet)}</span>
            </div>
        }
    } else {
        html! {
            <div class={classes!("item-display")} onclick={on_bullet_clicked_cb}>
                <img class={classes!("item-img")} src={bullet_svg_path} alt={format!("{}", bullet.0)} />
                <span class={classes!("item-name")}>{format!("{}", bullet.0)}</span>
            </div>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct BulletSelectListProps {
    pub weapon: GenericItem,
    pub bullet_slot: usize,
    pub on_bullet_selected: Callback<(CustomAmmo, usize)>,
}

#[function_component]
pub fn BulletSelectList(props: &BulletSelectListProps) -> Html {
    let BulletSelectListProps {
        weapon,
        bullet_slot,
        on_bullet_selected,
    } = props;

    let bullet_displays = weapon.get_bullet_variants().iter().map(|bullet| {
        let on_bullet_selected_handle = on_bullet_selected.clone();
        let bullet_slot = *bullet_slot;

        let on_bullet_clicked = move |bullet: (BulletSize, Option<BulletVariant>, u16)| {
            on_bullet_selected_handle.emit((bullet, bullet_slot));
        };

        html! {
            <BulletDisplay weapon={weapon.clone()} bullet={bullet.clone()} on_bullet_clicked={on_bullet_clicked} />
        }
    }).collect::<Html>();

    let on_bullet_clicked = {
        let on_bullet_selected_handle = on_bullet_selected.clone();
        let bullet_slot = *bullet_slot;

        move |bullet: (BulletSize, Option<BulletVariant>, u16)| {
            on_bullet_selected_handle.emit((bullet, bullet_slot));
        }
    };

    html! {
        <div class={classes!("item-list-container")}>
            <div class={classes!("item-list-inner")}>
                <BulletDisplay
                    weapon={weapon.clone()}
                    bullet={(weapon.get_bullet_size().expect("Bullet size not found on weapon"), None, 0)}
                    on_bullet_clicked={on_bullet_clicked}
                />
                {bullet_displays}
            </div>
        </div>
    }
}
