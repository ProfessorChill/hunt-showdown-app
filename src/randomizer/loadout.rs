use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::cmp::Ordering;

use crate::content::{
    generic_item::GenericItemLockable, BulletSize, BulletVariant, GenericItem, Slot,
    ToolSlotPreference, UsageType, CORE_SEARCH_UTIL,
};
use crate::randomizer::budget::Transaction;
use crate::randomizer::{budget, Budget, Config, LoadoutInvalid};

const ERR_INSF_FND_LOCK: &str =
    "Insufficient Funds, try unlocking this item or increase your budget.";
const MAX_DUPE_CHECK_AMOUNT: usize = 10;
pub const INVALID_DUALWIELD_NAMES: &[&str] =
    &["Cavalry Saber", "Hand Crossbow", "Combat Axe", "Machete"];

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

pub fn is_dual_wield(weapon_one: &GenericItem, weapon_two: &GenericItem) -> bool {
    matches!(
        (
            &weapon_one.slot,
            weapon_one.dual_wield,
            &weapon_two.slot,
            weapon_two.dual_wield,
        ),
        (Some(Slot::Small), true, Some(Slot::Large), false)
            | (Some(Slot::Large), false, Some(Slot::Small), true)
            | (Some(Slot::Medium), false, Some(Slot::Large), false)
            | (Some(Slot::Large), false, Some(Slot::Medium), false)
    )
}

