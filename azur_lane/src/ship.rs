//! Data structures relating directly to ships.

use std::fmt::Display;
use serde::*;

use crate::define_data_enum;
use super::Faction;
use super::equip::*;
use super::skill::*;

/// Provides data for a singular ship or a retrofit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipData {
    pub group_id: u32,
    pub name: String,
    pub rarity: ShipRarity,
    pub faction: Faction,
    pub hull_type: HullType,
    pub stars: u8,
    #[serde(default)]
    pub enhance_kind: EnhanceKind,
    pub stats: ShipStatBlock,
    pub equip_slots: Vec<EquipSlot>,
    pub shadow_equip: Vec<ShadowEquip>,
    pub skills: Vec<Skill>,
    pub retrofits: Vec<ShipData>,
    pub skins: Vec<ShipSkin>
}

/// Provides stat block information for a ship.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipStatBlock {
    pub hp: ShipStat,
    pub armor: ShipArmor,
    pub rld: ShipStat,
    pub fp: ShipStat,
    pub trp: ShipStat,
    pub eva: ShipStat,
    pub aa: ShipStat,
    pub avi: ShipStat,
    pub acc: ShipStat,
    pub asw: ShipStat,
    pub spd: f64,
    pub lck: f64,
    pub cost: u32,
    pub oxy: u32,
    pub amo: u32
}

/// Represents a single ship stat. Its value can be calculated on demand.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ShipStat(f64, f64, f64);

/// A singular normal equipment slot of a ship.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipSlot {
    /// Which kinds of equipment can be equipped in the slot.
    pub allowed: Vec<EquipKind>,
    /// If a weapon slot, the data for the mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mount: Option<EquipWeaponMount>
}

/// Mount information for an [`EquipSlot`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipWeaponMount {
    /// The mount efficiency, as displayed in-game.
    pub efficiency: f64,
    /// The amount of mounts.
    pub mounts: u8,
    /// The amount of parallel loads.
    /// 
    /// F.e. Gascogne's main gun and Unzen's torpedo has a value 2.
    pub parallel: u8,
    /// How many preloads this slot has.
    /// 
    /// This is only meaningful for Battleship main guns, torpedoes, and missiles.
    pub preload: u8,
}

/// Provides information about "shadow" equipment; inherent gear that is not displayed in-game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowEquip {
    /// The name of the associated equipment.
    pub name: String,
    /// The mount efficiency. Same meaning as [`EquipWeaponMount::efficiency`].
    pub efficiency: f64,
    /// The weapons on that equipment.
    pub weapons: Vec<Weapon>
}

/// Data for a ship skin. This may represent the default skin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipSkin {
    pub skin_id: u32,
    pub name: String,
    pub description: String,
    /// The default dialogue lines.
    pub words: ShipSkinWords,
    /// Replacement dialogue lines, usually after oath.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub words_extra: Option<Box<ShipSkinWords>>
}

/// The block of dialogue for a given skin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipSkinWords {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acquisition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(default = "Vec::new", skip_serializing_if = "Vec::is_empty")]
    pub main_screen: Vec<ShipMainScreenLine>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub touch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub special_touch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rub: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mission_reminder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mission_complete: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail_reminder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_to_port: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commission_complete: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enhance: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flagship_fight: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub victory: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defeat: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skill: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub low_health: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disappointed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stranger: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub friendly: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crush: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub love: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oath: Option<String>,
    /// Voices lines that may be played when sortieing other specific ships.
    #[serde(default = "Vec::new", skip_serializing_if = "Vec::is_empty")]
    pub couple_encourage: Vec<ShipCoupleEncourage>
}

/// Information about a ship line that may be displayed on the main screen.
/// 
/// Also see [`ShipSkinWords::main_screen`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipMainScreenLine(usize, String);

/// Data for voices lines that may be played when sortieing other specific ships.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipCoupleEncourage {
    pub line: String,
    pub amount: u32,
    pub condition: ShipCouple
}

/// Condition for [`ShipCoupleEncourage`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShipCouple {
    /// Triggered when other specific ships are present.
    /// Holds a vector of ship group IDs.
    ShipGroup(Vec<u32>),
    /// Triggered when ships of specified hull types are present.
    HullType(Vec<HullType>),
    /// Triggered when ships of a specified rarity are present.
    Rarity(Vec<ShipRarity>),
    /// Triggered when ships from a specified faction are present.
    Faction(Vec<Faction>),
    /// Triggered when ships from the same illustrator are present.
    /// 
    /// Actual in-game data specifies which one, but it's only ever used to refer to the same one as the source ship's.
    Illustrator
}

define_data_enum! {
    /// The rarities for a ship.
    pub enum ShipRarity for ShipRarityData {
        /// The display name for the rarity.
        pub name: &'static str,
        /// An RGB color that can be used to represent the rarity. 
        pub color_rgb: u32;

        /// N (Common)
        N("N", 0xC0C0C0),
        /// R (Rare)
        R("R", 0x9FE8FF),
        /// E (Elite)
        E("E", 0xC4ADFF),
        /// SR (Super Rare) / Priority
        SR("SR", 0xEDDD76),
        /// UR (Ultra Rare) / Decisive
        UR("UR", 0xFF8D8D)
    }
}

/// The enhancement mode kind.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EnhanceKind {
    /// Normal. Enhancement by feeding spare duplicate ships.
    #[default]
    Normal,
    /// Research ships enhanced with blueprints.
    Research,
    /// META ships with their own nonsense.
    META
}

define_data_enum! {
    /// The possible stat kinds.
    /// 
    /// Only includes ones that represent a numeric value.
    pub enum StatKind for StatKindData {
        /// The in-game display name.
        pub name: &'static str;

        HP("HP"),
        RLD("RLD"),
        FP("FP"),
        TRP("TRP"),
        EVA("EVA"),
        AA("AA"),
        AVI("AVI"),
        ACC("ACC"),
        ASW("ASW"),
        SPD("SPD"),
        LCK("LCK")
    }
}

