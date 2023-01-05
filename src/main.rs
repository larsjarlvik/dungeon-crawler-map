use macroquad::prelude::*;
use std::f32::consts;

const ROT_0: f32 = 0.0;
const ROT_1: f32 = consts::FRAC_PI_2;
const ROT_2: f32 = consts::PI;
const ROT_3: f32 = consts::PI + consts::FRAC_PI_2;

#[macroquad::main("Dungeon Crawler Map Generator")]
async fn main() {
    let corner: Texture2D = load_texture("assets/corner.png").await.unwrap();
    let corner_outer: Texture2D = load_texture("assets/corner_outer.png").await.unwrap();
    let wall: Texture2D = load_texture("assets/wall.png").await.unwrap();

    loop {
        clear_background(BLACK);
        draw_block(corner, -1, 0, ROT_0);
        draw_block(corner_outer, 0, 0, ROT_0);
        draw_block(wall, 0, -1, ROT_0);
        draw_block(corner, 0, -2, ROT_0);
        draw_block(corner, 1, -2, ROT_1);
        draw_block(wall, 1, -1, ROT_2);
        draw_block(wall, 1, 0, ROT_2);
        draw_block(corner, 1, 1, ROT_2);
        draw_block(wall, 0, 1, ROT_3);
        draw_block(corner, -1, 1, ROT_3);

        if is_key_down(KeyCode::Escape) {
            return;
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
