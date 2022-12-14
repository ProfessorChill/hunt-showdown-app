use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::cmp::Ordering;

use crate::content::{
    generic_item::{CustomAmmo, GenericItemLockable},
    BulletSize, GenericItem, Slot, ToolSlotPreference, CORE_SEARCH_UTIL,
};
use crate::randomizer::budget::{Transaction, TransactionResult};
use crate::randomizer::{budget, config::ToggleOption, Budget, Config, LoadoutInvalid};

const ERR_INSF_FND_LOCK: &str =
    "Insufficient Funds, try unlocking this item or increase your budget.";
const ERR_SLOT_GT_UNSIGNED: &str = "Somehow got slot greater than u8.";
const MAX_DUPE_CHECK_AMOUNT: usize = 10;
pub const INVALID_DUALWIELD_NAMES: &[&str] =
    &["Cavalry Saber", "Hand Crossbow", "Combat Axe", "Machete"];

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub enum LoadoutError {
    Weapon { error: String, slot: u8 },
    Tool { error: String, slot: u8 },
    Consumable { error: String, slot: u8 },
}

#[derive(Debug, Clone)]
pub struct Loadout {
    pub errors: Vec<LoadoutError>,
    pub weapon_one: GenericItemLockable,
    pub weapon_two: GenericItemLockable,
    pub tools: [GenericItemLockable; 4],
    pub consumables: [GenericItemLockable; 4],
}

impl Default for Loadout {
    fn default() -> Self {
        Self {
            errors: vec![],
            weapon_one: GenericItemLockable {
                item: None,
                locked: false,
            },
            weapon_two: GenericItemLockable {
                item: None,
                locked: false,
            },
            tools: [
                GenericItemLockable::default(),
                GenericItemLockable::default(),
                GenericItemLockable::default(),
                GenericItemLockable::default(),
            ],
            consumables: [
                GenericItemLockable::default(),
                GenericItemLockable::default(),
                GenericItemLockable::default(),
                GenericItemLockable::default(),
            ],
        }
    }
}

fn transaction_from_custom_ammo(
    budget: &mut Budget,
    ammo: &CustomAmmo,
) -> Result<(), TransactionResult> {
    budget::process_transaction(
        budget,
        Transaction::Bullet(
            false,
            ammo.2,
            ammo.1
                .as_ref()
                .map_or_else(|| ammo.0.to_string(), ToString::to_string),
        ),
    )
}

fn transaction_from_weapon(
    budget: &mut Budget,
    weapon: &GenericItem,
    refund: bool,
) -> Result<(), TransactionResult> {
    budget::process_transaction(
        budget,
        Transaction::Weapon(refund, weapon.get_cost(), weapon.to_full_name()),
    )
}

pub fn get_valid_slots(quartermaster: bool, slot: &Slot) -> Vec<Slot> {
    if quartermaster {
        match slot {
            Slot::Small | Slot::Medium => vec![Slot::Small, Slot::Medium, Slot::Large],
            Slot::Large => vec![Slot::Small, Slot::Medium],
        }
    } else {
        match slot {
            Slot::Small => vec![Slot::Small, Slot::Medium, Slot::Large],
            Slot::Medium => vec![Slot::Small, Slot::Medium],
            Slot::Large => vec![Slot::Small],
        }
    }
}

pub fn item_lte_cost<'a>(
    items: &'a [&GenericItem],
    cost: u16,
    rng: &mut ThreadRng,
) -> Option<GenericItem> {
    let items = items
        .iter()
        .filter(|item| item.get_cost() <= cost)
        .copied()
        .collect::<Vec<&GenericItem>>();

    if items.is_empty() {
        None
    } else {
        Some(items[rng.gen_range(0..items.len())].clone())
    }
}

pub fn refund_item(budget: &mut Budget, item: &GenericItemLockable) {
    if let Some(item) = &item.item {
        for ammo_type in &item.ammo_equipped {
            if ammo_type.2 > 0 {
                let _tx = budget::process_transaction(
                    budget,
                    Transaction::Bullet(
                        true,
                        ammo_type.2,
                        ammo_type
                            .1
                            .as_ref()
                            .map_or_else(|| ammo_type.0.to_string(), ToString::to_string),
                    ),
                )
                .ok();
            }
        }

        let _tx = budget::process_transaction(
            budget,
            Transaction::Weapon(true, item.get_cost(), item.to_full_name()),
        )
        .ok();
    }
}

pub fn purchase_item(budget: &mut Budget, item: &GenericItemLockable) {
    if let Some(item) = &item.item {
        for ammo_type in &item.ammo_equipped {
            if ammo_type.2 > 0 {
                let tx_res = budget::process_transaction(
                    budget,
                    Transaction::Bullet(
                        false,
                        ammo_type.2,
                        ammo_type
                            .1
                            .as_ref()
                            .map_or_else(|| ammo_type.0.to_string(), ToString::to_string),
                    ),
                );

                if let Err(_e) = tx_res {
                    // Come up with way to handle error.
                }
            }
        }

        let tx_res = budget::process_transaction(
            budget,
            Transaction::Weapon(false, item.get_cost(), item.to_full_name()),
        );

        if let Err(_e) = tx_res {
            // Come up with way to handle error.
        }
    }
}