define_data_enum! {
    /// The possible hull types/designations for ships.
    pub enum HullType for HullTypeData {
        /// The short-hand designation for the hull type.
        pub designation: &'static str,
        /// The long hull type name.
        pub name: &'static str,
        /// Which team type this hull type gets sortied in.
        pub team_type: TeamType;

        Unknown("??", "Unknown", TeamType::Vanguard),
        Destroyer("DD", "Destroyer", TeamType::Vanguard),
        LightCruiser("CL", "Light Cruiser", TeamType::Vanguard),
        HeavyCruiser("CA", "Heavy Cruiser", TeamType::Vanguard),
        Battlecruiser("BC", "Battlecruiser", TeamType::MainFleet),
        Battleship("BB", "Battleship", TeamType::MainFleet),
        LightCarrier("CVL", "Light Carrier", TeamType::MainFleet),
        AircraftCarrier("CV", "Aircraft Carrier", TeamType::MainFleet),
        Submarine("SS", "Submarine", TeamType::Submarine),
        AviationBattleship("BBV", "Aviation Battleship", TeamType::MainFleet),
        RepairShip("AR", "Repair Ship", TeamType::MainFleet),
        Monitor("BM", "Monitor", TeamType::MainFleet),
        AviationSubmarine("SSV", "Aviation Submarine", TeamType::Submarine),
        LargeCruiser("CB", "Large Cruiser", TeamType::Vanguard),
        MunitionShip("AE", "Munition Ship", TeamType::Vanguard),
        MissileDestroyerV("DDG v", "Missile Destroyer V", TeamType::Vanguard),
        MissileDestroyerM("DDG m", "Missile Destroyer M", TeamType::MainFleet),
        FrigateS("IX s", "Sailing Frigate S", TeamType::Submarine),
        FrigateV("IX v", "Sailing Frigate V", TeamType::Vanguard),
        FrigateM("IX m", "Sailing Frigate M", TeamType::MainFleet) 
    }
}

define_data_enum! {
    /// The armor thickness of a ship.
    pub enum ShipArmor for ShipArmorData {
        pub name: &'static str;

        Light("Light"),
        Medium("Medium"),
        Heavy("Heavy")
    }
}

define_data_enum! {
    /// The sortie team types.
    pub enum TeamType for TeamTypeData {
        pub name: &'static str;

        Vanguard("Vanguard"),
        MainFleet("Main Fleet"),
        Submarine("Submarine")
    }
}

impl Display for ShipArmor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl ShipStatBlock {
    /// Gets and calculates a certain stat value.
    #[must_use]
    pub fn calc_stat(&self, kind: StatKind, level: u32, affinity: f64) -> f64 {
        match kind {
            StatKind::HP => self.hp.calc(level, affinity),
            StatKind::RLD => self.rld.calc(level, affinity),
            StatKind::FP => self.fp.calc(level, affinity),
            StatKind::TRP => self.trp.calc(level, affinity),
            StatKind::EVA => self.eva.calc(level, affinity),
            StatKind::AA => self.aa.calc(level, affinity),
            StatKind::AVI => self.avi.calc(level, affinity),
            StatKind::ACC => self.acc.calc(level, affinity),
            StatKind::ASW => self.asw.calc(level, affinity),
            StatKind::SPD => self.spd,
            StatKind::LCK => self.lck
        }
    }
}

impl ShipStat {
    /// Creates a stat with all zeroes.
    #[must_use]
    pub const fn new() -> Self {
        Self(0f64, 0f64, 0f64)
    }

    /// Sets the base value, aka the level 1 stats.
    #[must_use]
    pub const fn set_base(mut self, base: f64) -> Self {
        self.0 = base;
        self
    }

    /// Sets the level growth value.
    #[must_use]
    pub const fn set_growth(mut self, growth: f64) -> Self {
        self.1 = growth;
        self
    }

    /// Sets the fixed addition unaffected by affinity.
    #[must_use]
    pub const fn set_fixed(mut self, fixed: f64) -> Self {
        self.2 = fixed;
        self
    }

    /// The base value, aka the level 1 stats.
    #[must_use]
    pub const fn base(&self) -> f64 { self.0 }

    /// The level growth value.
    #[must_use]
    pub const fn growth(&self) -> f64 { self.1 }

    /// A fixed addition unaffected by affinity.
    #[must_use]
    pub const fn fixed(&self) -> f64 { self.2 }

    /// Calculates the actual value.
    /// 
    /// Depending on how the data was stored, this may be inaccurate for levels below 100.
    #[must_use]
    pub fn calc(&self, level: u32, affinity: f64) -> f64 {
        (self.base() + self.growth() * f64::from(level - 1) * 0.001) * affinity + self.fixed()
    }
}

impl Default for ShipStat {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Add<Self> for ShipStat {
    type Output = Self;

    fn add(self, rhs: ShipStat) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::AddAssign for ShipStat {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl ShipMainScreenLine {
    /// Creates a new instance.
    #[must_use]
    pub fn new(index: usize, text: String) -> Self {
        Self(index, text)
    }

    /// Gets the index for the line. Relevant for replacement.
    #[must_use]
    pub fn index(&self) -> usize {
        self.0
    }

    /// Gets the text associated with the line.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.1
    }
}

impl ShipRarity {
    /// Returns the next higher rarity.
    /// 
    /// For [`ShipRarity::UR`], returns itself.
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::N => Self::R,
            Self::R => Self::E,
            Self::E => Self::SR,
            Self::SR | Self::UR => Self::UR,
        }
    }
}
