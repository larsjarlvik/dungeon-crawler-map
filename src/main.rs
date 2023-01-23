use ::rand::thread_rng;
use macroquad::prelude::*;
use std::{fs, time::Instant};
mod map;

const DISPLAY_SIZE: f32 = 64.0;
const TILE_SIZE: f32 = 5.0;

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
    let map_name = "dungeon";
    let variants = vec![
        map::Variants {
            index: 0,
            weight: 0.0,
            ..Default::default()
        },
        map::Variants {
            index: 1,
            entrance: true,
            exit: true,
            ..Default::default()
        },
        map::Variants {
            index: 2,
            weight: 2.0,
            ..Default::default()
        },
        map::Variants {
            index: 3,
            weight: 2.0,
            ..Default::default()
        },
        map::Variants {
            index: 4,
            weight: 2.0,
            ..Default::default()
        },
    ];

    let image = image::io::Reader::open(format!("maps/{map_name}/map.png"))
        .ok()
        .map(|image| (image.decode().expect("Failed to decode map image!"), TILE_SIZE as u32));

    let config = map::Config { image, variants };
    let mut rng = thread_rng();
    let mut map = map::Map::new(12, 20..40);
    map.build(&mut rng, &config, false);

    let mut asset_paths: Vec<_> = fs::read_dir(format!("maps/{}/tiles", map_name).as_str())
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    asset_paths.sort_by_key(|dir| {
        String::from(dir.path().file_stem().unwrap().to_str().unwrap())
            .parse::<i32>()
            .unwrap()
    });

    let mut assets = vec![];
    for path in asset_paths {
        let texture = load_texture(path.path().as_os_str().to_str().unwrap()).await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        assets.push(texture);
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

        let half_size = (map.history[history_index].size as i32) / 2;
        let mut x = -half_size;
        let mut y = -half_size;

        for tile in map.history[history_index].tiles.iter() {
            let (nx, ny) = get_xy(x, y);

            if let Some(tile) = tile {
                let rotation = (tile.direction.clone() as u8) as f32 * std::f32::consts::FRAC_PI_2;
                draw_tile(&assets, tile, nx, ny, rotation);
            } else {
                draw_rectangle(nx, ny, DISPLAY_SIZE, DISPLAY_SIZE, BLACK);
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
    let size = DISPLAY_SIZE;
    let x = (screen_width() / 2.0 - size / 2.0) + (x as f32 * size);
    let y = (screen_height() / 2.0 - size / 2.0) + (y as f32 * size);
    (x, y)
}

fn draw_tile(assets: &[Texture2D], tile: &map::Tile, x: f32, y: f32, rotation: f32) {
    let texture = assets[tile.asset];
    let h_tile_size = DISPLAY_SIZE / 2.0;

    draw_texture_ex(
        texture,
        x,
        y,
        DARKGRAY,
        DrawTextureParams {
            rotation,
            dest_size: Some(vec2(DISPLAY_SIZE, DISPLAY_SIZE)),
            ..Default::default()
        },
    );

    match tile.path {
        map::Path::Entrance => draw_circle(x + h_tile_size, y + h_tile_size, 4.0, GREEN),
        map::Path::Track => draw_circle(x + h_tile_size, y + h_tile_size, 2.0, BLUE),
        map::Path::Exit => draw_circle(x + h_tile_size, y + h_tile_size, 4.0, RED),
        _ => {}
    }

    let x = x - 1.0;
    let y = y - 1.0;
    let n = DISPLAY_SIZE / (TILE_SIZE - 1.0);
    for (i, _) in tile.edges.north.iter().enumerate().filter(|e| e.1 > &0).map(|(i, e)| (i as f32, e)) {
        draw_rectangle(x + i * n, y, 2.0, 2.0, ORANGE);
    }
    for (i, _) in tile.edges.east.iter().enumerate().filter(|e| e.1 > &0).map(|(i, e)| (i as f32, e)) {
        draw_rectangle(x + DISPLAY_SIZE, y + i * n, 2.0, 2.0, ORANGE);
    }
    for (i, _) in tile.edges.south.iter().enumerate().filter(|e| e.1 > &0).map(|(i, e)| (i as f32, e)) {
        draw_rectangle(x + i * n, y + DISPLAY_SIZE, 2.0, 2.0, ORANGE);
    }
    for (i, _) in tile.edges.west.iter().enumerate().filter(|e| e.1 > &0).map(|(i, e)| (i as f32, e)) {
        draw_rectangle(x, y + i * n, 2.0, 2.0, ORANGE);
    }
}
