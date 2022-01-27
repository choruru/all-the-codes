use crate::commands::smite::model::core::*;

pub struct Achilles {}
impl God for Achilles {
    fn passive(&self) -> Passive {
        Passive {
            effects: btreeset! {
                Effect::Buff(Buff::Health),
                Effect::Buff(Buff::PhysicalProtection),
                Effect::Buff(Buff::MagicalProtection),
                Effect::Buff(Buff::MovementSpeed),
                Effect::Buff(Buff::PhysicalPower),
            },
        }
    }

    fn ability1(&self) -> Ability {
        Ability {
            aa_cancel: true,
            cast_type: CastType::Cone,
            dagame_type: DamageType::Burst,
            effects: btreeset! {
                Effect::CC(CC::HardCC(HardCC::Stun)),
            },
        }
    }

    fn ability2(&self) -> Ability {
        Ability {
            aa_cancel: true,
            cast_type: CastType::SelfBuff,
            dagame_type: DamageType::No,
            effects: btreeset! {
                Effect::Buff(Buff::Health),
                Effect::Buff(Buff::PhysicalProtection),
                Effect::Buff(Buff::MagicalProtection),
                Effect::Buff(Buff::MovementSpeed),
                Effect::Buff(Buff::PhysicalPower),
            },
        }
    }

    fn ability3(&self) -> Ability {
        Ability {
            aa_cancel: true,
            cast_type: CastType::Dash,
            dagame_type: DamageType::Burst,
            effects: btreeset! {},
        }
    }

    fn ability4(&self) -> Ability {
        Ability {
            aa_cancel: true,
            cast_type: CastType::Dash,
            dagame_type: DamageType::Burst,
            effects: btreeset! {
                // Execution
            },
        }
    }
}
