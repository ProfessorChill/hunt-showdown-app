use crate::content::ToolSlotPreference;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub dual_wield: bool,
    pub duplicate_weapons: bool,
    pub custom_ammo: bool,
    pub quartermaster: bool,
    pub always_dual_wield: bool,
    pub always_duplicate_weapons: bool,
    pub always_custom_ammo: bool,
    pub always_quartermaster: bool,
    pub max_rank: u8,
    pub max_cost: Option<u16>,
    pub tool_preferences: [ToolSlotPreference; 4],
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dual_wield: true,
            duplicate_weapons: true,
            custom_ammo: true,
            quartermaster: false,
            always_dual_wield: false,
            always_duplicate_weapons: false,
            always_custom_ammo: false,
            always_quartermaster: false,
            max_rank: 100,
            max_cost: None,
            tool_preferences: [ToolSlotPreference::NoPreference; 4],
        }
    }
}
