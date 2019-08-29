use crate::InstanceData;
use crate::vs::ty::PushConstantData;

mod render_utils;
use render_utils::FPScounter;


const WIN_SIZE: u32 = 500;
const NX_TILES: usize = 26;
const NY_TILES: usize = 9;

pub struct View {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
}

#[derive(Clone, Copy)]
pub enum TileKind {
    Wood,
    Stone,
    Metal,
    RedStone,
}

#[derive(Clone, Copy)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub kind: TileKind,
    pub orient: Orientation,
}
impl Tile {
    #[allow(non_snake_case)]
    pub fn rotateC(&mut self) {
        use Orientation::*;
        self.orient = match self.orient {
            Up =>    Right,
            Down =>  Left,
            Left =>  Up,
            Right => Down,
        }
    }
}

pub struct Tyler {
    pub fps_counter: FPScounter,
    pub tiles: [[Tile ; NY_TILES]; NX_TILES],
    pub dimensions: [u32; 2],
    pub view: View,
}
impl Tyler {
    pub fn new() -> Tyler {
        use Orientation::*;
        use TileKind::*;
        let mut tiles = [[Tile { kind: Wood, orient: Down, }; NY_TILES]; NX_TILES];

        // T
        for x in 2..=4 { tiles[x][2] = Tile { kind: Metal, orient: Up }; }
        for y in 3..=6 { tiles[3][y] = Tile { kind: Metal, orient: Up }; }
        // H
        for y in 2..=6 { tiles[6][y] = Tile { kind: Metal, orient: Up }; }
        for y in 2..=6 { tiles[8][y] = Tile { kind: Metal, orient: Up }; }
        tiles[7][3] = Tile { kind: Metal, orient: Up };
        // E
        for y in 2..=6 { tiles[10][y] = Tile { kind: Metal, orient: Up }; }
        tiles[11][2] = Tile { kind: Metal, orient: Up };
        tiles[11][4] = Tile { kind: Metal, orient: Up };
        tiles[11][6] = Tile { kind: Metal, orient: Up };

        // R
        for y in 2..=6 { tiles[14][y] = Tile { kind: RedStone, orient: Up }; }
        tiles[15][2] = Tile { kind: RedStone, orient: Up };
        tiles[16][3] = Tile { kind: RedStone, orient: Up };
        tiles[15][4] = Tile { kind: RedStone, orient: Up };
        tiles[16][5] = Tile { kind: RedStone, orient: Up };
        tiles[16][6] = Tile { kind: RedStone, orient: Up };
        // E
        for y in 2..=6 { tiles[18][y] = Tile { kind: RedStone, orient: Up }; }
        tiles[19][2] = Tile { kind: RedStone, orient: Up };
        tiles[19][4] = Tile { kind: RedStone, orient: Up };
        tiles[19][6] = Tile { kind: RedStone, orient: Up };
        // D
        for y in 2..=6 { tiles[21][y] = Tile { kind: RedStone, orient: Up }; }
        tiles[22][2] = Tile { kind: RedStone, orient: Up };
        tiles[22][6] = Tile { kind: RedStone, orient: Up };
        for y in 3..=5 { tiles[23][y] = Tile { kind: RedStone, orient: Up }; }


        let dimensions = [0, 0];
        let view = View {x:-0.8, y:-0.5, scale: 0.2};
        Tyler { fps_counter: FPScounter::start(), tiles, dimensions, view }
    }

    pub fn generate_render_data(&self) -> (PushConstantData, Vec<InstanceData>) {

        let win_ratio: (f32, f32) = (self.dimensions[0] as f32 / WIN_SIZE as f32, self.dimensions[1] as f32 / WIN_SIZE as f32);

        let mut instance_data = Vec::new();
        for (x, tile_row) in self.tiles.iter().enumerate() {
            for (y, tile) in tile_row.iter().enumerate() {
                //println!("({}, {}): {:?}", x, y, tile);


                //[[angle.cos(), -angle.sin()],
                // [angle.sin(), angle.cos()]]
                let rot = match tile.orient {
                    Orientation::Up =>       [[ 1.0, -0.0],
                                              [ 0.0,  1.0]],
                    Orientation::Down => [[-1.0, -0.0],
                                          [ 0.0, -1.0]],
                    Orientation::Left =>     [[ 0.0,  1.0],
                                              [-1.0,  0.0]],
                    Orientation::Right => [[ 0.0, -1.0],
                                           [ 1.0,  0.0]],
                };

                let position_offset = [
                    (x as f32 * self.view.scale + self.view.x) / win_ratio.0 * 2.0,
                    (y as f32 * self.view.scale + self.view.y) / win_ratio.1 * 2.0,
                ];

                let tex_shift = match tile.kind {
                    TileKind::Wood => 0,
                    TileKind::Stone => 1,
                    TileKind::Metal => 2,
                    TileKind::RedStone => 3,
                };

                instance_data.push(InstanceData {
                    position_offset,
                    rot,
                    tex_shift,
                });
            }
        }
        //println!();

        let push_constants = PushConstantData {
            win_ratio: [win_ratio.0, win_ratio.1],
            scale: self.view.scale,
        };

        (push_constants, instance_data)
    }

    pub fn key_handler(&mut self, key: u32) {
        match key {
            19 => {
                for x in 0..self.tiles.len() {
                    for y in 0..self.tiles[0].len() {
                        self.tiles[x][y].rotateC()
                    }
                }
            }, // Rotate
            106 => self.view.x -= 0.1, // Left
            105 => self.view.x += 0.1, // Right
            108 => self.view.y -= 0.1, // Up
            103 => self.view.y += 0.1, // Down
            16 => {
                self.view.scale *= 1.1;
                self.view.x *= 1.1;
                self.view.y *= 1.1;
            },
            18 => {
                self.view.scale *= 0.9;
                self.view.x *= 0.9;
                self.view.y *= 0.9;
            },
            _ => (),
        }
    }
}
