use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::content::{GenericItem, ItemVariant, CORE_SEARCH_UTIL};

#[derive(PartialEq, Properties)]
pub struct ItemDisplayProps {
    pub item: GenericItem,
    pub on_item_clicked: Callback<GenericItem>,
}

#[function_component]
pub fn ItemDisplay(props: &ItemDisplayProps) -> Html {
    let ItemDisplayProps {
        item,
        on_item_clicked,
    } = props;

    let on_item_clicked_cb = {
        let on_item_clicked_handle = on_item_clicked.clone();
        let item_clone = item.clone();

        Callback::from(move |_e: MouseEvent| {
            on_item_clicked_handle.emit(item_clone.clone());
        })
    };

    html! {
        <div class={classes!("item-display")} onclick={on_item_clicked_cb}>
            <img class={classes!("item-img")} src={item.to_image_path()} alt={item.name.clone()} />
            <span class={classes!("item-name")}>{item.to_full_name()}</span>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct ItemSelectListProps {
    pub id: usize,
    pub items: Vec<GenericItem>,
    pub item_variant: ItemVariant,
    pub on_item_selected: Callback<(Option<GenericItem>, usize)>,
}

#[function_component]
pub fn ItemSelectList(props: &ItemSelectListProps) -> Html {
    let ItemSelectListProps {
        id,
        items,
        item_variant,
        on_item_selected,
    } = props;

    let id = *id;

    let dual_wield_handle = use_state(|| false);
    let dual_wield = *dual_wield_handle;

    let search_terms_handle = use_state(String::new);
    let search_terms = (*search_terms_handle).clone();
    let search_items_handle = use_state(|| items.clone());
    let search_items = (*search_items_handle).clone();

    let on_dual_wield_changed = {
        let search_items_handle = search_items_handle.clone();
        let item_variant = item_variant.clone();
        let dual_wield = !dual_wield;

        move |_| {
            let mut item_set = if dual_wield {
                match &item_variant {
                    ItemVariant::Weapon => CORE_SEARCH_UTIL
                        .get_dual_wield_weapons()
                        .iter_mut()
                        .filter_map(|weapon| {
                            if weapon
                                .to_full_name()
                                .to_lowercase()
                                .contains(&search_terms.to_lowercase())
                            {
                                let mut weapon = weapon.clone();
                                weapon.dual_wield = true;
                                Some(weapon)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<GenericItem>>(),
                    _ => unreachable!("Non-weapons should never see dual wield!"),
                }
            } else {
                match &item_variant {
                    ItemVariant::Weapon => CORE_SEARCH_UTIL
                        .weapons
                        .iter()
                        .filter_map(|weapon| {
                            if weapon
                                .to_full_name()
                                .to_lowercase()
                                .contains(&search_terms.to_lowercase())
                            {
                                Some(weapon.clone())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<GenericItem>>(),
                    _ => unreachable!("Non-weapons should never see dual wield!"),
                }
            };

            item_set.sort_by_key(GenericItem::to_full_name);

            search_items_handle.set(item_set);
            dual_wield_handle.set(dual_wield);
        }
    };

    let on_search_field_input = {
        let item_variant = item_variant.clone();

        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();

            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                let mut item_set = if dual_wield {
                    match &item_variant {
                        ItemVariant::Weapon => CORE_SEARCH_UTIL
                            .get_dual_wield_weapons()
                            .iter_mut()
                            .filter_map(|weapon| {
                                if weapon
                                    .to_full_name()
                                    .to_lowercase()
                                    .contains(&input.value().to_lowercase())
                                {
                                    let mut weapon = weapon.clone();
                                    weapon.dual_wield = true;
                                    Some(weapon)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<GenericItem>>(),
                        _ => vec![],
                    }
                } else {
                    match &item_variant {
                        ItemVariant::Weapon => CORE_SEARCH_UTIL
                            .weapons
                            .iter()
                            .filter(|weapon| {
                                weapon
                                    .to_full_name()
                                    .to_lowercase()
                                    .contains(&input.value().to_lowercase())
                            })
                            .cloned()
                            .collect::<Vec<GenericItem>>(),
                        ItemVariant::Consumable => CORE_SEARCH_UTIL
                            .consumables
                            .iter()
                            .filter(|consumable| {
                                consumable
                                    .to_full_name()
                                    .to_lowercase()
                                    .contains(&input.value().to_lowercase())
                            })
                            .cloned()
                            .collect::<Vec<GenericItem>>(),
                        ItemVariant::Tool => CORE_SEARCH_UTIL
                            .tools
                            .iter()
                            .filter(|tool| {
                                tool.to_full_name()
                                    .to_lowercase()
                                    .contains(&input.value().to_lowercase())
                            })
                            .cloned()
                            .collect::<Vec<GenericItem>>(),
                    }
                };

                item_set.sort_by_key(GenericItem::to_full_name);

                search_terms_handle.set(input.value());
                search_items_handle.set(item_set);
            }
        })
    };

    html! {
        <div class={classes!("item-list-container")}>
            <input
                class={classes!("input")}
                type="text"
                placeholder="Search"
                oninput={on_search_field_input}
            />
            {if item_variant == &ItemVariant::Weapon {
                Some(html! {
                    <div class={classes!("item-list-options")}>
                        <label class={classes!("checkbox")}>
                            <input
                                type="checkbox"
                                checked={dual_wield}
                                onchange={on_dual_wield_changed}
                            />
                            {"Dual Wield"}
                        </label>
                    </div>
                })
            } else { None }}
            <div class={classes!("item-list-inner")}>
                {search_items.iter().map(|item| {
                    let on_item_selected_handle = on_item_selected.clone();

                    let on_item_clicked = move |item: GenericItem| {
                        on_item_selected_handle.emit((Some(item), id));
                    };

                    html! {
                        <ItemDisplay item={item.clone()} on_item_clicked={on_item_clicked} />
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}