pub fn set_default_ammo(item: &mut GenericItemLockable) -> bool {
    if let Some(item) = &mut item.item {
        if !item.ammo_equipped.is_empty() {
            return false;
        }

        if let Some(bullet_size) = item.get_bullet_size() {
            if item.additional_ammo_slots.unwrap_or(false) {
                item.ammo_equipped = vec![(bullet_size.clone(), None, 0), (bullet_size, None, 0)];
            } else {
                item.ammo_equipped = vec![(bullet_size, None, 0)];
            }
        }
    }

    true
}

pub fn initial_weapon(
    loadout: &mut Loadout,
    budget: &mut Budget,
    rng: &mut ThreadRng,
    quartermaster: bool,
    weapon: &mut GenericItemLockable,
    check: &GenericItemLockable,
) {
    // Rusts powerful matching option makes advanced generation nice and easy!
    match (&mut weapon.item, weapon.locked) {
        // If weapon is locked and exists, process the weapon transaction.
        (Some(weapon), true) => {
            let tx_res = budget::process_transaction(
                budget,
                Transaction::Weapon(false, weapon.get_cost(), weapon.to_full_name()),
            );

            if let Err(_e) = tx_res {
                loadout.errors.push(LoadoutError::Weapon {
                    error: ERR_INSF_FND_LOCK.to_string(),
                    slot: 0,
                });
            }
        }
        // The weapon one exists but isn't locked, or weapon one doesn't exist and isn't
        // locked.
        (Some(_) | None, false) => {
            // If weapon two exists and is locked we want a weapon based on that, otherwise
            // generate a random weapon.
            if let (Some(check_weapon), _) = (&check.item, check.locked) {
                let slot = check_weapon.get_slot();
                let valid_slots = get_valid_slots(quartermaster, &slot);
                let weapons = CORE_SEARCH_UTIL.get_weapons_by_sizes(&valid_slots);
                let mut new_weapon = item_lte_cost(&weapons, budget.weapons_budget, rng);

                if let Some(new_check_weapon) = &new_weapon {
                    let tx_res = budget::process_transaction(
                        budget,
                        Transaction::Weapon(
                            false,
                            new_check_weapon.get_cost(),
                            new_check_weapon.to_full_name(),
                        ),
                    );

                    if let Err(_e) = tx_res {
                        // We cannot afford the new weapon.
                        new_weapon = None;
                    }
                }

                weapon.item = new_weapon;
            } else {
                let weapons = CORE_SEARCH_UTIL
                    .weapons
                    .iter()
                    .collect::<Vec<&GenericItem>>();

                let new_weapon = item_lte_cost(&weapons, budget.weapons_budget, rng);

                weapon.item = if let Some(new_check_weapon) = &new_weapon {
                    let tx_res = budget::process_transaction(
                        budget,
                        Transaction::Weapon(
                            false,
                            new_check_weapon.get_cost(),
                            new_check_weapon.to_full_name(),
                        ),
                    );

                    if let Err(_e) = tx_res {
                        // We cannot afford the new weapon.
                        None
                    } else {
                        new_weapon
                    }
                } else {
                    None
                };
            }
        }
        // The weapon doesn't exist and is locked, we don't want to generate an item.
        (None, true) => {}
    }
}

pub fn reset_tools(loadout: &mut Loadout, budget: &mut Budget) {
    for (slot, tool) in loadout.tools.iter_mut().enumerate() {
        if !tool.locked {
            tool.item = None;
        } else if let Some(tool) = &tool.item {
            let tx_res = budget::process_transaction(
                budget,
                Transaction::Tool(false, tool.get_cost(), tool.to_full_name()),
            );

            // If we can afford the transaction, purchase it, otherwise don't and report we
            // can't afford the item.
            if let Err(_e) = tx_res {
                loadout.errors.push(LoadoutError::Tool {
                    error: ERR_INSF_FND_LOCK.to_string(),
                    slot: slot.try_into().expect(ERR_SLOT_GT_UNSIGNED),
                });
            }
        }
    }
}

pub fn reset_consumables(loadout: &mut Loadout, budget: &mut Budget) {
    for (slot, consumable) in loadout.consumables.iter_mut().enumerate() {
        if !consumable.locked {
            consumable.item = None;
        } else if let Some(consumable) = &consumable.item {
            let tx_res = budget::process_transaction(
                budget,
                Transaction::Consumable(false, consumable.get_cost(), consumable.to_full_name()),
            );

            if let Err(_e) = tx_res {
                loadout.errors.push(LoadoutError::Consumable {
                    error: ERR_INSF_FND_LOCK.to_string(),
                    slot: slot.try_into().expect(ERR_SLOT_GT_UNSIGNED),
                });
            }
        }
    }
}

