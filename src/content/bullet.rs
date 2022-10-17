use serde::Deserialize;

use crate::content::{BulletVariant, Requirement, UtilityType};

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct Bullet {
    pub name: Option<BulletVariant>,
    pub cost: Option<u16>,
    pub types: Option<Vec<UtilityType>>,
    pub ammo: Vec<u8>,
    pub damage: u16,
    pub effective_range: u16,
    pub handling: u8,
    pub muzzle_velocity: u16,
    pub requirements: Option<Vec<Requirement>>,
}
