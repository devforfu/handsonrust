use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        let mut tiles = vec![TileType::Floor; NUM_TILES];
        tiles[0] = TileType::Wall;
        Self { tiles }
    }

//    pub fn render(&self, ctx: &mut BTerm) {
//        for y in 0..SCREEN_HEIGHT {
//            for x in 0..SCREEN_WIDTH {
//                let idx = map_idx(x, y);
//                match self.tiles[idx] {
//                    TileType::Floor => ctx.set(x, y, YELLOW, BLACK, to_cp437('.')),
//                    TileType::Wall => ctx.set(x, y, GREEN, BLACK, to_cp437('#')),
//                }
//            }
//        }
//    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if in_bounds(Point::new(x, y)) {
                    let idx = map_idx(x, y);
                    let char = match self.tiles[idx] {
                        TileType::Floor => '.',
                        TileType::Wall => '#',
                    };
                    ctx.set(x - camera.left_x, y - camera.top_y, WHITE, BLACK, to_cp437(char));
                }
            }
        }
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        if let Some(index) = try_idx(point) {
            in_bounds(point) && self.tiles[index] == TileType::Floor
        } else {
            false
        }
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub fn try_idx(point: Point) -> Option<usize> {
    if in_bounds(point) {
        Some(map_idx(point.x, point.y))
    } else {
        None
    }
}

fn in_bounds(point: Point) -> bool {
    point.x >= 0 &&
    point.x < SCREEN_WIDTH &&
    point.y >= 0 &&
    point.y < SCREEN_HEIGHT
}