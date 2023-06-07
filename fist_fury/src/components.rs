use bevy::prelude::*;

#[derive(Component)]
pub struct Fighter1 { // Fighter 1 component
    pub health: f32,
}

#[derive(Component)]
pub struct Fighter2 { // Fighter 2 component
    pub health: f32,
}

#[derive(Component)]
pub struct Fighter1Health {
}

#[derive(Component)]
pub struct Fighter2Health {
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Idle {
    pub has_started : bool,
}

#[derive(Component)]
pub struct MoveLeft {
    pub has_started : bool,
}

#[derive(Component)]
pub struct MoveRight {
    pub has_started : bool,
}

#[derive(Component)]
pub struct LightAttack {
    pub has_started : bool,
}