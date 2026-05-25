use elegia_core::{Element, GameState, Hex, PlayerId, Unit};
use macroquad::prelude::*;

const BG_COLOR: Color = Color::new(0.75, 0.85, 0.95, 1.0);
const HEX_FILL: Color = Color::new(0.6, 0.8, 0.6, 1.0);
const HEX_BORDER: Color = Color::new(0.2, 0.4, 0.2, 1.0);
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

fn closest_hex_to_point(
    mouse_x: f32,
    mouse_y: f32,
    hexes: &[Hex],
    origin_x: f32,
    origin_y: f32,
    size: f32,
) -> Option<Hex> {
    let mut closest_hex = None;
    let mut closest_distance_sq = f32::MAX;

    for hex in hexes {
        let (dx, dy) = hex_to_screen(hex.q.into(), hex.r.into(), size);
        let center_x = origin_x + dx;
        let center_y = origin_y + dy;

        let dist_x = mouse_x - center_x;
        let dist_y = mouse_y - center_y;
        let distance_sq = dist_x * dist_x + dist_y * dist_y;

        if distance_sq < closest_distance_sq {
            closest_distance_sq = distance_sq;
            closest_hex = Some(*hex);
        }
    }

    closest_hex
}

fn highlight_hex(x: f32, y: f32, size: f32, color: Color) {
    let inner_size = size * 0.85;

    let mut points = Vec::new();

    for i in 0..6 {
        let angle = std::f32::consts::PI / 6.0 + std::f32::consts::PI / 3.0 * i as f32;
        let px = x + inner_size * angle.cos();
        let py = y + inner_size * angle.sin();
        points.push(vec2(px, py));
    }

    for i in 0..6 {
        let p1 = points[i];
        let p2 = points[(i + 1) % 6];
        draw_line(p1.x, p1.y, p2.x, p2.y, 3.0, color);
    }
}

fn draw_button(
    rect: Rect,
    label: &str,
    color: Color,
    enabled: bool,
    mouse: Vec2,
    left_clicked: bool,
) -> bool {
    let hovered = enabled && rect.contains(mouse);

    let bg = if !enabled {
        GRAY
    } else if hovered {
        Color::new(color.r, color.g, color.b, 0.85)
    } else {
        color
    };

    draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, BLACK);

    let text_size = 28.0;
    let text = measure_text(label, None, text_size as u16, 1.0);

    draw_text(
        label,
        rect.x + rect.w / 2.0 - text.width / 2.0,
        rect.y + rect.h / 2.0 + text.height / 2.0,
        text_size,
        BLACK,
    );

    hovered && left_clicked
}

fn draw_unit_placeholder(x: f32, y: f32, size: f32, unit: &Unit, color: Color) {
    fn unit_initials(name: &str) -> String {
        name.split_whitespace()
            .filter_map(|word| word.chars().next())
            .collect()
    }

    let radius = size * 0.75;

    draw_circle(x, y, radius, color);
    draw_circle_lines(x, y, radius, 2.0, BLACK);

    let initials = unit_initials(unit.kind.name);

    let initials_size = 24.0;
    let initials_dim = measure_text(&initials, None, initials_size as u16, 1.0);

    draw_text(
        &initials,
        x - initials_dim.width / 2.0,
        y - 2.0,
        initials_size,
        WHITE,
    );

    let stats = format!(
        "{}/{}/{}",
        unit.current_attack, unit.current_health, unit.current_speed
    );

    let stats_size = 17.0;
    let stats_dim = measure_text(&stats, None, stats_size as u16, 1.0);

    draw_text(
        &stats,
        x - stats_dim.width / 2.0,
        y + radius * 0.55,
        stats_size,
        WHITE,
    );
}