pub fn always_quartermaster(loadout: &mut Loadout, budget: &mut Budget, rng: &mut ThreadRng) {
    // Always replace the loadout with a valid quartermaster loadout.

    if loadout.weapon_one.locked || loadout.weapon_two.locked {
        return;
    }

    if let Some(weapon_one) = &loadout.weapon_one.item {
        let _tx = budget::process_transaction(
            budget,
            Transaction::Weapon(true, weapon_one.get_cost(), weapon_one.to_full_name()),
        )
        .ok();
    }

    let weapons = CORE_SEARCH_UTIL.get_weapons_by_sizes(&[Slot::Large]);
    let new_weapon = item_lte_cost(&weapons, budget.weapons_budget, rng);

    loadout.weapon_one.item = if let Some(new_check_weapon) = &new_weapon {
        let tx_res = budget::process_transaction(
            budget,
            Transaction::Weapon(
                false,
                new_check_weapon.get_cost(),
                new_check_weapon.to_full_name(),
            ),
        );

        if let Err(_e) = tx_res {
            None
        } else {
            new_weapon
        }
    } else {
        None
    };

    if let Some(weapon_two) = &loadout.weapon_two.item {
        let _tx = budget::process_transaction(
            budget,
            Transaction::Weapon(true, weapon_two.get_cost(), weapon_two.to_full_name()),
        )
        .ok();
    }

    let weapons = CORE_SEARCH_UTIL.get_weapons_by_sizes(&[Slot::Medium]);
    let new_weapon = item_lte_cost(&weapons, budget.weapons_budget, rng);

    loadout.weapon_two.item = if let Some(new_check_weapon) = &new_weapon {
        let tx_res = budget::process_transaction(
            budget,
            Transaction::Weapon(
                false,
                new_check_weapon.get_cost(),
                new_check_weapon.to_full_name(),
            ),
        );

        if let Err(_e) = tx_res {
            None
        } else {
            new_weapon
        }
    } else {
        None
    };
}

pub fn always_dual_wield(
    budget: &mut Budget,
    rng: &mut ThreadRng,
    weapon: &mut GenericItemLockable,
) {
    // Temporary workaround to making cost work.
    let mut weapons = CORE_SEARCH_UTIL
        .get_dual_wield_weapons()
        .iter()
        .copied()
        .cloned()
        .collect::<Vec<GenericItem>>();
    let search_weapons = weapons
        .iter_mut()
        .map(|weapon| {
            weapon.dual_wield = true;
            &*weapon
        })
        .collect::<Vec<&GenericItem>>();

    if let (Some(weapon_check), false) = (&mut weapon.item, weapon.locked) {
        // Weapon one is already dual wield, we can ignore this.
        if weapon_check.dual_wield {
            return;
        }

        let _tx = budget::process_transaction(
            budget,
            Transaction::Weapon(true, weapon_check.get_cost(), weapon_check.to_full_name()),
        )
        .ok();

        let mut new_weapon = item_lte_cost(&search_weapons, budget.weapons_budget, rng);

        if let Some(new_check_weapon) = &new_weapon {
            let tx_res = budget::process_transaction(
                budget,
                Transaction::Weapon(
                    false,
                    new_check_weapon.get_cost(),
                    new_check_weapon.to_full_name(),
                ),
            );

            if let Err(_e) = tx_res {
                new_weapon = None;
            }
        }

        weapon.item = new_weapon;
    }
}

pub fn dedupe_weapons(
    loadout: &mut Loadout,
    budget: &mut Budget,
    rng: &mut ThreadRng,
    quartermaster: bool,
) {
    match (&loadout.weapon_one, &loadout.weapon_two) {
        (
            GenericItemLockable {
                locked: false | true,
                item: Some(weapon_one),
            },
            GenericItemLockable {
                locked: false,
                item: Some(weapon_two),
            },
        ) => {
            if weapon_one != weapon_two {
                return;
            }

            let mut attempts = 0;
            let _tx = transaction_from_weapon(budget, weapon_two, true).ok();
            let mut new_weapon = weapon_two.clone();

            while weapon_one == &new_weapon && attempts != MAX_DUPE_CHECK_AMOUNT {
                let slot = weapon_one.get_slot();
                let valid_slots = get_valid_slots(quartermaster, &slot);
                let weapons = CORE_SEARCH_UTIL.get_weapons_by_sizes(&valid_slots);
                let rand_weapon = item_lte_cost(&weapons, budget.weapons_budget, rng);

                if let Some(new_check_weapon) = rand_weapon {
                    new_weapon = new_check_weapon.clone();
                }

                attempts += 1;
            }

            let new_weapon = Some(new_weapon);
            loadout.weapon_two.item = if let Some(new_check_weapon) = &new_weapon {
                let tx_res = transaction_from_weapon(budget, new_check_weapon, false);

                if let Err(_e) = tx_res {
                    None
                } else {
                    new_weapon
                }
            } else {
                None
            };
        }
        (
            GenericItemLockable {
                locked: false,
                item: Some(weapon_one),
            },
            GenericItemLockable {
                locked: true,
                item: Some(weapon_two),
            },
        ) => {
            if weapon_two != weapon_one {
                return;
            }

            let mut attempts = 0;
            let _tx = transaction_from_weapon(budget, weapon_one, true).ok();
            let mut new_weapon = weapon_one.clone();

            while weapon_two == &new_weapon && attempts != MAX_DUPE_CHECK_AMOUNT {
                let slot = weapon_two.get_slot();
                let valid_slots = get_valid_slots(quartermaster, &slot);
                let weapons = CORE_SEARCH_UTIL.get_weapons_by_sizes(&valid_slots);
                let rand_weapon = item_lte_cost(&weapons, budget.weapons_budget, rng);

                if let Some(new_check_weapon) = rand_weapon {
                    new_weapon = new_check_weapon.clone();
                }

                attempts += 1;
            }

            let new_weapon = Some(new_weapon);
            loadout.weapon_one.item = if let Some(new_check_weapon) = &new_weapon {
                let tx_res = transaction_from_weapon(budget, new_check_weapon, false);

                if let Err(_e) = tx_res {
                    None
                } else {
                    new_weapon
                }
            } else {
                None
            };
        }
        // We can't replace items if we can't check duplicates.
        _ => {}
    }
}

