use yew::prelude::*;

use crate::components::{
    AdvancedOptions, BudgetDisplay, BulletSelectList, ConsumableSlot, ItemSelectList, ToolSlot,
    WeaponSlot,
};
use crate::content::{generic_item::CustomAmmo, GenericItem, ItemVariant, CORE_SEARCH_UTIL};
use crate::randomizer::budget::Transaction;
use crate::randomizer::{
    budget, config::ToggleOption, loadout, Budget, Config, Loadout, LoadoutInvalid,
};

#[function_component]
pub fn RandomLoadout() -> Html {
    let budget_handle = use_state(Budget::default);
    let budget = (*budget_handle).clone();

    let config_handle = use_state(Config::default);
    let config = (*config_handle).clone();

    let on_dual_wield_change = {
        let config = config.clone();
        let config_handle = config_handle.clone();

        move |_| {
            let mut config = config.clone();

            config.toggle_option(ToggleOption::DualWield);

            if !config.option_exists(ToggleOption::DualWield) {
                config.remove_option(ToggleOption::AlwaysDualWield);
            }

            config_handle.set(config);
        }
    };

    let on_duplicate_weapons_change = {
        let config = config.clone();
        let config_handle = config_handle.clone();

        move |_| {
            let mut config = config.clone();

            config.toggle_option(ToggleOption::DuplicateWeapons);

            if !config.option_exists(ToggleOption::DuplicateWeapons) {
                config.remove_option(ToggleOption::AlwaysDuplicateWeapons);
            }

            config_handle.set(config);
        }
    };

    let on_custom_ammo_change = {
        let config = config.clone();
        let config_handle = config_handle.clone();

        move |_| {
            let mut config = config.clone();

            config.toggle_option(ToggleOption::CustomAmmo);

            if !config.option_exists(ToggleOption::CustomAmmo) {
                config.remove_option(ToggleOption::AlwaysCustomAmmo);
            }

            config_handle.set(config);
        }
    };

    let on_quartermaster_change = {
        let config = config.clone();
        let config_handle = config_handle.clone();

        move |_| {
            let mut config = config.clone();

            config.toggle_option(ToggleOption::Quartermaster);

            if !config.option_exists(ToggleOption::Quartermaster) {
                config.remove_option(ToggleOption::AlwaysQuartermaster);
            }

            config_handle.set(config);
        }
    };

    let weapon_one_dropdown_handle = use_state(|| false);
    let weapon_one_dropdown = *weapon_one_dropdown_handle;

    let weapon_two_dropdown_handle = use_state(|| false);
    let weapon_two_dropdown = *weapon_two_dropdown_handle;

    let tools_dropdown_handle = use_state(|| [false; 4]);
    let tools_dropdown = *tools_dropdown_handle;

    let cons_dropdown_handle = use_state(|| [false; 4]);
    let cons_dropdown = *cons_dropdown_handle;

    let weapon_one_ammo_slot_handle = use_state(|| None);
    let weapon_one_ammo_slot = *weapon_one_ammo_slot_handle;

    let weapon_two_ammo_slot_handle = use_state(|| None);
    let weapon_two_ammo_slot = *weapon_two_ammo_slot_handle;

    let on_weapon_one_clicked = {
        let weapon_one_dropdown_handle = weapon_one_dropdown_handle.clone();
        let weapon_one_ammo_slot_handle = weapon_one_ammo_slot_handle.clone();
        let weapon_two_dropdown_handle = weapon_two_dropdown_handle.clone();
        let weapon_two_ammo_slot_handle = weapon_two_ammo_slot_handle.clone();
        let tools_dropdown_handle = tools_dropdown_handle.clone();
        let cons_dropdown_handle = cons_dropdown_handle.clone();

        move |_e: MouseEvent| {
            weapon_one_dropdown_handle.set(!weapon_one_dropdown);
            weapon_two_dropdown_handle.set(false);
            tools_dropdown_handle.set([false; 4]);
            cons_dropdown_handle.set([false; 4]);
            weapon_one_ammo_slot_handle.set(None);
            weapon_two_ammo_slot_handle.set(None);
        }
    };

    let on_weapon_two_clicked = {
        let weapon_one_dropdown_handle = weapon_one_dropdown_handle.clone();
        let weapon_one_ammo_slot_handle = weapon_one_ammo_slot_handle.clone();
        let weapon_two_dropdown_handle = weapon_two_dropdown_handle.clone();
        let weapon_two_ammo_slot_handle = weapon_two_ammo_slot_handle.clone();
        let tools_dropdown_handle = tools_dropdown_handle.clone();
        let cons_dropdown_handle = cons_dropdown_handle.clone();

        move |_e: MouseEvent| {
            weapon_one_dropdown_handle.set(false);
            weapon_two_dropdown_handle.set(!weapon_two_dropdown);
            tools_dropdown_handle.set([false; 4]);
            cons_dropdown_handle.set([false; 4]);
            weapon_one_ammo_slot_handle.set(None);
            weapon_two_ammo_slot_handle.set(None);
        }
    };

    let loadout_handle = use_state(Loadout::default);
    let loadout = (*loadout_handle).clone();
    let on_generate_loadout_clicked = {
        let loadout = loadout.clone();
        let config = config.clone();
        let budget = budget.clone();
        let loadout_handle = loadout_handle.clone();
        let budget_handle = budget_handle.clone();
        let weapon_one_dropdown_handle = weapon_one_dropdown_handle.clone();
        let weapon_one_ammo_slot_handle = weapon_one_ammo_slot_handle.clone();
        let weapon_two_dropdown_handle = weapon_two_dropdown_handle.clone();
        let weapon_two_ammo_slot_handle = weapon_two_ammo_slot_handle.clone();
        let tools_dropdown_handle = tools_dropdown_handle.clone();
        let cons_dropdown_handle = cons_dropdown_handle.clone();

        move |_e: MouseEvent| {
            let mut loadout = loadout.clone();
            let mut budget = budget.clone();
            weapon_one_dropdown_handle.set(false);
            weapon_two_dropdown_handle.set(false);
            tools_dropdown_handle.set([false; 4]);
            cons_dropdown_handle.set([false; 4]);
            weapon_one_ammo_slot_handle.set(None);
            weapon_two_ammo_slot_handle.set(None);
            loadout::random(&mut loadout, &mut budget, &config);

            budget_handle.set(budget);
            loadout_handle.set(loadout);
        }
    };

    let on_weapon_one_selected = {
        let loadout_handle = loadout_handle.clone();
        let budget_handle = budget_handle.clone();
        let loadout = loadout.clone();
        let budget = budget.clone();
        let weapon_one_dropdown_handle = weapon_one_dropdown_handle.clone();

        move |(item, _): (Option<GenericItem>, usize)| {
            let mut loadout = loadout.clone();
            let mut budget = budget.clone();
            loadout::refund_item(&mut budget, &loadout.weapon_one);
            loadout.weapon_one.item = item;
            loadout.weapon_one.locked = true;
            let _ = loadout::set_default_ammo(&mut loadout.weapon_one);
            loadout::purchase_item(&mut budget, &loadout.weapon_one);
            weapon_one_dropdown_handle.set(false);
            budget_handle.set(budget);
            loadout_handle.set(loadout);
        }
    };

    let on_weapon_two_selected = {
        let loadout_handle = loadout_handle.clone();
        let budget_handle = budget_handle.clone();
        let loadout = loadout.clone();
        let budget = budget.clone();
        let weapon_two_dropdown_handle = weapon_two_dropdown_handle.clone();

        move |(item, _): (Option<GenericItem>, usize)| {
            let mut loadout = loadout.clone();
            let mut budget = budget.clone();
            loadout::refund_item(&mut budget, &loadout.weapon_two);
            loadout.weapon_two.item = item;
            loadout.weapon_two.locked = true;
            let _ = loadout::set_default_ammo(&mut loadout.weapon_two);
            loadout::purchase_item(&mut budget, &loadout.weapon_two);
            weapon_two_dropdown_handle.set(false);
            budget_handle.set(budget);
            loadout_handle.set(loadout);
        }
    };

    let on_weapon_one_toggle_lock = {
        let loadout_handle = loadout_handle.clone();
        let loadout = loadout.clone();

        move |_: MouseEvent| {
            let mut loadout = loadout.clone();
            loadout.weapon_one.locked = !loadout.weapon_one.locked;
            loadout_handle.set(loadout);
        }
    };

    let on_weapon_two_toggle_lock = {
        let loadout_handle = loadout_handle.clone();
        let loadout = loadout.clone();

        move |_: MouseEvent| {
            let mut loadout = loadout.clone();
            loadout.weapon_two.locked = !loadout.weapon_two.locked;
            loadout_handle.set(loadout);
        }
    };

    let on_weapon_one_delete = {
        let budget_handle = budget_handle.clone();
        let loadout_handle = loadout_handle.clone();
        let loadout = loadout.clone();
        let budget = budget.clone();

        move |_: MouseEvent| {
            let mut loadout = loadout.clone();
            let mut budget = budget.clone();
            loadout::refund_item(&mut budget, &loadout.weapon_one);
            loadout.weapon_one.item = None;
            loadout.weapon_one.locked = false;
            budget_handle.set(budget);
            loadout_handle.set(loadout);
        }
    };

    let on_weapon_two_delete = {
        let budget_handle = budget_handle.clone();
        let loadout_handle = loadout_handle.clone();
        let loadout = loadout.clone();
        let budget = budget.clone();

        move |_: MouseEvent| {
            let mut loadout = loadout.clone();
            let mut budget = budget.clone();
            loadout::refund_item(&mut budget, &loadout.weapon_two);
            loadout.weapon_two.item = None;
            loadout.weapon_two.locked = false;
            budget_handle.set(budget);
            loadout_handle.set(loadout);
        }
    };

    let on_weapon_one_randomize = {
        let budget_handle = budget_handle.clone();
        let loadout_handle = loadout_handle.clone();
        let loadout = loadout.clone();
        let config = config.clone();
        let budget = budget.clone();

        move |_: MouseEvent| {
            let mut loadout = loadout.clone();
            let mut budget = budget.clone();
            loadout.weapon_one.locked = false;
            loadout::random_weapon_one(&mut loadout, &mut budget, &config);
            loadout_handle.set(loadout);
            budget_handle.set(budget);
        }
    };

    let on_weapon_two_randomize = {
        let budget_handle = budget_handle.clone();
        let loadout_handle = loadout_handle.clone();
        let loadout = loadout.clone();
        let config = config.clone();
        let budget = budget.clone();

        move |_: MouseEvent| {
            let mut loadout = loadout.clone();
            let mut budget = budget.clone();
            loadout.weapon_two.locked = false;
            loadout::random_weapon_two(&mut loadout, &mut budget, &config);
            loadout_handle.set(loadout);
            budget_handle.set(budget);
        }
    };

    let on_weapon_one_ammo_slot_toggled = {
        let weapon_one_dropdown_handle = weapon_one_dropdown_handle.clone();
        let weapon_one_ammo_slot_handle = weapon_one_ammo_slot_handle.clone();
        let weapon_two_dropdown_handle = weapon_two_dropdown_handle.clone();
        let weapon_two_ammo_slot_handle = weapon_two_ammo_slot_handle.clone();
        let tools_dropdown_handle = tools_dropdown_handle.clone();
        let cons_dropdown_handle = cons_dropdown_handle.clone();

        move |slot: usize| {
            weapon_one_dropdown_handle.set(false);
            weapon_two_dropdown_handle.set(false);
            tools_dropdown_handle.set([false; 4]);
            cons_dropdown_handle.set([false; 4]);
            weapon_one_ammo_slot_handle.set(Some(slot));
            weapon_two_ammo_slot_handle.set(None);
        }
    };

    let on_weapon_two_ammo_slot_toggled = {
        let weapon_one_dropdown_handle = weapon_one_dropdown_handle.clone();
        let weapon_one_ammo_slot_handle = weapon_one_ammo_slot_handle.clone();
        let weapon_two_dropdown_handle = weapon_two_dropdown_handle.clone();
        let weapon_two_ammo_slot_handle = weapon_two_ammo_slot_handle.clone();
        let tools_dropdown_handle = tools_dropdown_handle.clone();
        let cons_dropdown_handle = cons_dropdown_handle.clone();

        move |slot: usize| {
            weapon_one_dropdown_handle.set(false);
            weapon_two_dropdown_handle.set(false);
            tools_dropdown_handle.set([false; 4]);
            cons_dropdown_handle.set([false; 4]);
            weapon_one_ammo_slot_handle.set(None);
            weapon_two_ammo_slot_handle.set(Some(slot));
        }
    };

    let weapon_one_bullet_select_html = {
        let loadout = loadout.clone();
        let budget = budget.clone();
        let weapon_one_ammo_slot_handle = weapon_one_ammo_slot_handle.clone();
        let weapon_two_ammo_slot_handle = weapon_two_ammo_slot_handle.clone();

        loadout.weapon_one.item.as_ref().map_or_else(|| html! {}, |weapon| {
            let weapon = weapon.clone();

            weapon.ammo_equipped.iter().enumerate().map(|(slot, current_bullet): (usize, &CustomAmmo)| {
                if let Some(weapon_one_ammo_slot) = &weapon_one_ammo_slot {
                    if *weapon_one_ammo_slot != slot {
                        return html!{};
                    }
                } else {
                    return html!{};
                }

                let on_bullet_select = {
                    let current_bullet = current_bullet.clone();
                    let loadout = loadout.clone();
                    let budget = budget.clone();
                    let loadout_handle = loadout_handle.clone();
                    let budget_handle = budget_handle.clone();
                    let weapon_one_ammo_slot_handle = weapon_one_ammo_slot_handle.clone();
                    let weapon_two_ammo_slot_handle = weapon_two_ammo_slot_handle.clone();

                    move |(bullet, pos): (CustomAmmo, usize)| {
                        let mut loadout = loadout.clone();
                        let mut budget = budget.clone();
                        let current_bullet = current_bullet.clone();

                        if current_bullet.2 > 0 {
                            let _ = budget::process_transaction(&mut budget, Transaction::Bullet(true, current_bullet.2, match current_bullet.1 {
                                Some(variant) => variant.to_string(),
                                None => current_bullet.0.to_string(),
                            })).ok();
                        }

                        if let Some(variant) = &bullet.1 {
                            let tx_res = budget::process_transaction(&mut budget, Transaction::Bullet(false, bullet.2, variant.to_string()));
                            loadout.weapon_one.locked = true;

                            if let Some(weapon_one) = &mut loadout.weapon_one.item {
                                weapon_one.ammo_equipped[pos] = bullet;
                            }

                            if let Ok(_tx_res) = tx_res {
                                // Process bullet.
                            }
                        } else if let Some(weapon_one) = &mut loadout.weapon_one.item {
                            weapon_one.ammo_equipped[pos] = bullet;
                        }

                        weapon_one_ammo_slot_handle.set(None);
                        weapon_two_ammo_slot_handle.set(None);
                        loadout_handle.set(loadout);
                        budget_handle.set(budget);
                    }
                };

                html! {
                    <BulletSelectList weapon={weapon.clone()} bullet_slot={slot} on_bullet_selected={on_bullet_select} />
                }
            }).collect::<Html>()
        })
    };

    let weapon_two_bullet_select_html = {
        let loadout = loadout.clone();
        let budget = budget.clone();
        let weapon_one_ammo_slot_handle = weapon_one_ammo_slot_handle.clone();
        let weapon_two_ammo_slot_handle = weapon_two_ammo_slot_handle.clone();

        loadout.weapon_two.item.as_ref().map_or_else(|| html! {}, |weapon| {
            let weapon = weapon.clone();

            weapon.ammo_equipped.iter().enumerate().map(|(slot, current_bullet): (usize, &CustomAmmo)| {
                if let Some(weapon_two_ammo_slot) = &weapon_two_ammo_slot {
                    if *weapon_two_ammo_slot != slot {
                        return html!{};
                    }
                } else {
                    return html!{};
                }

                let on_bullet_select = {
                    let current_bullet = current_bullet.clone();
                    let additional_ammo = weapon.additional_ammo_slots.unwrap_or(false);
                    let loadout = loadout.clone();
                    let budget = budget.clone();
                    let loadout_handle = loadout_handle.clone();
                    let budget_handle = budget_handle.clone();
                    let weapon_one_ammo_slot_handle = weapon_one_ammo_slot_handle.clone();
                    let weapon_two_ammo_slot_handle = weapon_two_ammo_slot_handle.clone();

                    move |(bullet, pos): (CustomAmmo, usize)| {
                        let mut loadout = loadout.clone();
                        let mut budget = budget.clone();
                        let current_bullet = current_bullet.clone();

                        if current_bullet.2 > 0 {
                            let _ = budget::process_transaction(&mut budget, Transaction::Bullet(true, if additional_ammo {
                                // As of 1.10 ammo cost is calculated this way.
                                current_bullet.2 / 2
                            } else { current_bullet.2 }, match current_bullet.1 {
                                Some(variant) => variant.to_string(),
                                None => current_bullet.0.to_string(),
                            })).ok();
                        }

                        if let Some(variant) = &bullet.1 {
                            let tx_res = budget::process_transaction(&mut budget, Transaction::Bullet(false, if additional_ammo {
                                // As of 1.10 ammo cost is calculated this way.
                                bullet.2 / 2
                            } else { bullet.2 }, variant.to_string()));
                            loadout.weapon_two.locked = true;

                            if let Some(weapon_two) = &mut loadout.weapon_two.item {
                                weapon_two.ammo_equipped[pos] = bullet;
                            }

                            if let Ok(_tx_res) = tx_res {
                                // Process bullet.
                            }
                        } else if let Some(weapon_two) = &mut loadout.weapon_two.item {
                            weapon_two.ammo_equipped[pos] = bullet;
                        }

                        weapon_one_ammo_slot_handle.set(None);
                        weapon_two_ammo_slot_handle.set(None);
                        loadout_handle.set(loadout);
                        budget_handle.set(budget);
                    }
                };

                html! {
                    <BulletSelectList weapon={weapon.clone()} bullet_slot={slot} on_bullet_selected={on_bullet_select} />
                }
            }).collect::<Html>()
        })
    };

    let advanced_options_toggled_handle = use_state(|| false);
    let advanced_options_toggled = *advanced_options_toggled_handle;
    let on_advanced_options_toggled = {
        let advanced_options_toggled_handle = advanced_options_toggled_handle.clone();

        move |_: MouseEvent| {
            advanced_options_toggled_handle.set(!advanced_options_toggled);
        }
    };

    let on_advanced_options_close = move |new_config: Config| {
        advanced_options_toggled_handle.set(!advanced_options_toggled);
        config_handle.set(new_config);
    };

    let on_tool_selected = {
        let loadout_handle = loadout_handle.clone();
        let loadout = loadout.clone();
        let budget_handle = budget_handle.clone();
        let budget = budget.clone();
        let tools_dropdown_handle = tools_dropdown_handle.clone();

        move |(item, id): (Option<GenericItem>, usize)| {
            let mut loadout = loadout.clone();
            let mut budget = budget.clone();
            loadout::refund_item(&mut budget, &loadout.tools[id]);
            loadout.tools[id].item = item;
            loadout::purchase_item(&mut budget, &loadout.tools[id]);
            loadout.tools[id].locked = true;
            tools_dropdown_handle.set([false; 4]);
            loadout_handle.set(loadout);
            budget_handle.set(budget);
        }
    };

    let on_tool_clicked = {
        let weapon_one_dropdown_handle = weapon_one_dropdown_handle.clone();
        let weapon_two_dropdown_handle = weapon_two_dropdown_handle.clone();
        let tools_dropdown_handle = tools_dropdown_handle.clone();
        let cons_dropdown_handle = cons_dropdown_handle.clone();
        let weapon_one_ammo_slot_handle = weapon_one_ammo_slot_handle.clone();
        let weapon_two_ammo_slot_handle = weapon_two_ammo_slot_handle.clone();

        move |id: usize| {
            let mut tools_dropdown_new = [false; 4];
            tools_dropdown_new[id] = !tools_dropdown[id];

            weapon_one_dropdown_handle.set(false);
            weapon_two_dropdown_handle.set(false);
            tools_dropdown_handle.set(tools_dropdown_new);
            cons_dropdown_handle.set([false; 4]);
            weapon_one_ammo_slot_handle.set(None);
            weapon_two_ammo_slot_handle.set(None);
        }
    };

    let on_tool_toggle_lock = {
        let loadout_handle = loadout_handle.clone();
        let loadout = loadout.clone();

        let weapon_one_dropdown_handle = weapon_one_dropdown_handle.clone();
        let weapon_two_dropdown_handle = weapon_two_dropdown_handle.clone();
        let tools_dropdown_handle = tools_dropdown_handle.clone();
        let cons_dropdown_handle = cons_dropdown_handle.clone();
        let weapon_one_ammo_slot_handle = weapon_one_ammo_slot_handle.clone();
        let weapon_two_ammo_slot_handle = weapon_two_ammo_slot_handle.clone();

        move |id: usize| {
            let mut loadout = loadout.clone();
            loadout.tools[id].locked = !loadout.tools[id].locked;
            loadout_handle.set(loadout);

            weapon_one_dropdown_handle.set(false);
            weapon_two_dropdown_handle.set(false);
            tools_dropdown_handle.set([false; 4]);
            cons_dropdown_handle.set([false; 4]);
            weapon_one_ammo_slot_handle.set(None);
            weapon_two_ammo_slot_handle.set(None);
        }
    };

    let on_tool_delete = {
        let loadout_handle = loadout_handle.clone();
        let loadout = loadout.clone();
        let budget_handle = budget_handle.clone();
        let budget = budget.clone();

        move |id: usize| {
            let mut loadout = loadout.clone();
            let mut budget = budget.clone();
            loadout::refund_item(&mut budget, &loadout.tools[id]);
            loadout.tools[id].locked = false;
            loadout.tools[id].item = None;
            loadout_handle.set(loadout);
            budget_handle.set(budget);
        }
    };

    let on_tool_randomize = {
        let loadout_handle = loadout_handle.clone();
        let loadout = loadout.clone();
        let budget_handle = budget_handle.clone();
        let budget = budget.clone();
        let config = config.clone();

        let weapon_one_dropdown_handle = weapon_one_dropdown_handle.clone();
        let weapon_two_dropdown_handle = weapon_two_dropdown_handle.clone();
        let tools_dropdown_handle = tools_dropdown_handle.clone();
        let cons_dropdown_handle = cons_dropdown_handle.clone();
        let weapon_one_ammo_slot_handle = weapon_one_ammo_slot_handle.clone();
        let weapon_two_ammo_slot_handle = weapon_two_ammo_slot_handle.clone();

        move |id: usize| {
            let mut loadout = loadout.clone();
            let mut budget = budget.clone();
            loadout.tools[id].locked = false;
            loadout::random_tool(
                &mut loadout,
                &mut budget,
                &config,
                id.try_into().unwrap_or(0),
            );
            loadout_handle.set(loadout);
            budget_handle.set(budget);

            weapon_one_dropdown_handle.set(false);
            weapon_two_dropdown_handle.set(false);
            tools_dropdown_handle.set([false; 4]);
            cons_dropdown_handle.set([false; 4]);
            weapon_one_ammo_slot_handle.set(None);
            weapon_two_ammo_slot_handle.set(None);
        }
    };

    let on_consumable_selected = {
        let loadout_handle = loadout_handle.clone();
        let loadout = loadout.clone();
        let budget_handle = budget_handle.clone();
        let budget = budget.clone();
        let cons_dropdown_handle = cons_dropdown_handle.clone();

        move |(item, id): (Option<GenericItem>, usize)| {
            let mut loadout = loadout.clone();
            let mut budget = budget.clone();
            loadout::refund_item(&mut budget, &loadout.consumables[id]);
            loadout.consumables[id].item = item;
            loadout::purchase_item(&mut budget, &loadout.consumables[id]);
            loadout.consumables[id].locked = true;
            cons_dropdown_handle.set([false; 4]);
            loadout_handle.set(loadout);
            budget_handle.set(budget);
        }
    };

    let on_consumable_clicked = {
        let weapon_one_dropdown_handle = weapon_one_dropdown_handle.clone();
        let weapon_two_dropdown_handle = weapon_two_dropdown_handle.clone();
        let tools_dropdown_handle = tools_dropdown_handle.clone();
        let cons_dropdown_handle = cons_dropdown_handle.clone();
        let weapon_one_ammo_slot_handle = weapon_one_ammo_slot_handle.clone();
        let weapon_two_ammo_slot_handle = weapon_two_ammo_slot_handle.clone();

        move |id: usize| {
            let mut cons_dropdown_new = [false; 4];
            cons_dropdown_new[id] = !cons_dropdown[id];

            weapon_one_dropdown_handle.set(false);
            weapon_two_dropdown_handle.set(false);
            cons_dropdown_handle.set(cons_dropdown_new);
            tools_dropdown_handle.set([false; 4]);
            weapon_one_ammo_slot_handle.set(None);
            weapon_two_ammo_slot_handle.set(None);
        }
    };

    let on_consumable_toggle_lock = {
        let loadout_handle = loadout_handle.clone();
        let loadout = loadout.clone();

        let weapon_one_dropdown_handle = weapon_one_dropdown_handle.clone();
        let weapon_two_dropdown_handle = weapon_two_dropdown_handle.clone();
        let tools_dropdown_handle = tools_dropdown_handle.clone();
        let cons_dropdown_handle = cons_dropdown_handle.clone();
        let weapon_one_ammo_slot_handle = weapon_one_ammo_slot_handle.clone();
        let weapon_two_ammo_slot_handle = weapon_two_ammo_slot_handle.clone();

        move |id: usize| {
            let mut loadout = loadout.clone();
            loadout.consumables[id].locked = !loadout.consumables[id].locked;
            loadout_handle.set(loadout);

            weapon_one_dropdown_handle.set(false);
            weapon_two_dropdown_handle.set(false);
            tools_dropdown_handle.set([false; 4]);
            cons_dropdown_handle.set([false; 4]);
            weapon_one_ammo_slot_handle.set(None);
            weapon_two_ammo_slot_handle.set(None);
        }
    };

    let on_consumable_delete = {
        let loadout_handle = loadout_handle.clone();
        let loadout = loadout.clone();
        let budget_handle = budget_handle.clone();
        let budget = budget.clone();

        let weapon_one_dropdown_handle = weapon_one_dropdown_handle.clone();
        let weapon_two_dropdown_handle = weapon_two_dropdown_handle.clone();
        let tools_dropdown_handle = tools_dropdown_handle.clone();
        let cons_dropdown_handle = cons_dropdown_handle.clone();
        let weapon_one_ammo_slot_handle = weapon_one_ammo_slot_handle.clone();
        let weapon_two_ammo_slot_handle = weapon_two_ammo_slot_handle.clone();

        move |id: usize| {
            let mut loadout = loadout.clone();
            let mut budget = budget.clone();
            loadout::refund_item(&mut budget, &loadout.consumables[id]);
            loadout.consumables[id].locked = false;
            loadout.consumables[id].item = None;
            loadout_handle.set(loadout);
            budget_handle.set(budget);

            weapon_one_dropdown_handle.set(false);
            weapon_two_dropdown_handle.set(false);
            tools_dropdown_handle.set([false; 4]);
            cons_dropdown_handle.set([false; 4]);
            weapon_one_ammo_slot_handle.set(None);
            weapon_two_ammo_slot_handle.set(None);
        }
    };

    let on_consumable_randomize = {
        let loadout = loadout.clone();
        let budget = budget.clone();

        move |id: usize| {
            let mut loadout = loadout.clone();
            let mut budget = budget.clone();
            loadout.consumables[id].locked = false;
            loadout::random_consumable(&mut loadout, &mut budget, id.try_into().unwrap_or(0));
            loadout_handle.set(loadout);
            budget_handle.set(budget);

            weapon_one_dropdown_handle.set(false);
            weapon_two_dropdown_handle.set(false);
            tools_dropdown_handle.set([false; 4]);
            cons_dropdown_handle.set([false; 4]);
            weapon_one_ammo_slot_handle.set(None);
            weapon_two_ammo_slot_handle.set(None);
        }
    };

    let budget_hover_handle = use_state(|| false);
    let budget_hover = *budget_hover_handle;

    let on_budget_hover = move |_: MouseEvent| {
        budget_hover_handle.set(!budget_hover);
    };

    let loadout_validity = {
        let mut loadout = loadout.clone();

        loadout::check_loadout_validity(
            &mut loadout,
            config.option_exists(ToggleOption::Quartermaster),
        )
    };

    let mut weapon_valid = [None, None];
    let mut tools_valid = [None, None, None, None];

    for validity in &loadout_validity {
        match validity {
            LoadoutInvalid::WeaponSlot(slot) => weapon_valid[*slot as usize] = Some("invalid"),
            LoadoutInvalid::ToolSlot(slot) => tools_valid[*slot as usize] = Some("invalid"),
        }
    }

    html! {
        <>
        <div class={classes!("container", "my-4")}>
            <div class={classes!("columns", "is-centered", "has-text-centered")}>
                <div class={classes!("column")}>
                    <label class="checkbox">
                        <input
                            type="checkbox"
                            checked={config.option_exists(ToggleOption::DualWield)}
                            onchange={on_dual_wield_change}
                        />
                        {"Dual Wield"}
                    </label>
                </div>

                <div class={classes!("column")}>
                    <label class="checkbox">
                        <input
                            type="checkbox"
                            checked={config.option_exists(ToggleOption::DuplicateWeapons)}
                            onchange={on_duplicate_weapons_change}
                        />
                        {"Duplicate Weapons"}
                    </label>
                </div>

                <div class={classes!("column")}>
                    <label class="checkbox">
                        <input
                            type="checkbox"
                            checked={config.option_exists(ToggleOption::CustomAmmo)}
                            onchange={on_custom_ammo_change}
                        />
                        {"Custom Ammo"}
                    </label>
                </div>

                <div class={classes!("column")}>
                    <label class="checkbox">
                        <input
                            type="checkbox"
                            checked={config.option_exists(ToggleOption::Quartermaster)}
                            onchange={on_quartermaster_change}
                        />
                        {"Quartermaster"}
                    </label>
                </div>
            </div>

            <div class={classes!("columns", "is-centered", "has-text-centered")}>
                <div class={classes!("column", "is-flex-grow-0")}>
                    <button
                        class="button"
                        onclick={on_advanced_options_toggled}
                    >
                        {"Advanced Options"}
                    </button>
                </div>

                <div class={classes!("column", "is-flex-grow-0")}>
                    <button
                        class="button"
                        onclick={on_generate_loadout_clicked}
                    >
                        {"Generate Loadout"}
                    </button>
                </div>
            </div>

            <div class={classes!("loadout")}>
                <div class={classes!("loadout-container", weapon_valid[0])}>
                    <WeaponSlot
                        weapon={loadout.weapon_one.item.clone()}
                        locked={loadout.weapon_one.locked}
                        dual_wield={loadout.weapon_one.item.as_ref().map_or(false, |weapon_one| weapon_one.dual_wield)}
                        ammo_types={loadout.weapon_one.item.as_ref().map_or_else(Vec::new, |weapon_one| weapon_one.ammo_equipped.clone())}
                        on_weapon_slot_clicked={on_weapon_one_clicked}
                        on_weapon_toggle_lock={on_weapon_one_toggle_lock}
                        on_weapon_delete={on_weapon_one_delete}
                        on_weapon_randomize={on_weapon_one_randomize}
                        on_ammo_slot_clicked={on_weapon_one_ammo_slot_toggled}
                    />

                    if weapon_one_dropdown {
                        <ItemSelectList
                            id={0}
                            items={CORE_SEARCH_UTIL.weapons.clone()}
                            item_variant={ItemVariant::Weapon}
                            on_item_selected={on_weapon_one_selected}
                        />
                    }

                    {weapon_one_bullet_select_html}
                </div>

                <div class={classes!("loadout-container", weapon_valid[1])}>
                    <WeaponSlot
                        weapon={loadout.weapon_two.item.clone()}
                        locked={loadout.weapon_two.locked}
                        dual_wield={loadout.weapon_two.item.as_ref().map_or(false, |weapon_two| weapon_two.dual_wield)}
                        ammo_types={loadout.weapon_two.item.as_ref().map_or_else(Vec::new, |weapon_two| weapon_two.ammo_equipped.clone())}
                        on_weapon_slot_clicked={on_weapon_two_clicked}
                        on_weapon_toggle_lock={on_weapon_two_toggle_lock}
                        on_weapon_delete={on_weapon_two_delete}
                        on_weapon_randomize={on_weapon_two_randomize}
                        on_ammo_slot_clicked={on_weapon_two_ammo_slot_toggled}
                    />

                    if weapon_two_dropdown {
                        <ItemSelectList
                            id={0}
                            items={CORE_SEARCH_UTIL.weapons.clone()}
                            item_variant={ItemVariant::Weapon}
                            on_item_selected={on_weapon_two_selected}
                        />
                    }

                    {weapon_two_bullet_select_html}
                </div>

                <h4 class={classes!("is-size-4", "has-text-centered")}>{"Tools"}</h4>

                <div class={classes!("columns", "is-centered")}>
                    {loadout.tools.iter().enumerate().map(|(id, tool)| html! {
                        <div class={classes!("loadout-container", "tool", tools_valid[id])}>
                            <ToolSlot
                                tool={tool.item.clone()}
                                locked={tool.locked}
                                {id}
                                on_tool_slot_clicked={on_tool_clicked.clone()}
                                on_tool_toggle_lock={on_tool_toggle_lock.clone()}
                                on_tool_delete={on_tool_delete.clone()}
                                on_tool_randomize={on_tool_randomize.clone()}
                            />

                            if tools_dropdown[id] {
                                <ItemSelectList
                                    {id}
                                    items={CORE_SEARCH_UTIL.tools.clone()}
                                    item_variant={ItemVariant::Tool}
                                    on_item_selected={on_tool_selected.clone()}
                                />
                            }
                        </div>
                    }).collect::<Html>()}
                </div>

                <h4 class={classes!("is-size-4", "has-text-centered")}>{"Consumables"}</h4>

                <div class={classes!("columns", "is-centered")}>
                    {loadout.consumables.iter().enumerate().map(|(id, consumable)| html! {
                        <div class={classes!("loadout-container", "consumable")}>
                            <ConsumableSlot
                                consumable={consumable.item.clone()}
                                locked={consumable.locked}
                                {id}
                                on_consumable_slot_clicked={on_consumable_clicked.clone()}
                                on_consumable_toggle_lock={on_consumable_toggle_lock.clone()}
                                on_consumable_delete={on_consumable_delete.clone()}
                                on_consumable_randomize={on_consumable_randomize.clone()}
                            />

                            if cons_dropdown[id] {
                                <ItemSelectList
                                    {id}
                                    items={CORE_SEARCH_UTIL.consumables.clone()}
                                    item_variant={ItemVariant::Consumable}
                                    on_item_selected={on_consumable_selected.clone()}
                                />
                            }
                        </div>
                    }).collect::<Html>()}
                </div>

                <div
                    class={classes!("transactions-container")}
                    onmouseenter={on_budget_hover.clone()}
                    onmouseleave={on_budget_hover}
                >
                    <h2 class={classes!("is-size-2", "has-text-centered")}>{&*format!("Hunt Dollars: {}", budget.total_cost)}</h2>

                    if budget_hover {
                        <BudgetDisplay budget={budget} />
                    }
                </div>
            </div>
        </div>

        if advanced_options_toggled {
            <AdvancedOptions
                is_active={advanced_options_toggled}
                config={config}
                on_options_close={on_advanced_options_close}
            />
        }
        </>
    }
}
