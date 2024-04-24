use std::sync::Arc;
use serde::*;

use crate::define_data_enum;
use crate::ship::*;
use crate::data_def::*;
use super::Faction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equip {
    pub kind: EquipKind,
    pub faction: Faction,
    #[serde(default = "make_empty_arc", skip_serializing_if = "is_empty_arc")]
    pub hull_allowed: Arc<[HullType]>,
    #[serde(default = "make_empty_arc", skip_serializing_if = "is_empty_arc")]
    pub hull_disallowed: Arc<[HullType]>,
    #[serde(default = "make_empty_arc", skip_serializing_if = "is_empty_arc")]
    pub weapons: Arc<[Weapon]>,
    #[serde(default = "make_empty_arc", skip_serializing_if = "is_empty_arc")]
    pub stat_bonuses: Arc<[EquipStatBonus]>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub weapon_id: u32,
    pub reload_time: f32,
    pub fixed_delay: f32,
    pub data: WeaponData
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Barrage {
    pub damage: f32,
    pub coefficient: f32,
    pub scaling: f32,
    pub scaling_stat: StatKind,
    pub range: f32,
    pub firing_angle: f32,
    pub bullets: Arc<[Bullet]>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bullet {
    pub bullet_id: u32,

    // From bullet template:
    pub pierce: u32,
    pub ammo: AmmoKind,
    pub kind: BulletKind,
    pub velocity: f32,
    pub modifiers: ArmorModifiers,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spread: Option<BulletSpread>,
    
    // From barrage template:
    pub amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulletSpread {
    pub spread_x: f32,
    pub spread_y: f32,
    pub hit_range: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aircraft {
    pub aircraft_id: u32,
    pub amount: u32,
    pub speed: f32,
    pub weapons: Arc<[Weapon]>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeaponData {
    Bullets(Barrage),
    Aircraft(Aircraft),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmorModifiers(pub f32, pub f32, pub f32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipStatBonus {
    pub stat_kind: StatKind,
    pub amount: f32
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EquipKind {
    DestroyerGun,
    LightCruiserGun,
    HeavyCruiserGun,
    LargeCruiserGun,
    BattleshipGun,
    SurfaceTorpedo,
    SubmarineTorpedo,
    AntiAirGun,
    FuzeAntiAirGun,
    Fighter,
    DiveBomber,
    TorpedoBomber,
    SeaPlane,
    AntiSubWeapon,
    AntiSubAircraft,
    Helicopter,
    Missile,
    Cargo,
    Auxiliary,
}

define_data_enum! {
    pub enum BulletKind for BulletKindData {
        pub name: &'static str;

        Cannon("Cannon"),
        Bomb("Bomb"),
        Torpedo("Torpedo"),
        Direct("Direct"),
        Shrapnel("Shrapnel"),
        AntiAir("AntiAir"),
        AntiSea("AntiSea"),
        Effect("Effect"),
        Beam("Beam"),
        GBullet("GBullet"),
        EletricArc("Eletric Arc"),
        Missile("Missile"),
        SpaceLaser("Space Laser"),
        Scale("Scale"),
        TriggerBomb("Trigger Bomb"),
        AAMissile("AA Missile")
    }
}

define_data_enum! {
    pub enum AmmoKind for AmmoKindData {
        pub name: &'static str;

        Normal("Normal"),
        AP("AP"),
        HE("HE"),
        Torpedo("Torpedo"),
        Unknown5("5"),
        Bomb("Bomb"),
        SAP("SAP"),
        Unknown8("8"),
        Unknown9("9")
    }
}

impl ArmorModifiers {
    pub fn get_modifier(&self, armor_kind: ShipArmor) -> f32 {
        match armor_kind {
            ShipArmor::Light => self.0,
            ShipArmor::Medium => self.1,
            ShipArmor::Heavy => self.2,
        }
    }
}
