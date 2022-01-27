use crate::commands::smite::model::core::*;

pub struct Hercules {}
impl God for Hercules {
    fn passive(&self) -> Passive {
        Passive {
            effects: btreeset! {
                Effect::Buff(Buff::PhysicalPower),
            },
        }
    }

    fn ability1(&self) -> Ability {
        Ability {
            aa_cancel: true,
            cast_type: CastType::Dash,
            dagame_type: DamageType::Burst,
            effects: btreeset! {
                Effect::CC(CC::HardCC(HardCC::Stun)),
            },
        }
    }

    fn ability2(&self) -> Ability {
        Ability {
            aa_cancel: false,
            cast_type: CastType::Line,
            dagame_type: DamageType::Burst,
            effects: btreeset! {
                Effect::CC(CC::HardCC(HardCC::KnockUp)),
            },
        }
    }

    fn ability3(&self) -> Ability {
        Ability {
            aa_cancel: true,
            cast_type: CastType::SelfBuff,
            dagame_type: DamageType::No,
            effects: btreeset! {
                Effect::Buff(Buff::HP5),
                Effect::Buff(Buff::PhysicalProtection),
                Effect::Buff(Buff::MagicalProtection),
                Effect::Buff(Buff::LifeSteal),
            },
        }
    }

    fn ability4(&self) -> Ability {
        Ability {
            aa_cancel: false,
            cast_type: CastType::Line,
            dagame_type: DamageType::Burst,
            effects: btreeset! {},
        }
    }
}
