use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct God {
    pub id: i64,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type")]
    pub type_: String,
    #[serde(rename = "Roles")]
    pub role: String,

    #[serde(rename = "Health")]
    pub health: f64,
    #[serde(rename = "HealthPerLevel")]
    pub health_per_level: f64,
    #[serde(rename = "HealthPerFive")]
    pub health_per_five: f64,
    #[serde(rename = "HP5PerLevel")]
    pub hp5_per_level: f64,

    #[serde(rename = "Mana")]
    pub mana: f64,
    #[serde(rename = "ManaPerLevel")]
    pub mana_per_level: f64,
    #[serde(rename = "ManaPerFive")]
    pub mana_per_five: f64,
    #[serde(rename = "MP5PerLevel")]
    pub mp5_per_level: f64,

    #[serde(rename = "PhysicalPower")]
    pub physical_power: f64,
    #[serde(rename = "PhysicalPowerPerLevel")]
    pub physical_power_per_level: f64,
    #[serde(rename = "PhysicalProtection")]
    pub physical_protection: f64,
    #[serde(rename = "PhysicalProtectionPerLevel")]
    pub physical_protection_per_level: f64,

    #[serde(rename = "MagicalPower")]
    pub magical_power: f64,
    #[serde(rename = "MagicalPowerPerLevel")]
    pub magical_power_per_level: f64,
    #[serde(rename = "MagicProtection")]
    pub magic_protection: f64,
    #[serde(rename = "MagicProtectionPerLevel")]
    pub magic_protection_per_level: f64,

    #[serde(rename = "Speed")]
    pub speed: f64,

    #[serde(rename = "basicAttack")]
    pub basic_attack: BasicAttack,

    #[serde(rename = "Ability5")]
    pub passive_name: String,
    #[serde(rename = "abilityDescription5")]
    pub passive: Ability,

    #[serde(rename = "Ability1")]
    pub ability1_name: String,
    #[serde(rename = "abilityDescription1")]
    pub ability1: Ability,

    #[serde(rename = "Ability2")]
    pub ability2_name: String,
    #[serde(rename = "abilityDescription2")]
    pub ability2: Ability,

    #[serde(rename = "Ability3")]
    pub ability3_name: String,
    #[serde(rename = "abilityDescription3")]
    pub ability3: Ability,

    #[serde(rename = "Ability4")]
    pub ability4_name: String,
    #[serde(rename = "abilityDescription4")]
    pub ability4: Ability,
}

#[derive(Debug, Deserialize)]
pub struct BasicAttack {
    #[serde(rename = "itemDescription")]
    pub item_description: ItemDescription,
}

#[derive(Debug, Deserialize)]
pub struct Ability {
    #[serde(rename = "itemDescription")]
    pub item_description: ItemDescription,
}

#[derive(Debug, Deserialize)]
pub struct ItemDescription {
    pub cooldown: String,
    pub cost: String,
    pub description: String,
    pub menuitems: Vec<Item>,
    pub rankitems: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub description: String,
    pub value: String,
}
