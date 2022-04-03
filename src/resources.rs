#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Default)]
pub struct Map {
    pub tiles: Vec<TileType>,
}

#[derive(Default)]
pub struct SpriteSpecs {
    pub size: f32,
    pub buffer: f32,
}
