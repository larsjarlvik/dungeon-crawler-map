use builder::{tile::TileType, Map};
use macroquad::prelude::*;
use std::{f32::consts, fs};
mod builder;

const ROT_0: f32 = 0.0;
const ROT_1: f32 = consts::FRAC_PI_2;
const ROT_2: f32 = consts::PI;
const ROT_3: f32 = consts::PI + consts::FRAC_PI_2;

fn window_conf() -> Conf {
    Conf {
        window_title: "Dungeon Crawler Map Generator".to_owned(),
        window_height: 1200,
        window_width: 1200,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let empty: Texture2D = load_texture("assets/empty.png").await.unwrap();
    let floor: Texture2D = load_texture("assets/floor.png").await.unwrap();
    let constraint: Texture2D = load_texture("assets/constraint.png").await.unwrap();
    let corner: Texture2D = load_texture("assets/corner.png").await.unwrap();
    let wall: Texture2D = load_texture("assets/wall.png").await.unwrap();

    let sample = Map::from_string(fs::read_to_string("assets/map.txt").unwrap());
    let history = builder::Map::build(sample);
    let mut index = 0;

    loop {
        clear_background(BLACK);

        let mut x = 0;
        let mut y = 0;

        if is_key_pressed(KeyCode::Escape) {
            return;
        }
        if is_key_pressed(KeyCode::Space) {
            index += 1;
            index %= history.len();
        }

        clear_background(BLACK);

        let map = history[index].clone();
        let w2 = map.width / 2;
        let h2 = map.height / 2;

        for tile in map.tiles.iter() {
            match tile {
                TileType::Empty => draw_block(empty, x - w2, y - h2, ROT_0),
                TileType::Floor => draw_block(floor, x - w2, y - h2, ROT_0),
                TileType::Constraint => draw_block(constraint, x - w2, y - h2, ROT_0),
                TileType::CornerTL => draw_block(corner, x - w2, y - h2, ROT_0),
                TileType::CornerTR => draw_block(corner, x - w2, y - h2, ROT_1),
                TileType::CornerBR => draw_block(corner, x - w2, y - h2, ROT_2),
                TileType::CornerBL => draw_block(corner, x - w2, y - h2, ROT_3),
                TileType::WallL => draw_block(wall, x - w2, y - h2, ROT_0),
                TileType::WallT => draw_block(wall, x - w2, y - h2, ROT_1),
                TileType::WallR => draw_block(wall, x - w2, y - h2, ROT_2),
                TileType::WallB => draw_block(wall, x - w2, y - h2, ROT_3),
            }

            x += 1;
            if x >= map.width {
                x = 0;
                y += 1;
            }
        }

        next_frame().await
    }
}

fn draw_block(texture: Texture2D, x: i32, y: i32, rotation: f32) {
    let x = (screen_width() / 2.0 - 8.0) + (x as f32 * 16.0);
    let y = (screen_height() / 2.0 - 8.0) + (y as f32 * 16.0);

    draw_texture_ex(
        texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            rotation,
            ..Default::default()
        },
    );
}
