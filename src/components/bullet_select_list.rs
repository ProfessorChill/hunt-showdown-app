use yew::prelude::*;

use crate::content::{generic_item::CustomAmmo, GenericItem};

#[derive(PartialEq, Properties)]
pub struct BulletDisplayProps {
    pub weapon: GenericItem,
    pub bullet: CustomAmmo,
    pub on_bullet_clicked: Callback<CustomAmmo>,
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

    let bullet_svg_path = bullet.1.as_ref().map_or_else(
        || bullet.0.to_svg_path(Some(weapon)),
        |bullet_variant| bullet_variant.to_svg_path(Some(weapon), &bullet.0),
    );

    if let Some(bullet) = &bullet.1 {
        html! {
            <div class={classes!("item-display")} onclick={on_bullet_clicked_cb}>
                <img class={classes!("item-img")} src={bullet_svg_path} alt={bullet.to_string()} />
                <span class={classes!("item-name")}>{bullet.to_string()}</span>
            </div>
        }
    } else {
        html! {
            <div class={classes!("item-display")} onclick={on_bullet_clicked_cb}>
                <img class={classes!("item-img")} src={bullet_svg_path} alt={bullet.0.to_string()} />
                <span class={classes!("item-name")}>{bullet.0.to_string()}</span>
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

        let on_bullet_clicked = move |bullet: CustomAmmo| {
            on_bullet_selected_handle.emit((bullet, bullet_slot));
        };

        html! {
            <BulletDisplay weapon={weapon.clone()} bullet={bullet.clone()} on_bullet_clicked={on_bullet_clicked} />
        }
    }).collect::<Html>();

    let on_bullet_clicked = {
        let on_bullet_selected_handle = on_bullet_selected.clone();
        let bullet_slot = *bullet_slot;

        move |bullet: CustomAmmo| {
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