#[macroquad::main("Elegia")]
async fn main() {
    let size = 40.0;
    let mut game = GameState::default();

    let board = game.board;
    let hexes = board.all_hexes();
    let north_orb = board.orb_hex(PlayerId::North);
    let south_orb = board.orb_hex(PlayerId::South);

    let mut selected_hex: Option<Hex> = None;
    let mut selected_roster_index: Option<usize> = None;

    loop {
        clear_background(BG_COLOR);
        let screen_w = screen_width();
        let screen_h = screen_height();

        let (mouse_x, mouse_y) = mouse_position();
        let mouse = vec2(mouse_x, mouse_y);
        let left_clicked = is_mouse_button_pressed(MouseButton::Left);

        let mut ui_clicked = false;

        // Board

        let origin_x = screen_w / 2.0;
        let origin_y = screen_h / 2.0;

        for hex in &hexes {
            let (dx, dy) = hex_to_screen(hex.q.into(), hex.r.into(), size);
            let x = origin_x + dx;
            let y = origin_y + dy;

            draw_hex_fill(x, y, size, HEX_FILL);

            if Some(*hex) == selected_hex {
                highlight_hex(x, y, size, YELLOW);
            }

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

        for unit in game.units_from_player(PlayerId::South) {
            let (dx, dy) = hex_to_screen(unit.position.q.into(), unit.position.r.into(), size);
            let x = origin_x + dx;
            let y = origin_y + dy;

            draw_unit_placeholder(x, y, size, unit, DARKBLUE);
        }

        // Mana buttons and display

        let ui_margin_bottom = 64.0;
        let button_size = 48.0;
        let gap = 12.0;
        let start_x = 24.0;
        let start_y = screen_h - button_size - ui_margin_bottom;
        let mana_enabled = !game.turn().has_increased_mana();
        let pool = game.turn().current_pool();
        let max_pool = game.current_player_max_pool();

        let fire_rect = Rect::new(start_x, start_y, button_size, button_size);
        let water_rect = Rect::new(
            start_x + (button_size + gap),
            start_y,
            button_size,
            button_size,
        );
        let air_rect = Rect::new(
            start_x + 2.0 * (button_size + gap),
            start_y,
            button_size,
            button_size,
        );
        let earth_rect = Rect::new(
            start_x + 3.0 * (button_size + gap),
            start_y,
            button_size,
            button_size,
        );

        if draw_button(fire_rect, "F", ORANGE, mana_enabled, mouse, left_clicked) {
            game.increase_current_player_mana(Element::Fire);
            ui_clicked = true;
        }

        if draw_button(water_rect, "W", BLUE, mana_enabled, mouse, left_clicked) {
            game.increase_current_player_mana(Element::Water);
            ui_clicked = true;
        }

        if draw_button(air_rect, "A", SKYBLUE, mana_enabled, mouse, left_clicked) {
            game.increase_current_player_mana(Element::Air);
            ui_clicked = true;
        }

        if draw_button(
            earth_rect,
            "E",
            DARKGREEN,
            mana_enabled,
            mouse,
            left_clicked,
        ) {
            game.increase_current_player_mana(Element::Earth);
            ui_clicked = true;
        }

        draw_text(
            &format!(
                "F:{}/{}  W:{}/{}  A:{}/{}  E:{}/{}",
                pool.fire,
                max_pool.fire,
                pool.water,
                max_pool.water,
                pool.air,
                max_pool.air,
                pool.earth,
                max_pool.earth
            ),
            start_x,
            start_y + button_size + 28.0,
            24.0,
            BLACK,
        );

        // End turn button

        let end_turn_rect = Rect::new(
            start_x + 4.0 * (button_size + gap) + 24.0,
            start_y,
            140.0,
            button_size,
        );

        if draw_button(
            end_turn_rect,
            "End turn",
            LIGHTGRAY,
            true,
            mouse,
            left_clicked,
        ) {
            game.end_turn();
            game.end_turn();
            ui_clicked = true;
        }

        // Rosters

        let south_roster = game.roster_from_player(PlayerId::South);

        let roster_x = 24.0;
        let roster_y = 140.0;
        let row_h = 32.0;
        let row_w = 240.0;

        let (mouse_x, mouse_y) = mouse_position();
        let mouse = vec2(mouse_x, mouse_y);

        for (index, unit) in south_roster.iter().enumerate() {
            let text_x = roster_x;
            let text_y = roster_y + index as f32 * row_h;

            let rect = Rect::new(roster_x - 4.0, text_y - 24.0, row_w, row_h);

            let hovered = rect.contains(mouse);
            let selected = selected_roster_index == Some(index);

            if selected {
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, YELLOW);
            } else if hovered {
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, LIGHTGRAY);
            }

            if hovered && left_clicked {
                if selected {
                    selected_roster_index = None;
                } else {
                    selected_roster_index = Some(index);
                }

                ui_clicked = true;
            }

            draw_text(unit.name, text_x, text_y, 24.0, BLACK);
        }

        // Hex selection

        if !ui_clicked && left_clicked {
            if let Some(hex) =
                closest_hex_to_point(mouse_x, mouse_y, &hexes, origin_x, origin_y, size)
            {
                if let Some(unit_index) = selected_roster_index {
                    let roster = game.roster_from_player(PlayerId::South);
                    let unit = roster[unit_index];

                    let result = game.spawn_unit(unit, hex);
                    println!("Spawn result: {:?}", result);
                } else {
                    selected_hex = Some(hex);
                    println!("Selected hex: {:?}", hex);
                }
            }
        }

        next_frame().await;
    }
}
