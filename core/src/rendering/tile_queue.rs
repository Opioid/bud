use base::math::{int2, int4};
use std::sync::atomic::{AtomicU32, Ordering};

pub struct TileQueue {
    tile_dimensions: int2,
    tiles_per_row: i32,
    tiles: Vec<int4>,
    current_consume: AtomicU32,
}

impl TileQueue {
    pub fn new(resolution: int2, tile_dimensions: int2, filter_radius: i32) -> TileQueue {
        let tiles_per_row = (resolution.v[0] as f32 / tile_dimensions.v[0] as f32).ceil() as i32;
        let num_tiles = (resolution.v[0] as f32 / tile_dimensions.v[0] as f32).ceil() as u32
            * (resolution.v[1] as f32 / tile_dimensions.v[1] as f32).ceil() as u32;
        let tiles = vec![int4::identity(); num_tiles as usize];

        let mut tq = TileQueue {
            tile_dimensions,
            tiles_per_row,
            tiles,
            current_consume: AtomicU32::new(num_tiles),
        };

        let mut current_pixel = int2::identity();

        loop {
            let mut start = current_pixel;
            let mut end = (current_pixel + tile_dimensions).min(resolution);

            if 0 == start.v[1] {
                start.v[1] -= filter_radius;
            }

            if resolution.v[1] == end.v[1] {
                end.v[1] += filter_radius;
            }

            if 0 == start.v[0] {
                start.v[0] -= filter_radius;
            }

            if resolution.v[0] == end.v[0] {
                end.v[0] += filter_radius;
            }

            tq.push(int4::from_2_2(start, end - int2::from_scalar(1)));

            current_pixel.v[0] += tile_dimensions.v[0];

            if current_pixel.v[0] >= resolution.v[0] {
                current_pixel.v[0] = 0;
                current_pixel.v[1] += tile_dimensions.v[1];
            }

            if current_pixel.v[1] >= resolution.v[1] {
                break;
            }
        }

        tq
    }

    pub fn pop(&mut self) -> Option<int4> {
        let current = self.current_consume.fetch_add(1, Ordering::Relaxed) as usize;

        if current < self.tiles.len() {
            return Some(self.tiles[current]);
        }

        None
    }

    pub fn index(&self, tile: int4) -> u32 {
        let x = tile.v[0].max(0) / self.tile_dimensions.v[0];
        let y = tile.v[1].max(0) / self.tile_dimensions.v[1];

        (y * self.tiles_per_row + x) as u32
    }

    fn push(&mut self, tile: int4) {
        let current =
            self.tiles.len() as u32 - self.current_consume.fetch_sub(1, Ordering::Relaxed);

        self.tiles[current as usize] = tile;
    }
}