pub fn refund_item(budget: &mut Budget, item: &GenericItemLockable) {
    if let Some(item) = &item.item {
        for ammo_type in item.ammo_equipped.iter() {
            if ammo_type.2 > 0 {
                let _tx = budget::process_transaction(
                    budget,
                    Transaction::Bullet(
                        true,
                        ammo_type.2,
                        match &ammo_type.1 {
                            Some(variant) => variant.to_string(),
                            None => ammo_type.0.to_string(),
                        },
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
        for ammo_type in item.ammo_equipped.iter() {
            if ammo_type.2 > 0 {
                let tx_res = budget::process_transaction(
                    budget,
                    Transaction::Bullet(
                        false,
                        ammo_type.2,
                        match &ammo_type.1 {
                            Some(variant) => variant.to_string(),
                            None => ammo_type.0.to_string(),
                        },
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
    // Rusts powerfun matching option makes advanced generation nice and easy!
    let new_weapon = match (&mut weapon.item, weapon.locked) {
        // If weapon one is locked and exists, process the weapon transaction.
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

            // Copy itself, we're setting itself to itself.
            Some(weapon.clone())
        }
        // The weapon one exists but isn't locked, or weapon one doesn't exist and isn't
        // locked.
        (Some(_), false) | (None, false) => {
            // If weapon two exists and is locked we want a weapon based on that, otherwise
            // generate a random weapon.
            match (&check.item, check.locked) {
                (Some(check_weapon), _) => {
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

                    new_weapon
                }
                _ => {
                    let weapons = CORE_SEARCH_UTIL
                        .weapons
                        .iter()
                        .collect::<Vec<&GenericItem>>();

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

                    new_weapon
                }
            }
        }
        // The weapon doesn't exist and is locked, we don't want to generate an item in that
        // slot, we can safely set the new_weapon to None.
        (None, true) => None,
    };

    weapon.item = new_weapon;
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
                    slot: slot.try_into().unwrap_or(0),
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
                    slot: slot.try_into().unwrap_or(0),
                });
            }
        }
    }
}

pub fn always_quartermaster(loadout: &mut Loadout, budget: &mut Budget, rng: &mut ThreadRng) {
    match (&mut loadout.weapon_one, &mut loadout.weapon_two) {
        // Both weapons are unlocked and exist.
        (
            GenericItemLockable {
                locked: false,
                item: Some(weapon_one),
            },
            GenericItemLockable {
                locked: false,
                item: Some(weapon_two),
            },
        ) => {
            // Check if they're already valid quartermaster.
            if is_dual_wield(weapon_one, weapon_two) {
                return;
            }

            match (weapon_one.get_slot(), weapon_two.get_slot()) {
                // Weapon one is small, weapon two is large, replace weapon two with a medium
                // slot weapon.
                (Slot::Small, Slot::Large) => {
                    let _tx = budget::process_transaction(
                        budget,
                        Transaction::Weapon(true, weapon_one.get_cost(), weapon_one.to_full_name()),
                    )
                    .ok();

                    let weapons = CORE_SEARCH_UTIL.get_weapons_by_sizes(&[Slot::Medium]);
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
                            new_weapon = None;
                        }
                    }

                    loadout.weapon_one.item = new_weapon;
                }
                // Weapon one is large, weapon two is small, replace weapon two with a medium
                // slot weapon.
                (Slot::Large, Slot::Small) => {
                    let _tx = budget::process_transaction(
                        budget,
                        Transaction::Weapon(true, weapon_two.get_cost(), weapon_two.to_full_name()),
                    )
                    .ok();

                    let weapons = CORE_SEARCH_UTIL.get_weapons_by_sizes(&[Slot::Medium]);
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
                            new_weapon = None;
                        }
                    }

                    loadout.weapon_two.item = new_weapon;
                }
                // Both weapons are medium slot, replace weapon one with a large slot weapon,
                // just preference of ordering.
                (Slot::Medium, Slot::Medium) => {
                    let _tx = budget::process_transaction(
                        budget,
                        Transaction::Weapon(true, weapon_one.get_cost(), weapon_one.to_full_name()),
                    )
                    .ok();

                    let weapons = CORE_SEARCH_UTIL.get_weapons_by_sizes(&[Slot::Large]);
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
                            new_weapon = None;
                        }
                    }

                    loadout.weapon_one.item = new_weapon;
                }
                (Slot::Medium, Slot::Small) => {
                    let _tx = budget::process_transaction(
                        budget,
                        Transaction::Weapon(true, weapon_two.get_cost(), weapon_two.to_full_name()),
                    )
                    .ok();

                    let weapons = CORE_SEARCH_UTIL.get_weapons_by_sizes(&[Slot::Large]);
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
                            new_weapon = None;
                        }
                    }

                    loadout.weapon_two.item = new_weapon;
                }
                (Slot::Small, Slot::Medium) => {
                    let _tx = budget::process_transaction(
                        budget,
                        Transaction::Weapon(true, weapon_one.get_cost(), weapon_one.to_full_name()),
                    )
                    .ok();

                    let weapons = CORE_SEARCH_UTIL.get_weapons_by_sizes(&[Slot::Large]);
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
                            new_weapon = None;
                        }
                    }

                    loadout.weapon_one.item = new_weapon;
                }
                // No other options are valid.
                _ => {}
            }
        }
        // Weapon one is locked or unlocked but weapon two doesn't exist and is unlocked
        // so we'll update it based on that.
        (
            GenericItemLockable {
                locked: _,
                item: Some(weapon_one),
            },
            GenericItemLockable {
                locked: false,
                item: None,
            },
        ) => {
            let slots_to_search = match weapon_one.get_slot() {
                Slot::Large => vec![Slot::Medium],
                Slot::Medium => vec![Slot::Large],
                _ => vec![],
            };

            if !slots_to_search.is_empty() {
                let weapons = CORE_SEARCH_UTIL.get_weapons_by_sizes(&slots_to_search);
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
                        new_weapon = None;
                    }
                }

                loadout.weapon_two.item = new_weapon;
            }
        }
        // Weapon two is locked or unlocked but weapon one doesn't exist and is unlocked
        // so we'll update it based on that.
        (
            GenericItemLockable {
                locked: false,
                item: None,
            },
            GenericItemLockable {
                locked: _,
                item: Some(weapon_two),
            },
        ) => {
            let slots_to_search = match weapon_two.get_slot() {
                Slot::Large => vec![Slot::Medium],
                Slot::Medium => vec![Slot::Large],
                _ => vec![],
            };

            if !slots_to_search.is_empty() {
                let weapons = CORE_SEARCH_UTIL.get_weapons_by_sizes(&slots_to_search);
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
                        new_weapon = None;
                    }
                }

                loadout.weapon_one.item = new_weapon;
            }
        }
        // Weapon one is locked and has a weapon, weapon two is not locked and doesn't have a
        // weapon, we will change weapon_two if needed.
        (
            GenericItemLockable {
                locked: true,
                item: Some(weapon_one),
            },
            GenericItemLockable {
                locked: false,
                item: Some(weapon_two),
            },
        ) => {
            if is_dual_wield(weapon_one, weapon_two) {
                return;
            }

            let slots_to_search = match (weapon_one.get_slot(), weapon_two.get_slot()) {
                (Slot::Medium, Slot::Medium) => vec![Slot::Large],
                (Slot::Medium, Slot::Small) => vec![Slot::Large],
                (Slot::Large, Slot::Small) => vec![Slot::Medium],
                _ => vec![],
            };

            if !slots_to_search.is_empty() {
                let weapons = CORE_SEARCH_UTIL.get_weapons_by_sizes(&slots_to_search);
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
                        new_weapon = None;
                    }
                }

                loadout.weapon_two.item = new_weapon;
            }
        }
        // Weapon one is locked and has a weapon, weapon two is not locked and doesn't have a
        // weapon, we will change weapon_two if needed.
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
            if is_dual_wield(weapon_one, weapon_two) {
                return;
            }

            let slots_to_search = match (weapon_two.get_slot(), weapon_one.get_slot()) {
                (Slot::Medium, Slot::Medium) => vec![Slot::Large],
                (Slot::Medium, Slot::Small) => vec![Slot::Large],
                (Slot::Large, Slot::Small) => vec![Slot::Medium],
                _ => vec![],
            };

            if !slots_to_search.is_empty() {
                let weapons = CORE_SEARCH_UTIL.get_weapons_by_sizes(&slots_to_search);
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
                        new_weapon = None;
                    }
                }

                loadout.weapon_one.item = new_weapon;
            }
        }
        _ => {}
    }
}

