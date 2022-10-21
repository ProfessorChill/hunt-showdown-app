pub mod bullet;
pub mod bullet_size;
pub mod bullet_variant;
pub mod core_search_util;
pub mod generic_item;
pub mod tool_slot_preference;
pub mod weapon_variant;

pub use bullet::Bullet;
pub use bullet_size::BulletSize;
pub use bullet_variant::BulletVariant;
pub use core_search_util::CoreSearchUtil;
pub use generic_item::GenericItem;
pub use tool_slot_preference::ToolSlotPreference;
pub use weapon_variant::WeaponVariant;

use serde::Deserialize;
use serde_repr::Deserialize_repr;

lazy_static! {
    pub static ref CORE_SEARCH_UTIL: CoreSearchUtil = CoreSearchUtil {
        tools: match serde_json::from_str(include_str!("../../data/tools.json")) {
            Ok(val) => val,
            Err(err) => {
                log::error!("Unable to read tools json: {}", err);
                panic!();
            }
        },
        consumables: match serde_json::from_str(include_str!("../../data/consumables.json")) {
            Ok(val) => val,
            Err(err) => {
                log::error!("Unable to read consumables json: {}", err);
                panic!();
            }
        },
        weapons: match serde_json::from_str(include_str!("../../data/weapons.json")) {
            Ok(val) => val,
            Err(err) => {
                log::error!("Unable to read weapons json: {}", err);
                panic!();
            }
        },
    };
}

#[derive(Clone, Eq, PartialEq)]
pub enum ItemVariant {
    Weapon,
    Consumable,
    Tool,
}

#[derive(Clone, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExtractCategory {
    Light,
}

#[derive(Clone, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UtilityType {
    Antidote,
    Blunt,
    Choke,
    DerringerBullet,
    Explosion,
    Fire,
    Heal,
    Light,
    Noisy,
    Piercing,
    Poison,
    Rending,
    Silent,
    Stamina,
}

#[derive(Clone, Deserialize_repr, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Intensity {
    Light = 0,
    Medium,
    Heavy,
}

#[derive(Clone, Deserialize_repr, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Slot {
    Small = 0,
    Medium,
    Large,
}

#[derive(Clone, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "req_type", rename_all = "snake_case")]
pub enum PreviousRequirement {
    Ammo {
        weapon: String,
        ammo: BulletVariant,
    },
    Consumable {
        consumable: String,
    },
    Tool {
        tool: String,
    },
    Weapon {
        weapon: String,
        variant: Option<WeaponVariant>,
    },
}

#[derive(Clone, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "t", content = "c", rename_all = "snake_case")]
pub enum Requirement {
    Rank(u8),
    Experience {
        amount: u16,
        variant: String,
    },
    Extract {
        category: ExtractCategory,
        times: u8,
    },
    Use {
        category: UtilityType,
        times: u8,
    },
    PreviousRequirements(PreviousRequirement),
}

#[derive(Clone, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "t", content = "c", rename_all = "snake_case")]
pub enum UsageType {
    BasicMelee {
        types: Vec<UtilityType>,
        intensity: Option<Intensity>,
        damage: u16,
    },
    HeavyMelee {
        types: Vec<UtilityType>,
        intensity: Option<Intensity>,
        damage: u16,
    },
    Placeable {
        types: Vec<UtilityType>,
        intensity: Option<Intensity>,
        damage: Option<u16>,
        damage_per_tick: Option<u8>,
        duration: Option<u16>,
        effect_radius: Option<u8>,
    },
    Shoot {
        bullet_types: Vec<Bullet>,
        bullet_size: BulletSize,
        rate_of_fire: u16,
        reload_speed: u8,
    },
    // This is quite literally only here for the LeMat Mark II
    ShootSecondary {
        bullet_types: Vec<Bullet>,
        bullet_size: BulletSize,
        rate_of_fire: u16,
        reload_speed: u8,
    },
    Throw {
        types: Vec<UtilityType>,
        intensity: Option<Intensity>,
        damage: Option<u16>,
        damage_per_tick: Option<u8>,
        effective_range: u16,
        rate_of_fire: Option<u16>,
        reload_speed: Option<u8>,
        muzzle_velocity: Option<u16>,
        effective_radius: Option<u8>,
        effect_duration: Option<u16>,
        effect_radius: Option<u8>,
        control_range: Option<u8>,
    },
    ThrowLight {
        types: Vec<UtilityType>,
        intensity: Option<Intensity>,
        damage: u16,
        duration: u16,
    },
    Use {
        types: Vec<UtilityType>,
        heal_amount: Option<u8>,
        effect_duration: Option<u16>,
    },
}
