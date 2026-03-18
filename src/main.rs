use elegia_core::{Board, PlayerId};
use macroquad::prelude::*;

const BG_COLOR: Color = Color::new(0.75, 0.85, 0.95, 1.0); // bleu pâle
const HEX_FILL: Color = Color::new(0.6, 0.8, 0.6, 1.0); // vert pâle (herbe)
const HEX_BORDER: Color = Color::new(0.2, 0.4, 0.2, 1.0); // vert foncé
const NORTH_ORB: Color = Color::new(0.35, 0.55, 0.95, 1.0);
const SOUTH_ORB: Color = Color::new(0.95, 0.35, 0.35, 1.0);
const NORTH_SPAWN: Color = Color::new(0.35, 0.55, 0.95, 0.3);
const SOUTH_SPAWN: Color = Color::new(0.95, 0.35, 0.35, 0.3);

fn draw_hex_fill(x: f32, y: f32, size: f32, fill_color: Color) {
    draw_poly(x, y, 6, size, 30.0, fill_color);
}

fn draw_hex_border(x: f32, y: f32, size: f32, border_color: Color) {
    let mut points = Vec::new();

    for i in 0..6 {
        let angle = std::f32::consts::PI / 6.0 + std::f32::consts::PI / 3.0 * i as f32;
        let px = x + size * angle.cos();
        let py = y + size * angle.sin();
        points.push(vec2(px, py));
    }

    for i in 0..6 {
        let p1 = points[i];
        let p2 = points[(i + 1) % 6];
        draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, border_color);
    }
}

fn hex_to_screen(q: i32, r: i32, size: f32) -> (f32, f32) {
    let x = size * (3.0_f32).sqrt() * (q as f32 + r as f32 / 2.0);
    let y = size * 1.5 * r as f32;

    (x, y)
}

#[macroquad::main("Elegia")]
async fn main() {
    let board = Board::default();
    let hexes = board.all_hexes();
    let north_orb = board.orb_hex(PlayerId::North);
    let south_orb = board.orb_hex(PlayerId::South);

    let size = 40.0;

    loop {
        clear_background(BG_COLOR);

        let screen_w = screen_width();
        let screen_h = screen_height();

        let origin_x = screen_w / 2.0;
        let origin_y = screen_h / 2.0;

        for hex in &hexes {
            let (dx, dy) = hex_to_screen(hex.q.into(), hex.r.into(), size);
            let x = origin_x + dx;
            let y = origin_y + dy;

            draw_hex_fill(x, y, size, HEX_FILL);

            if board.is_spawn_hex(*hex, PlayerId::North) {
                draw_hex_fill(x, y, size, NORTH_SPAWN);
            }

            if board.is_spawn_hex(*hex, PlayerId::South) {
                draw_hex_fill(x, y, size, SOUTH_SPAWN);
            }

            draw_hex_border(x, y, size, HEX_BORDER);

            let label = format!("{},{}", hex.q, hex.r);
            draw_text(&label, x - size * 0.35, y + 5.0, 16.0, DARKGREEN);

            if *hex == north_orb {
                draw_circle(x, y, size * 0.35, NORTH_ORB);
            }

            if *hex == south_orb {
                draw_circle(x, y, size * 0.35, SOUTH_ORB);
            }
        }

        next_frame().await;
    }
}