pub fn always_dual_wield(
    budget: &mut Budget,
    rng: &mut ThreadRng,
    weapon: &mut GenericItemLockable,
) {
    // Temporary workaround to making cost work.
    let mut weapons = CORE_SEARCH_UTIL.get_dual_wield_weapons();
    let mut weapons = weapons
        .iter_mut()
        .map(|weapon| (*weapon).clone())
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
                locked: false,
                item: Some(weapon_one),
            },
            GenericItemLockable {
                locked: false,
                item: Some(weapon_two),
            },
        )
        | (
            GenericItemLockable {
                locked: true,
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
            let _tx = budget::process_transaction(
                budget,
                Transaction::Weapon(true, weapon_two.get_cost(), weapon_two.to_full_name()),
            )
            .ok();
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

            let mut new_weapon = Some(new_weapon);
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

            loadout.weapon_two.item = new_weapon;
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
            let _tx = budget::process_transaction(
                budget,
                Transaction::Weapon(true, weapon_one.get_cost(), weapon_one.to_full_name()),
            )
            .ok();
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

            let mut new_weapon = Some(new_weapon);
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

            loadout.weapon_one.item = new_weapon;
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
            _ => {}
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
            _ => {}
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
            for ammo_type in weapon.ammo_equipped.iter() {
                if ammo_type.1.is_some() {
                    let tx_res = budget::process_transaction(
                        budget,
                        Transaction::Bullet(
                            false,
                            ammo_type.2,
                            match &ammo_type.1 {
                                Some(variant) => variant.to_string(),
                                None => ammo_type.0.to_string(),
                            },
                        ),
                    );

                    if tx_res.is_err() {
                        // Handle not purchasable ammo.
                    }
                }
            }
        }

        return;
    }

    if let Some(weapon) = &mut weapon.item {
        let mut bullet_size_cap = None;

        // Get a vector of bullet size, bullet variant, and bullet cost.
        let bullet_types = weapon
            .usage_types
            .iter()
            .filter_map(|usage_type| match usage_type {
                UsageType::Shoot {
                    bullet_types,
                    bullet_size,
                    ..
                } => {
                    bullet_size_cap = Some(bullet_size.clone());
                    let bullet_types = bullet_types
                        .iter()
                        .filter_map(|bullet| match &bullet.name {
                            Some(variant) => Some((
                                bullet_size.clone(),
                                Some(variant.clone()),
                                if let Some(bullet_cost) = bullet.cost {
                                    if weapon.additional_ammo_slots.unwrap_or(false) {
                                        // As of update 1.10 this is how bullet_cost is calculated.
                                        bullet_cost / 2
                                    } else {
                                        bullet_cost
                                    }
                                } else {
                                    0
                                },
                            )),
                            None => None,
                        })
                        .collect::<Vec<(BulletSize, Option<BulletVariant>, u16)>>();

                    if bullet_types.is_empty() {
                        None
                    } else {
                        Some(bullet_types)
                    }
                }
                _ => None,
            })
            .flatten()
            .collect::<Vec<(BulletSize, Option<BulletVariant>, u16)>>();

        weapon.ammo_equipped = vec![];

        if bullet_types.is_empty() {
            return;
        }

        if always {
            let ammo_type = bullet_types[rng.gen_range(0..bullet_types.len())].clone();
            let tx_res = budget::process_transaction(
                budget,
                Transaction::Bullet(
                    false,
                    ammo_type.2,
                    match &ammo_type.1 {
                        Some(variant) => variant.to_string(),
                        None => ammo_type.0.to_string(),
                    },
                ),
            );
            if tx_res.is_ok() {
                weapon.ammo_equipped.push(ammo_type);
            }

            if weapon.additional_ammo_slots.unwrap_or(false) {
                let ammo_type = bullet_types[rng.gen_range(0..bullet_types.len())].clone();
                let tx_res = budget::process_transaction(
                    budget,
                    Transaction::Bullet(
                        false,
                        ammo_type.2,
                        match &ammo_type.1 {
                            Some(variant) => variant.to_string(),
                            None => ammo_type.0.to_string(),
                        },
                    ),
                );
                if tx_res.is_ok() {
                    weapon.ammo_equipped.push(ammo_type);
                }
            }
        } else {
            if rng.gen_bool(0.25) {
                let ammo_type = bullet_types[rng.gen_range(0..bullet_types.len())].clone();
                let tx_res = budget::process_transaction(
                    budget,
                    Transaction::Bullet(
                        false,
                        ammo_type.2,
                        match &ammo_type.1 {
                            Some(variant) => variant.to_string(),
                            None => ammo_type.0.to_string(),
                        },
                    ),
                );
                if tx_res.is_ok() {
                    weapon.ammo_equipped.push(ammo_type);
                }
            } else if let Some(bullet_size) = &bullet_size_cap {
                weapon.ammo_equipped.push((bullet_size.clone(), None, 0));
            }

            if rng.gen_bool(0.25) && weapon.additional_ammo_slots.unwrap_or(false) {
                let ammo_type = bullet_types[rng.gen_range(0..bullet_types.len())].clone();
                let tx_res = budget::process_transaction(
                    budget,
                    Transaction::Bullet(
                        false,
                        ammo_type.2,
                        match &ammo_type.1 {
                            Some(variant) => variant.to_string(),
                            None => ammo_type.0.to_string(),
                        },
                    ),
                );
                if tx_res.is_ok() {
                    weapon.ammo_equipped.push(ammo_type);
                }
            } else if weapon.additional_ammo_slots.unwrap_or(false) {
                if let Some(bullet_size) = &bullet_size_cap {
                    weapon.ammo_equipped.push((bullet_size.clone(), None, 0));
                }
            }
        }

        if let Some(bullet_size) = &bullet_size_cap {
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
            loadout.tools[slot].item = if let Some(check_tool) = &random_tool {
                let tx_res = budget::process_transaction(
                    budget,
                    Transaction::Tool(false, check_tool.cost, check_tool.to_full_name()),
                );

                if tx_res.is_ok() {
                    Some(check_tool.clone())
                } else {
                    None
                }
            } else {
                None
            };
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
    for consumable in loadout.consumables.iter_mut() {
        if consumable.locked {
            continue;
        }

        let random_consumables = CORE_SEARCH_UTIL
            .consumables
            .iter()
            .collect::<Vec<&GenericItem>>();
        let random_consumable = item_lte_cost(&random_consumables, budget.consumables_budget, rng);

        consumable.item = if let Some(check_consumable) = &random_consumable {
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
        } else {
            None
        };
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
        config.quartermaster,
        &mut weapon_one,
        &weapon_two,
    );
    loadout.weapon_one = weapon_one.clone();

    if config.quartermaster && config.always_quartermaster {
        // Lock weapon two since we ONLY want to ensure weapon one is changed.
        loadout.weapon_two.locked = true;
        always_quartermaster(loadout, budget, &mut rng);
        loadout.weapon_two.locked = initial_weapon_two_lock;
    }

    if config.always_dual_wield {
        let mut weapon_one = loadout.weapon_one.clone();
        always_dual_wield(budget, &mut rng, &mut weapon_one);
        loadout.weapon_one = weapon_one.clone();
    }

    if !config.duplicate_weapons {
        // Again lock weapon two since we ONLY want to ensure weapon one is changed.
        loadout.weapon_two.locked = true;
        dedupe_weapons(loadout, budget, &mut rng, config.quartermaster);
        loadout.weapon_two.locked = initial_weapon_two_lock;
    }

    if config.custom_ammo {
        let mut weapon_one = loadout.weapon_one.clone();
        custom_ammo(budget, &mut rng, &mut weapon_one, config.always_custom_ammo);
        loadout.weapon_one = weapon_one.clone();
    }

    if config.duplicate_weapons && config.always_duplicate_weapons {
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
        config.quartermaster,
        &mut weapon_two,
        &weapon_one,
    );
    loadout.weapon_two = weapon_two.clone();

    if config.quartermaster && config.always_quartermaster {
        // Lock weapon one since we ONLY want to ensure weapon two is changed.
        loadout.weapon_one.locked = true;
        always_quartermaster(loadout, budget, &mut rng);
        loadout.weapon_one.locked = initial_weapon_one_lock;
    }

    if config.always_dual_wield {
        let mut weapon_two = loadout.weapon_two.clone();
        always_dual_wield(budget, &mut rng, &mut weapon_two);
        loadout.weapon_two = weapon_two.clone();
    }

    if !config.duplicate_weapons {
        // Again lock weapon one since we ONLY want to ensure weapon two is changed.
        loadout.weapon_one.locked = true;
        dedupe_weapons(loadout, budget, &mut rng, config.quartermaster);
        loadout.weapon_one.locked = initial_weapon_one_lock;
    }

    if config.custom_ammo {
        let mut weapon_two = loadout.weapon_two.clone();
        custom_ammo(budget, &mut rng, &mut weapon_two, config.always_custom_ammo);
        loadout.weapon_two = weapon_two.clone();
    }

    if config.duplicate_weapons && config.always_duplicate_weapons {
        // ONCE AGAIN, lock weapon one, we don't want to change it.
        loadout.weapon_one.locked = true;
        always_duplicate_weapons(loadout, budget);
        loadout.weapon_one.locked = initial_weapon_one_lock;
    }

    refund_item(&mut previous_budget, &previous_weapon);
    purchase_item(&mut previous_budget, &loadout.weapon_two);
    *budget = previous_budget;
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
        config.quartermaster,
        &mut weapon_one,
        &weapon_two,
    );

    // Run initial weapon check for weapon_two checking weapon_one.
    initial_weapon(
        loadout,
        budget,
        &mut rng,
        config.quartermaster,
        &mut weapon_two,
        &weapon_one,
    );
    loadout.weapon_one = weapon_one.clone();
    loadout.weapon_two = weapon_two.clone();

    // We do the always quartermaster check here since other options WILL override it anyway.
    if config.quartermaster && config.always_quartermaster {
        always_quartermaster(loadout, budget, &mut rng);
    }

    // If we requested always dual wield and the weapon isn't locked, always look for a random
    // dual wield weapon.
    if config.always_dual_wield {
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
    if !config.duplicate_weapons {
        dedupe_weapons(loadout, budget, &mut rng, config.quartermaster);
    }

    if config.custom_ammo {
        let mut weapon_one = loadout.weapon_one.clone();
        let mut weapon_two = loadout.weapon_two.clone();
        custom_ammo(budget, &mut rng, &mut weapon_one, config.always_custom_ammo);
        custom_ammo(budget, &mut rng, &mut weapon_two, config.always_custom_ammo);
        loadout.weapon_one = weapon_one.clone();
        loadout.weapon_two = weapon_two.clone();
    }

    if config.duplicate_weapons && config.always_duplicate_weapons {
        always_duplicate_weapons(loadout, budget);
    }

    budget::transfer_weapons_to_tools(budget);

    random_tools(loadout, budget, config, &mut rng);

    budget::transfer_tools_to_consumables(budget);

    random_consumables(loadout, budget, &mut rng);
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
            if !used_tools.contains(tool) {
                used_tools.push(tool.clone());
            } else {
                invalid_checks.push(LoadoutInvalid::ToolSlot(tool_id.try_into().unwrap_or(0)));
            }
        }
    }

    // I don't think there is a way for consumables to be invalid but there is an enum there
    // just incase I'm missing something.

    invalid_checks
}
