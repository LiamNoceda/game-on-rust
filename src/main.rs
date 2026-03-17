use macroquad::prelude::*;

#[macroquad::main("My Game")]
async fn main() {
    let mut x = 50.0;
    let mut y = 0.0;
    let mut vel_y = 0.0;
    let gravity = 0.5;
    let jump_strength = -10.0;
    let speed = 5.0;

    loop {
        clear_background(WHITE);

        if is_key_down(KeyCode::Right) { x += speed; }
        if is_key_down(KeyCode::Left) { x -= speed; }

        if is_key_pressed(KeyCode::Space) && y >= 0.0 {
            vel_y = jump_strength;
        }

        vel_y += gravity;
        y += vel_y;

        if y > screen_height() - 40.0 {
            y = screen_height() - 40.0;
            vel_y = 0.0;
        }

        draw_rectangle(0.0, screen_height() - 20.0, screen_width(), 20.0, DARKGRAY);

        draw_rectangle(x, y, 40.0, 40.0, RED);

        next_frame().await;
    }
}