pub fn always_duplicate_weapons(loadout: &mut Loadout, budget: &mut Budget) {
    if let (
        GenericItemLockable {
            locked: _,
            item: Some(weapon_one),
        },
        GenericItemLockable {
            locked: false,
            item: weapon_two,
        },
    ) = (&loadout.weapon_one, &loadout.weapon_two)
    {
        match weapon_one.get_slot() {
            // We can only clone small or medium slot weapons
            Slot::Small | Slot::Medium => {
                if let Some(weapon_two) = weapon_two {
                    let _tx = budget::process_transaction(
                        budget,
                        Transaction::Weapon(true, weapon_two.get_cost(), weapon_two.to_full_name()),
                    )
                    .ok();
                }

                let tx_res = budget::process_transaction(
                    budget,
                    Transaction::Weapon(false, weapon_one.get_cost(), weapon_one.to_full_name()),
                );

                if tx_res.is_ok() {
                    loadout.weapon_two = loadout.weapon_one.clone();
                } else if let Some(weapon_two) = weapon_two {
                    // Reversing transaction, this should not fail.
                    let _tx = budget::process_transaction(
                        budget,
                        Transaction::Weapon(
                            false,
                            weapon_two.get_cost(),
                            weapon_two.to_full_name(),
                        ),
                    )
                    .ok();
                }

                return;
            }
            Slot::Large => {}
        }
    }

    if let (
        GenericItemLockable {
            locked: false,
            item: weapon_one,
        },
        GenericItemLockable {
            locked: _,
            item: Some(weapon_two),
        },
    ) = (&loadout.weapon_one, &loadout.weapon_two)
    {
        match weapon_two.get_slot() {
            Slot::Small | Slot::Medium => {
                if let Some(weapon_one) = weapon_one {
                    let _tx = budget::process_transaction(
                        budget,
                        Transaction::Weapon(true, weapon_one.get_cost(), weapon_one.to_full_name()),
                    )
                    .ok();
                }

                let tx_res = budget::process_transaction(
                    budget,
                    Transaction::Weapon(false, weapon_two.get_cost(), weapon_two.to_full_name()),
                );

                if tx_res.is_ok() {
                    loadout.weapon_one = loadout.weapon_two.clone();
                } else if let Some(weapon_one) = weapon_one {
                    // Reversing transaction, this should not fail.
                    let _tx = budget::process_transaction(
                        budget,
                        Transaction::Weapon(
                            false,
                            weapon_one.get_cost(),
                            weapon_one.to_full_name(),
                        ),
                    )
                    .ok();
                }
            }
            Slot::Large => {}
        }
    }
}

