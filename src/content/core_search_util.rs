use crate::content::{GenericItem, Slot, UsageType};

// This was a bit bit of an oversight, but finding decoys using conventional usage_type searching
// is not viable since decoys are typically "throwable", "melee" and cause damage... Which is the
// same criteria for throwable weapons.
//
// As a result, finding many tool types is difficult and will be limited to "name searching" and
// string matching which is not ideal. At least until I implement types in the json files.
const VALID_DECOY_NAMES: &[&str] = &["Decoys", "Blank Fire Decoys", "Decoy Fuses"];
const VALID_OTHER_TOOLS: &[&str] = &[
    "Electric Lamp",
    "Spyglass",
    "Flare Pistol",
    "Quad Derringer",
];

pub struct CoreSearchUtil {
    pub tools: Vec<GenericItem>,
    pub consumables: Vec<GenericItem>,
    pub weapons: Vec<GenericItem>,
}

impl CoreSearchUtil {
    pub fn get_weapons_by_sizes(&self, sizes: &[Slot]) -> Vec<&GenericItem> {
        self.weapons
            .iter()
            .filter(|weapon| {
                weapon
                    .slot
                    .as_ref()
                    .map_or(false, |slot| sizes.contains(slot))
            })
            .collect::<Vec<&GenericItem>>()
    }

    pub fn get_decoy_tools(&self) -> Vec<&GenericItem> {
        self.tools
            .iter()
            .filter(|tool| VALID_DECOY_NAMES.contains(&tool.to_full_name().as_str()))
            .collect::<Vec<&GenericItem>>()
    }

    pub fn get_trip_mines(&self) -> Vec<&GenericItem> {
        self.tools
            .iter()
            .filter(|tool| {
                tool.usage_types
                    .iter()
                    .any(|usage_type| matches!(usage_type, UsageType::Placeable { .. }))
            })
            .collect::<Vec<&GenericItem>>()
    }

    pub fn get_melee_tools(&self) -> Vec<&GenericItem> {
        self.tools
            .iter()
            .filter(|tool| {
                tool.usage_types.iter().all(|usage_type| {
                    matches!(
                        usage_type,
                        UsageType::BasicMelee { .. } | UsageType::HeavyMelee { .. }
                    )
                }) && !VALID_OTHER_TOOLS.contains(&tool.to_full_name().as_str())
            })
            .collect::<Vec<&GenericItem>>()
    }

    pub fn get_throwables(&self) -> Vec<&GenericItem> {
        self.tools
            .iter()
            .filter(|tool| {
                tool.usage_types
                    .iter()
                    .any(|usage_type| matches!(usage_type, UsageType::Throw { .. }))
                    && !VALID_DECOY_NAMES.contains(&tool.to_full_name().as_str())
            })
            .collect::<Vec<&GenericItem>>()
    }

    pub fn get_other_tools(&self) -> Vec<&GenericItem> {
        self.tools
            .iter()
            .filter(|tool| VALID_OTHER_TOOLS.contains(&tool.to_full_name().as_str()))
            .collect::<Vec<&GenericItem>>()
    }

    pub fn name_is_tool(&self, name: &str) -> bool {
        self.tools.iter().any(|tool| tool.name == name)
    }

    pub fn name_is_consumable(&self, name: &str) -> bool {
        self.consumables
            .iter()
            .any(|consumable| consumable.name == name)
    }

    pub fn get_dual_wield_weapons(&self) -> Vec<&GenericItem> {
        self.weapons
            .iter()
            .filter(|weapon| weapon.can_dual_wield())
            .collect::<Vec<&GenericItem>>()
    }
}
