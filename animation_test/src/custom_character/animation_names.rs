pub enum RpgAnimationNames {
    Death,
    Idle,
    IdleWeapon,
    PickUp,
    Punch,
    RecieveHit,
    RecieveHitAttacking,
    Run,
    Spell1,
    StaffAttack,
    Walk,
}

impl RpgAnimationNames {
    pub fn as_str(&self) -> &str {
        match self {
            RpgAnimationNames::Death => "Death",
            RpgAnimationNames::Idle => "Idle",
            RpgAnimationNames::PickUp => "PickUp",
            RpgAnimationNames::IdleWeapon => "Idle_Weapon",
            RpgAnimationNames::Punch => "Punch",
            RpgAnimationNames::RecieveHit => "RecieveHit",
            RpgAnimationNames::RecieveHitAttacking => "RecieveHit_Attacking",
            RpgAnimationNames::Run => "Run",
            RpgAnimationNames::Spell1 => "Spell1",
            RpgAnimationNames::StaffAttack => "Staff_Attack",
            RpgAnimationNames::Walk => "Walk",
        }
    }
}