pub fn custom_ammo(
    budget: &mut Budget,
    rng: &mut ThreadRng,
    weapon: &mut GenericItemLockable,
    always: bool,
) {
    // If the weapon is locked we don't want to bother selecting custom ammo. We want to charge
    // for the existing ammo selection.
    if weapon.locked {
        if set_default_ammo(weapon) {
            return;
        }

        if let Some(weapon) = &mut weapon.item {
            for ammo_type in &weapon.ammo_equipped {
                if ammo_type.1.is_some() {
                    let tx_res = transaction_from_custom_ammo(budget, ammo_type);

                    if tx_res.is_err() {
                        // Handle not purchasable ammo.
                    }
                }
            }
        }

        return;
    }

    if let Some(weapon) = &mut weapon.item {
        let bullet_types = weapon.get_bullet_variants();

        weapon.ammo_equipped = vec![];

        if bullet_types.is_empty() {
            return;
        }

        if always {
            let ammo_type = bullet_types[rng.gen_range(0..bullet_types.len())].clone();
            let tx_res = transaction_from_custom_ammo(budget, &ammo_type);

            if tx_res.is_ok() {
                weapon.ammo_equipped.push(ammo_type);
            }

            if weapon.additional_ammo_slots.unwrap_or(false) {
                let ammo_type = bullet_types[rng.gen_range(0..bullet_types.len())].clone();
                let tx_res = transaction_from_custom_ammo(budget, &ammo_type);

                if tx_res.is_ok() {
                    weapon.ammo_equipped.push(ammo_type);
                }
            }
        } else {
            if rng.gen_bool(0.25) {
                let ammo_type = bullet_types[rng.gen_range(0..bullet_types.len())].clone();
                let tx_res = transaction_from_custom_ammo(budget, &ammo_type);

                if tx_res.is_ok() {
                    weapon.ammo_equipped.push(ammo_type);
                }
            } else if let Some(bullet_size) = &weapon.get_bullet_size() {
                weapon.ammo_equipped.push((bullet_size.clone(), None, 0));
            }

            if rng.gen_bool(0.25) && weapon.additional_ammo_slots.unwrap_or(false) {
                let ammo_type = bullet_types[rng.gen_range(0..bullet_types.len())].clone();
                let tx_res = transaction_from_custom_ammo(budget, &ammo_type);

                if tx_res.is_ok() {
                    weapon.ammo_equipped.push(ammo_type);
                }
            } else if weapon.additional_ammo_slots.unwrap_or(false) {
                if let Some(bullet_size) = &weapon.get_bullet_size() {
                    weapon.ammo_equipped.push((bullet_size.clone(), None, 0));
                }
            }
        }

        if let Some(bullet_size) = &weapon.get_bullet_size() {
            if weapon.ammo_equipped.is_empty() && weapon.additional_ammo_slots.unwrap_or(false) {
                weapon.ammo_equipped = vec![
                    (bullet_size.clone(), None, 0),
                    (bullet_size.clone(), None, 0),
                ];
            } else if weapon.ammo_equipped.is_empty() {
                weapon.ammo_equipped = vec![(bullet_size.clone(), None, 0)];
            } else if weapon.ammo_equipped.len() == 1
                && weapon.additional_ammo_slots.unwrap_or(false)
            {
                weapon.ammo_equipped.push((bullet_size.clone(), None, 0));
            }
        }
    }
}

