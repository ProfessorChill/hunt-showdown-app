use serde::Deserialize;

use crate::content::{
    BulletSize, BulletVariant, Requirement, Slot, UsageType, WeaponVariant, CORE_SEARCH_UTIL,
};
use crate::randomizer::loadout::INVALID_DUALWIELD_NAMES;

// For serde default on dual wield field.
const fn default_false() -> bool {
    false
}

// For serde default on ammo_equipped.
const fn default_ammo() -> Vec<CustomAmmo> {
    vec![]
}

pub type CustomAmmo = (BulletSize, Option<BulletVariant>, u16);

#[derive(Default, Debug, Clone)]
pub struct GenericItemLockable {
    pub item: Option<GenericItem>,
    pub locked: bool,
}

#[derive(Clone, Deserialize, Debug, Eq, PartialEq)]
pub struct GenericItem {
    pub name: String,
    // Only used for Caldwell Conversion Pistol
    pub postfix: Option<String>,
    pub variant: Option<WeaponVariant>,
    pub slot: Option<Slot>,
    pub additional_ammo_slots: Option<bool>,
    pub handling: Option<u8>,
    pub cost: u16,
    pub requirements: Vec<Requirement>,
    #[serde(rename = "types")]
    pub usage_types: Vec<UsageType>,
    pub variants: Option<Vec<WeaponVariant>>,
    // For usage in `struct Loadout`.
    #[serde(default = "default_false")]
    pub dual_wield: bool,
    #[serde(default = "default_ammo")]
    pub ammo_equipped: Vec<CustomAmmo>,
}

impl GenericItem {
    pub const fn get_cost(&self) -> u16 {
        if self.dual_wield {
            self.cost * 2
        } else {
            self.cost
        }
    }

    pub fn get_slot(&self) -> Slot {
        if self.dual_wield {
            Slot::Medium
        } else if let Some(slot) = &self.slot {
            slot.clone()
        } else {
            unreachable!(
                "Not searching for slot on weapon! Item name error: {}",
                self.to_full_name()
            );
        }
    }

    pub fn get_bullet_variants(&self) -> Vec<CustomAmmo> {
        self.usage_types
            .iter()
            .filter_map(|usage_type| match usage_type {
                UsageType::Shoot {
                    bullet_types,
                    bullet_size,
                    ..
                } => Some(
                    bullet_types
                        .iter()
                        .filter_map(|bullet| {
                            bullet.name.as_ref().map(|variant| {
                                (
                                    bullet_size.clone(),
                                    Some(variant.clone()),
                                    bullet.cost.map_or(0, |bullet_cost| {
                                        if self.additional_ammo_slots.unwrap_or(false) {
                                            bullet_cost / 2
                                        } else {
                                            bullet_cost
                                        }
                                    }),
                                )
                            })
                        })
                        .collect::<Vec<CustomAmmo>>(),
                ),
                _ => None,
            })
            .flatten()
            .collect::<Vec<CustomAmmo>>()
    }

    pub fn get_bullet_size(&self) -> Option<BulletSize> {
        let mut found_bullet_size = None;

        for usage_type in &self.usage_types {
            if let UsageType::Shoot { bullet_size, .. } = usage_type {
                found_bullet_size = Some(bullet_size.clone());
                break;
            }
        }

        found_bullet_size
    }

    pub fn can_dual_wield(&self) -> bool {
        if let Some(slot) = &self.slot {
            if *slot == Slot::Small && !INVALID_DUALWIELD_NAMES.contains(&self.name.as_str()) {
                return true;
            }
        }

        false
    }

    pub fn to_full_name(&self) -> String {
        format!(
            "{}{}{}",
            self.name,
            self.variant
                .as_ref()
                .map_or_else(String::new, |variant| format!(" {variant} ")),
            self.postfix
                .as_ref()
                .map_or_else(String::new, |postfix| format!(" {postfix}")),
        )
    }

    pub fn to_image_path(&self) -> String {
        // Just resolves to weapon path until I make a way for it to differentiate
        // weapon/consumable/tool.

        if CORE_SEARCH_UTIL.name_is_tool(&self.name) {
            self.to_tool_path()
        } else if CORE_SEARCH_UTIL.name_is_consumable(&self.name) {
            self.to_consumable_path()
        } else {
            self.to_weapon_path()
        }
    }

    pub fn to_weapon_path(&self) -> String {
        format!(
            "/images/weapons/{}{}{}.webp",
            self.name.replace([' ', '.'], ""),
            self.variant
                .as_ref()
                .map_or_else(String::new, |variant| variant.to_string().replace(' ', "")),
            self.postfix
                .clone()
                .unwrap_or_default()
                .replace([' ', '.', '(', ')'], ""),
        )
    }

    pub fn to_tool_path(&self) -> String {
        format!("/images/tools/{}.webp", self.name.replace([' ', '.'], ""),)
    }

    pub fn to_consumable_path(&self) -> String {
        format!(
            "/images/consumables/{}.webp",
            self.name.replace([' ', '.', '(', ')'], ""),
        )
    }
}
