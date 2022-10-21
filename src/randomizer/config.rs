use crate::content::ToolSlotPreference;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleOption {
    DualWield,
    DuplicateWeapons,
    CustomAmmo,
    Quartermaster,
    AlwaysDualWield,
    AlwaysDuplicateWeapons,
    AlwaysCustomAmmo,
    AlwaysQuartermaster,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub toggled_options: Vec<ToggleOption>,
    pub max_rank: u8,
    pub max_cost: Option<u16>,
    pub tool_preferences: [ToolSlotPreference; 4],
    pub long_ammo_chance: f32,
    pub medium_ammo_chance: f32,
    pub compact_ammo_chance: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            toggled_options: vec![
                ToggleOption::DualWield,
                ToggleOption::DuplicateWeapons,
                ToggleOption::CustomAmmo,
            ],
            max_rank: 100,
            max_cost: None,
            tool_preferences: [ToolSlotPreference::NoPreference; 4],
            long_ammo_chance: 33.33,
            medium_ammo_chance: 33.33,
            compact_ammo_chance: 33.33,
        }
    }
}

impl Config {
    pub fn remove_option(&mut self, option: ToggleOption) {
        if let Some(option_pos) = self.toggled_options.iter().position(|x| x == &option) {
            self.toggled_options.remove(option_pos);
        }
    }

    pub fn toggle_option(&mut self, option: ToggleOption) {
        if let Some(option_pos) = self.toggled_options.iter().position(|x| x == &option) {
            self.toggled_options.remove(option_pos);
        } else {
            self.toggled_options.push(option);
        }
    }

    pub fn option_exists(&self, option: ToggleOption) -> bool {
        self.toggled_options.contains(&option)
    }
}