pub fn random_tools(
    loadout: &mut Loadout,
    budget: &mut Budget,
    config: &Config,
    rng: &mut ThreadRng,
) {
    let check_clone = loadout.tools.clone();

    for (slot, tool) in check_clone.iter().enumerate() {
        if tool.locked {
            continue;
        }

        let random_tools = match config.tool_preferences[slot] {
            ToolSlotPreference::NoPreference => {
                CORE_SEARCH_UTIL.tools.iter().collect::<Vec<&GenericItem>>()
            }
            ToolSlotPreference::Decoys => CORE_SEARCH_UTIL.get_decoy_tools(),
            ToolSlotPreference::Tripmines => CORE_SEARCH_UTIL.get_trip_mines(),
            ToolSlotPreference::Melee => CORE_SEARCH_UTIL.get_melee_tools(),
            ToolSlotPreference::Throwable => CORE_SEARCH_UTIL.get_throwables(),
            ToolSlotPreference::Medkit => CORE_SEARCH_UTIL
                .tools
                .iter()
                .filter(|tool| tool.to_full_name().as_str() == "First Aid Kit")
                .collect::<Vec<&GenericItem>>(),
            ToolSlotPreference::Others => CORE_SEARCH_UTIL.get_other_tools(),
        };

        let random_tool = item_lte_cost(&random_tools, budget.tools_budget, rng);

        if loadout.tools.iter().all(|t| t.item != random_tool) {
            loadout.tools[slot].item = random_tool.as_ref().and_then(|check_tool| {
                let tx_res = budget::process_transaction(
                    budget,
                    Transaction::Tool(false, check_tool.cost, check_tool.to_full_name()),
                );

                if tx_res.is_ok() {
                    Some(check_tool.clone())
                } else {
                    None
                }
            });
        }
    }

    if config
        .tool_preferences
        .iter()
        .all(|pref| *pref == ToolSlotPreference::NoPreference)
    {
        loadout.tools.sort_by(|a, b| {
            if a.locked || b.locked {
                Ordering::Equal
            } else if a.item.is_none() && b.item.is_some() {
                Ordering::Greater
            } else if b.item.is_none() && a.item.is_some() {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
    }
}

pub fn random_tool(loadout: &mut Loadout, budget: &mut Budget, config: &Config, slot: u8) {
    let mut rng = thread_rng();
    let mut previous_budget = budget.clone();
    let previous_tool = loadout.tools[slot as usize].clone();

    let not_tools = loadout
        .tools
        .iter()
        .enumerate()
        .filter_map(|(sr_slot, tool)| {
            if sr_slot.try_into().unwrap_or(0) == slot {
                None
            } else {
                tool.item.clone()
            }
        })
        .collect::<Vec<GenericItem>>();

    let random_tools = match config.tool_preferences[slot as usize] {
        ToolSlotPreference::NoPreference => {
            CORE_SEARCH_UTIL.tools.iter().collect::<Vec<&GenericItem>>()
        }
        ToolSlotPreference::Decoys => CORE_SEARCH_UTIL.get_decoy_tools(),
        ToolSlotPreference::Tripmines => CORE_SEARCH_UTIL.get_trip_mines(),
        ToolSlotPreference::Melee => CORE_SEARCH_UTIL.get_melee_tools(),
        ToolSlotPreference::Throwable => CORE_SEARCH_UTIL.get_throwables(),
        ToolSlotPreference::Medkit => CORE_SEARCH_UTIL
            .tools
            .iter()
            .filter(|tool| tool.to_full_name().as_str() == "First Aid Kit")
            .collect::<Vec<&GenericItem>>(),
        ToolSlotPreference::Others => CORE_SEARCH_UTIL.get_other_tools(),
    };

    let random_tools = random_tools
        .iter()
        .filter(|tool| !not_tools.contains(tool))
        .copied()
        .collect::<Vec<&GenericItem>>();

    if !random_tools.is_empty() {
        loadout.tools[slot as usize].item =
            Some(random_tools[rng.gen_range(0..random_tools.len())].clone());

        refund_item(&mut previous_budget, &previous_tool);
        purchase_item(&mut previous_budget, &loadout.tools[slot as usize]);
        *budget = previous_budget;
    }
}

pub fn random_consumable(loadout: &mut Loadout, budget: &mut Budget, slot: u8) {
    let mut rng = thread_rng();
    let mut previous_budget = budget.clone();
    let previous_consumable = loadout.consumables[slot as usize].clone();

    let random_consumables = CORE_SEARCH_UTIL
        .consumables
        .iter()
        .collect::<Vec<&GenericItem>>();

    if !random_consumables.is_empty() {
        loadout.consumables[slot as usize].item =
            Some(random_consumables[rng.gen_range(0..random_consumables.len())].clone());

        refund_item(&mut previous_budget, &previous_consumable);
        purchase_item(&mut previous_budget, &loadout.consumables[slot as usize]);
        *budget = previous_budget;
    }
}

fn random_consumables(loadout: &mut Loadout, budget: &mut Budget, rng: &mut ThreadRng) {
    for consumable in &mut loadout.consumables {
        if consumable.locked {
            continue;
        }

        let random_consumables = CORE_SEARCH_UTIL
            .consumables
            .iter()
            .collect::<Vec<&GenericItem>>();
        let random_consumable = item_lte_cost(&random_consumables, budget.consumables_budget, rng);

        consumable.item = random_consumable.as_ref().and_then(|check_consumable| {
            let tx_res = budget::process_transaction(
                budget,
                Transaction::Consumable(
                    false,
                    check_consumable.cost,
                    check_consumable.to_full_name(),
                ),
            );

            if tx_res.is_ok() {
                Some(check_consumable.clone())
            } else {
                None
            }
        });
    }

    loadout.consumables.sort_by(|a, b| {
        if a.locked || b.locked {
            Ordering::Equal
        } else if a.item.is_none() && b.item.is_some() {
            Ordering::Greater
        } else if b.item.is_none() && a.item.is_some() {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
}

pub fn random_weapon_one(loadout: &mut Loadout, budget: &mut Budget, config: &Config) {
    let mut previous_budget = budget.clone();
    let previous_weapon = loadout.weapon_one.clone();

    let mut rng = thread_rng();
    let initial_weapon_two_lock = loadout.weapon_two.locked;

    let mut weapon_one = loadout.weapon_one.clone();
    let weapon_two = loadout.weapon_two.clone();
    initial_weapon(
        loadout,
        budget,
        &mut rng,
        config.option_exists(ToggleOption::Quartermaster),
        &mut weapon_one,
        &weapon_two,
    );
    loadout.weapon_one = weapon_one.clone();

    if config.option_exists(ToggleOption::Quartermaster)
        && config.option_exists(ToggleOption::AlwaysQuartermaster)
    {
        // Lock weapon two since we ONLY want to ensure weapon one is changed.
        loadout.weapon_two.locked = true;
        always_quartermaster(loadout, budget, &mut rng);
        loadout.weapon_two.locked = initial_weapon_two_lock;
    }

    if config.option_exists(ToggleOption::AlwaysDualWield) {
        let mut weapon_one = loadout.weapon_one.clone();
        always_dual_wield(budget, &mut rng, &mut weapon_one);
        loadout.weapon_one = weapon_one.clone();
    }

    if !config.option_exists(ToggleOption::DuplicateWeapons) {
        // Again lock weapon two since we ONLY want to ensure weapon one is changed.
        loadout.weapon_two.locked = true;
        dedupe_weapons(
            loadout,
            budget,
            &mut rng,
            config.option_exists(ToggleOption::Quartermaster),
        );
        loadout.weapon_two.locked = initial_weapon_two_lock;
    }

    if config.option_exists(ToggleOption::CustomAmmo) {
        let mut weapon_one = loadout.weapon_one.clone();
        custom_ammo(
            budget,
            &mut rng,
            &mut weapon_one,
            config.option_exists(ToggleOption::AlwaysCustomAmmo),
        );
        loadout.weapon_one = weapon_one.clone();
    }

    if config.option_exists(ToggleOption::DuplicateWeapons)
        && config.option_exists(ToggleOption::AlwaysDuplicateWeapons)
    {
        // ONCE AGAIN, lock weapon two, we don't want to change it.
        loadout.weapon_two.locked = true;
        always_duplicate_weapons(loadout, budget);
        loadout.weapon_two.locked = initial_weapon_two_lock;
    }

    refund_item(&mut previous_budget, &previous_weapon);
    purchase_item(&mut previous_budget, &loadout.weapon_one);
    *budget = previous_budget;
}

pub fn random_weapon_two(loadout: &mut Loadout, budget: &mut Budget, config: &Config) {
    let mut previous_budget = budget.clone();
    let previous_weapon = loadout.weapon_two.clone();

    let mut rng = thread_rng();
    let initial_weapon_one_lock = loadout.weapon_one.locked;

    let mut weapon_two = loadout.weapon_two.clone();
    let weapon_one = loadout.weapon_one.clone();
    initial_weapon(
        loadout,
        budget,
        &mut rng,
        config.option_exists(ToggleOption::Quartermaster),
        &mut weapon_two,
        &weapon_one,
    );
    loadout.weapon_two = weapon_two.clone();

    if config.option_exists(ToggleOption::Quartermaster)
        && config.option_exists(ToggleOption::AlwaysQuartermaster)
    {
        // Lock weapon one since we ONLY want to ensure weapon two is changed.
        loadout.weapon_one.locked = true;
        always_quartermaster(loadout, budget, &mut rng);
        loadout.weapon_one.locked = initial_weapon_one_lock;
    }

    if config.option_exists(ToggleOption::AlwaysDualWield) {
        let mut weapon_two = loadout.weapon_two.clone();
        always_dual_wield(budget, &mut rng, &mut weapon_two);
        loadout.weapon_two = weapon_two.clone();
    }

    if !config.option_exists(ToggleOption::DuplicateWeapons) {
        // Again lock weapon one since we ONLY want to ensure weapon two is changed.
        loadout.weapon_one.locked = true;
        dedupe_weapons(
            loadout,
            budget,
            &mut rng,
            config.option_exists(ToggleOption::Quartermaster),
        );
        loadout.weapon_one.locked = initial_weapon_one_lock;
    }

    if config.option_exists(ToggleOption::CustomAmmo) {
        let mut weapon_two = loadout.weapon_two.clone();
        custom_ammo(
            budget,
            &mut rng,
            &mut weapon_two,
            config.option_exists(ToggleOption::AlwaysCustomAmmo),
        );
        loadout.weapon_two = weapon_two.clone();
    }

    if config.option_exists(ToggleOption::DuplicateWeapons)
        && config.option_exists(ToggleOption::AlwaysDuplicateWeapons)
    {
        // ONCE AGAIN, lock weapon one, we don't want to change it.
        loadout.weapon_one.locked = true;
        always_duplicate_weapons(loadout, budget);
        loadout.weapon_one.locked = initial_weapon_one_lock;
    }

    refund_item(&mut previous_budget, &previous_weapon);
    purchase_item(&mut previous_budget, &loadout.weapon_two);
    *budget = previous_budget;
}

fn sort_weapons(loadout: &mut Loadout) {
    if !loadout.weapon_one.locked && !loadout.weapon_two.locked {
        match (&mut loadout.weapon_one.item, &mut loadout.weapon_two.item) {
            (Some(weapon_one), Some(weapon_two)) => {
                match (weapon_one.get_slot(), weapon_two.get_slot()) {
                    (Slot::Small | Slot::Medium, Slot::Large) => {
                        let weapon_one_clone = weapon_one.clone();
                        *weapon_one = weapon_two.clone();
                        *weapon_two = weapon_one_clone;
                    }
                    (Slot::Medium, Slot::Medium) | (Slot::Small, Slot::Small) => {
                        match (weapon_one.get_bullet_size(), weapon_two.get_bullet_size()) {
                            (
                                Some(BulletSize::Compact)
                                | Some(BulletSize::Medium)
                                | Some(BulletSize::Special),
                                Some(_),
                            ) => {
                                let weapon_one_clone = weapon_one.clone();
                                *weapon_one = weapon_two.clone();
                                *weapon_two = weapon_one_clone;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            (weapon_one @ None, weapon_two @ Some(_)) => {
                *weapon_one = weapon_two.clone();
                *weapon_two = None;
            }
            _ => {}
        }
    }
}

pub fn random(loadout: &mut Loadout, budget: &mut Budget, config: &Config) {
    let mut rng = thread_rng();

    if let Some(max_cost) = config.max_cost {
        let _ = budget::set_budget(budget, max_cost);
    } else {
        *budget = Budget::default();
    }

    reset_tools(loadout, budget);
    reset_consumables(loadout, budget);

    // Run initial weapon check for weapon_one checking weapon_two.
    let mut weapon_one = loadout.weapon_one.clone();
    let mut weapon_two = loadout.weapon_two.clone();
    initial_weapon(
        loadout,
        budget,
        &mut rng,
        config.option_exists(ToggleOption::Quartermaster),
        &mut weapon_one,
        &weapon_two,
    );

    // Run initial weapon check for weapon_two checking weapon_one.
    initial_weapon(
        loadout,
        budget,
        &mut rng,
        config.option_exists(ToggleOption::Quartermaster),
        &mut weapon_two,
        &weapon_one,
    );
    loadout.weapon_one = weapon_one.clone();
    loadout.weapon_two = weapon_two.clone();

    // We do the always quartermaster check here since other options WILL override it anyway.
    if config.option_exists(ToggleOption::Quartermaster)
        && config.option_exists(ToggleOption::AlwaysQuartermaster)
    {
        always_quartermaster(loadout, budget, &mut rng);
    }

    // If we requested always dual wield and the weapon isn't locked, always look for a random
    // dual wield weapon.
    if config.option_exists(ToggleOption::AlwaysDualWield) {
        let mut weapon_one = loadout.weapon_one.clone();
        let mut weapon_two = loadout.weapon_two.clone();
        always_dual_wield(budget, &mut rng, &mut weapon_one);
        always_dual_wield(budget, &mut rng, &mut weapon_two);
        loadout.weapon_one = weapon_one.clone();
        loadout.weapon_two = weapon_two.clone();
    }

    // If we don't want duplicate weapons, replace it. and weapon two isn't locked, replace
    // weapon two. We also do the custom ammo check in here to prevent issues with weapon
    // mis-matching custom ammo.
    if !config.option_exists(ToggleOption::DuplicateWeapons) {
        dedupe_weapons(
            loadout,
            budget,
            &mut rng,
            config.option_exists(ToggleOption::Quartermaster),
        );
    }

    if config.option_exists(ToggleOption::CustomAmmo) {
        let mut weapon_one = loadout.weapon_one.clone();
        let mut weapon_two = loadout.weapon_two.clone();
        let always_custom_ammo = config.option_exists(ToggleOption::AlwaysCustomAmmo);
        custom_ammo(budget, &mut rng, &mut weapon_one, always_custom_ammo);
        custom_ammo(budget, &mut rng, &mut weapon_two, always_custom_ammo);
        loadout.weapon_one = weapon_one.clone();
        loadout.weapon_two = weapon_two.clone();
    }

    if config.option_exists(ToggleOption::DuplicateWeapons)
        && config.option_exists(ToggleOption::AlwaysDuplicateWeapons)
    {
        always_duplicate_weapons(loadout, budget);
    }

    budget::transfer_weapons_to_tools(budget);

    random_tools(loadout, budget, config, &mut rng);

    budget::transfer_tools_to_consumables(budget);

    random_consumables(loadout, budget, &mut rng);
    sort_weapons(loadout);
}

pub fn check_loadout_validity(loadout: &mut Loadout, quartermaster: bool) -> Vec<LoadoutInvalid> {
    let mut invalid_checks = vec![];

    if let (Some(weapon_one), Some(weapon_two)) =
        (&loadout.weapon_one.item, &loadout.weapon_two.item)
    {
        if let (Some(weapon_one_slot), Some(weapon_two_slot)) = (&weapon_one.slot, &weapon_two.slot)
        {
            let valid_slots = get_valid_slots(quartermaster, weapon_one_slot);

            if !valid_slots.contains(weapon_two_slot) {
                // Both weapons are invalid since if one weapon is the wrong size either can be
                // changed to make the loadout valid in most cases.
                invalid_checks.push(LoadoutInvalid::WeaponSlot(0));
                invalid_checks.push(LoadoutInvalid::WeaponSlot(1));
            }
        }
    }

    // If there's a duplicate tool it's invalid, you can't have more than one of the same tool.
    let mut used_tools = vec![];
    for (tool_id, tool) in loadout.tools.iter().enumerate() {
        if let Some(tool) = &tool.item {
            if used_tools.contains(tool) {
                invalid_checks.push(LoadoutInvalid::ToolSlot(tool_id.try_into().unwrap_or(0)));
            } else {
                used_tools.push(tool.clone());
            }
        }
    }

    // I don't think there is a way for consumables to be invalid but there is an enum there
    // just incase I'm missing something.

    invalid_checks
}
