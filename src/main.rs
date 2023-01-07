use std::time::Instant;

use macroquad::prelude::*;
mod map;

const TILE_SIZE: f32 = 16.0;

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
    let mut history_index = 0;
    let assets = vec![
        load_texture("assets/0.png").await.unwrap(),
        load_texture("assets/1.png").await.unwrap(),
    ];

    let mut map = map::Map::new(60);
    let mut map_image = image::io::Reader::open("maps/test.png")
        .expect("Failed to open map image!")
        .decode()
        .expect("Failed to decode map image!");

    map.load_tiles(&mut map_image, 3);
    map.generate_map();

    let mut timer = Instant::now();

    loop {
        clear_background(DARKGRAY);

        if is_key_pressed(KeyCode::Escape) {
            return;
        }
        if is_key_down(KeyCode::Space)
            && history_index < map.history.len() - 1
            && timer.elapsed().as_secs_f32() > 0.1
        {
            history_index += 1;
            timer = Instant::now();
        }

        let half_size = (map.size as i32) / 2;
        let mut x: i32 = -half_size;
        let mut y: i32 = -half_size;

        for tile in map.history[history_index].grid.iter() {
            let (nx, ny) = get_xy(x, y);

            if let Some(tile) = tile {
                let rotation = (tile.rotation.clone() as u8) as f32 * std::f32::consts::FRAC_PI_2;
                draw_block(assets[tile.index], nx, ny, rotation);
            } else {
                draw_rectangle(nx, ny, TILE_SIZE, TILE_SIZE, LIGHTGRAY);
            }

            x += 1;
            if x >= half_size {
                x = -half_size;
                y += 1;
            }
        }

        next_frame().await
    }
}

fn get_xy(x: i32, y: i32) -> (f32, f32) {
    let size = TILE_SIZE + 1.0;
    let x = (screen_width() / 2.0 - size / 2.0) + (x as f32 * size);
    let y = (screen_height() / 2.0 - size / 2.0) + (y as f32 * size);
    (x, y)
}

fn draw_block(texture: Texture2D, x: f32, y: f32, rotation: f32) {
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
