use ::rand::thread_rng;
use macroquad::prelude::*;
use std::{fs, time::Instant};
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
    let map_name = "custom";
    let variants = vec![
        // Woods
        map::Variants {
            index: 0,
            weight: 1.0,
            neighbors: vec![0, 1],
        },
        // Grass
        map::Variants {
            index: 1,
            weight: 1.0,
            neighbors: vec![0, 1, 2],
        },
        // Sand
        map::Variants {
            index: 2,
            weight: 1.0,
            neighbors: vec![1, 2, 3],
        },
        // Water
        map::Variants {
            index: 3,
            weight: 3.0,
            neighbors: vec![2, 3],
        },
    ];

    // let tile_size = 3;
    // let image = image::io::Reader::open(format!("maps/{map_name}/map.png"))
    //     .ok()
    //     .map(|image| (image.decode().expect("Failed to decode map image!"), tile_size));

    let config = map::Config { image: None, variants };
    let mut rng = thread_rng();
    let mut map = map::Map::new(40);
    map.build(&mut rng, &config, false);

    let asset_paths = fs::read_dir(format!("maps/{}/tiles", map_name).as_str()).unwrap();
    let mut assets = vec![];
    for path in asset_paths {
        assets.push(load_texture(path.unwrap().path().as_os_str().to_str().unwrap()).await.unwrap());
    }

    let mut update_timer = Instant::now();
    let mut is_playing = false;
    let mut history_index = 0;

    loop {
        clear_background(Color::from_rgba(26, 26, 26, 255));

        if is_key_pressed(KeyCode::Escape) {
            return;
        }
        if is_key_pressed(KeyCode::Enter) {
            history_index += 1;
            history_index %= map.history.len();
        }
        if is_key_pressed(KeyCode::Space) {
            is_playing = !is_playing;
        }

        if is_playing && update_timer.elapsed().as_secs_f32() > 0.05 && history_index < map.history.len() - 1 {
            history_index += 1;
            update_timer = Instant::now();
        }

        let half_size = (map.size as i32) / 2;
        let mut x: i32 = -half_size;
        let mut y: i32 = -half_size;

        for tile in map.history[history_index].grid.iter() {
            let (nx, ny) = get_xy(x, y);

            if let Some(tile) = tile {
                let rotation = (tile.direction.clone() as u8) as f32 * std::f32::consts::FRAC_PI_2;
                draw_block(assets[tile.asset], nx, ny, rotation);
            } else {
                draw_rectangle(nx, ny, TILE_SIZE, TILE_SIZE, LIGHTGRAY);
            }

            x += 1;
            if x >= half_size {
                x = -half_size;
                y += 1;
            }
        }

        if is_key_pressed(KeyCode::R) {
            map.build(&mut rng, &config, false);
            history_index = map.history.len() - 1;
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
