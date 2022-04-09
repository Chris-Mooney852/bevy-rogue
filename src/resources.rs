#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TurnState {
    PlayerTurn,
}

#[derive(Default)]
pub struct SpriteSpecs {
    pub size: f32,
    pub buffer: f32,
}
