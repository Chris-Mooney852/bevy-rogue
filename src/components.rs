use crate::prelude::*;

#[derive(Component)]
pub struct Player {}

#[derive(Component, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Blocking {}
