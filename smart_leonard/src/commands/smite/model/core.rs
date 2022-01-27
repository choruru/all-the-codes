use super::gods::achilles::Achilles;
use super::gods::hercules::Hercules;
use lazy_static::lazy_static;
use std::collections::{BTreeSet, HashMap};

lazy_static! {
    static ref GODS: HashMap<&'static str, Option<&'static dyn God>> = hashmap! {
        "Achilles" => Some((&Achilles {}) as &dyn God),
        "Agni"=> None,
        "Ah Muzen Cab"=> None,
        "Ah Puch"=> None,
        "Amaterasu"=> None,
        "Anhur"=> None,
        "Anubis"=> None,
        "Ao Kuang"=> None,
        "Aphrodite"=> None,
        "Apollo"=> None,
        "Arachne"=> None,
        "Ares"=> None,
        "Artemis"=> None,
        "Artio"=> None,
        "Athena"=> None,
        "Atlas"=> None,
        "Awilix"=> None,
        "Baba Yaga"=> None,
        "Bacchus"=> None,
        "Bakasura"=> None,
        "Baron Samedi"=> None,
        "Bastet"=> None,
        "Bellona"=> None,
        "Cabrakan"=> None,
        "Camazotz"=> None,
        "Cerberus"=> None,
        "Cernunnos"=> None,
        "Chaac"=> None,
        "Chang'e"=> None,
        "Charybdis"=> None,
        "Chernobog"=> None,
        "Chiron"=> None,
        "Chronos"=> None,
        "Cliodhna"=> None,
        "Cthulhu"=> None,
        "Cu Chulainn"=> None,
        "Cupid"=> None,
        "Da Ji"=> None,
        "Danzaburou"=> None,
        "Discordia"=> None,
        "Erlang Shen"=> None,
        "Eset"=> None,
        "Fafnir"=> None,
        "Fenrir"=> None,
        "Freya"=> None,
        "Ganesha"=> None,
        "Geb"=> None,
        "Gilgamesh"=> None,
        "Guan Yu"=> None,
        "Hachiman"=> None,
        "Hades"=> None,
        "He Bo"=> None,
        "Heimdallr"=> None,
        "Hel"=> None,
        "Hera"=> None,
        "Hercules"=> Some(&Hercules {}),
        "Horus"=> None,
        "Hou Yi"=> None,
        "Hun Batz"=> None,
        "Izanami"=> None,
        "Janus"=> None,
        "Jing Wei"=> None,
        "Jormungandr"=> None,
        "Kali"=> None,
        "Khepri"=> None,
        "King Arthur"=> None,
        "Kukulkan"=> None,
        "Kumbhakarna"=> None,
        "Kuzenbo"=> None,
        "Loki"=> None,
        "Medusa"=> None,
        "Mercury"=> None,
        "Merlin"=> None,
        "Morgan Le Fay"=> None,
        "Mulan"=> None,
        "Ne Zha"=> None,
        "Neith"=> None,
        "Nemesis"=> None,
        "Nike"=> None,
        "Nox"=> None,
        "Nu Wa"=> None,
        "Odin"=> None,
        "Olorun"=> None,
        "Osiris"=> None,
        "Pele"=> None,
        "Persephone"=> None,
        "Poseidon"=> None,
        "Ra"=> None,
        "Raijin"=> None,
        "Rama"=> None,
        "Ratatoskr"=> None,
        "Ravana"=> None,
        "Scylla"=> None,
        "Serqet"=> None,
        "Set"=> None,
        "Shiva"=> None,
        "Skadi"=> None,
        "Sobek"=> None,
        "Sol"=> None,
        "Sun Wukong"=> None,
        "Susano"=> None,
        "Sylvanus"=> None,
        "Terra"=> None,
        "Thanatos"=> None,
        "The Morrigan"=> None,
        "Thor"=> None,
        "Thoth"=> None,
        "Tiamat"=> None,
        "Tsukuyomi"=> None,
        "Tyr"=> None,
        "Ullr"=> None,
        "Vamana"=> None,
        "Vulcan"=> None,
        "Xbalanque"=> None,
        "Xing Tian"=> None,
        "Yemoja"=> None,
        "Ymir"=> None,
        "Zeus"=> None,
        "Zhong Kui" => None
    };
}

pub trait God: Sync {
    fn passive(&self) -> Passive;
    fn ability1(&self) -> Ability;
    fn ability2(&self) -> Ability;
    fn ability3(&self) -> Ability;
    fn ability4(&self) -> Ability;
}

pub struct Passive {
    pub effects: BTreeSet<Effect>,
}

pub struct Ability {
    pub aa_cancel: bool,
    pub cast_type: CastType,
    pub dagame_type: DamageType,
    pub effects: BTreeSet<Effect>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum AACable {
    Instant,
    Slow,
    AfterJump,
    AfterDash,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum CastType {
    SelfBuff,
    Circle,
    RangedCircle,
    Dash,
    Line,
    Cone,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum DamageType {
    Burst,
    Tick,
    Timer,
    No,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Effect {
    Buff(Buff),
    CC(CC),
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Buff {
    Health,
    Mana,
    MovementSpeed,
    Range,
    AttackSpeed,
    PhysicalPower,
    MagicalPower,
    PhysicalProtection,
    MagicalProtection,
    HP5,
    MP5,
    CCR,
    CDR,
    LifeSteal,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum CC {
    SoftCC(SoftCC),
    HardCC(HardCC),
    RampUpCC,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum SoftCC {
    Bilnd,
    Cripple,
    Root,
    Slow,
    Vortex,
    Deafen,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum HardCC {
    Banish,
    Disarm,
    Disorient,
    Fear,
    Grab,
    Intoxication,
    Knockback,
    KnockUp,
    Madness,
    Mesmerize,
    Polymorph,
    Pull,
    Silence,
    SlipperySurface,
    Stun,
    Taunt,
    Tremble,
}
