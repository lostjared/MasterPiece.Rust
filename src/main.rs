/*
  MasterPiece - Rust Edition
  GPL v3
*/

mod puzzle;
mod scores;

use puzzle::game;
use puzzle::game::Grid;
use rand::Rng;
use scores::high_scores::ScoreMenu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq)]
enum Screen {
    Intro,
    Start,
    Game,
    GameOver,
    Options,
    Credits,
}

fn printtext(
    can: &mut sdl2::render::Canvas<sdl2::video::Window>,
    tex: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font: &sdl2::ttf::Font,
    x: i32,
    y: i32,
    color: sdl2::pixels::Color,
    text: &str,
) {
    let text_surf = font.render(text).blended(color).unwrap();
    let text_surf_tex = tex.create_texture_from_surface(&text_surf).unwrap();
    let TextureQuery {
        width: wi,
        height: hi,
        ..
    } = text_surf_tex.query();
    can.copy(
        &text_surf_tex,
        Some(Rect::new(0, 0, wi, hi)),
        Some(Rect::new(x, y, wi, hi)),
    )
    .expect("on font copy");
}

fn load_gfx(
    texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) -> Vec<sdl2::render::Texture> {
    let mut images = Vec::new();
    let image_strings = vec![
        "./img/intro.bmp",
        "./img/start.bmp",
        "./img/gamebg.bmp",
        "./img/cursor.bmp",
        "./img/logo.bmp",
    ];
    for i in &image_strings {
        let mut surf = sdl2::surface::Surface::load_bmp(&i).unwrap();
        surf.set_color_key(true, Color::RGB(255, 0, 255))
            .expect("on colorkey");
        images.push(texture_creator.create_texture_from_surface(surf).unwrap());
    }
    images
}

fn load_blocks(
    texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) -> Vec<sdl2::render::Texture> {
    let mut images = Vec::new();
    let image_strings = vec![
        "./img/block_black.bmp",
        "./img/block_clear.bmp",
        "./img/block_dblue.bmp",
        "./img/block_gray.bmp",
        "./img/block_green.bmp",
        "./img/block_ltblue.bmp",
        "./img/block_orange.bmp",
        "./img/block_pink.bmp",
        "./img/block_purple.bmp",
        "./img/block_red.bmp",
        "./img/block_yellow.bmp",
    ];
    for i in &image_strings {
        images.push(
            texture_creator
                .create_texture_from_surface(sdl2::surface::Surface::load_bmp(&i).unwrap())
                .unwrap(),
        );
    }
    images
}

fn draw_grid(
    grid: &game::Grid,
    can: &mut sdl2::render::Canvas<sdl2::video::Window>,
    blocks: &[sdl2::render::Texture],
) {
    let offset_x = 185;
    let offset_y = 95;

    for x in 0..grid.get_width() as usize {
        for y in 0..grid.get_height() as usize {
            let color = grid.get_grid_point(x, y);
            if color >= 1 {
                let b = blocks.get(color as usize).unwrap();
                can.copy(
                    b,
                    None,
                    Some(Rect::new(
                        x as i32 * 32 + offset_x,
                        (y as i32 * 16) + offset_y,
                        32,
                        16,
                    )),
                )
                .expect("on copy block");
            } else if color < 0 {
                let mut rng = rand::thread_rng();
                let value: Color = Color::RGB(
                    rng.gen_range(0..255),
                    rng.gen_range(0..255),
                    rng.gen_range(0..255),
                );
                can.set_draw_color(value);
                can.fill_rect(Some(Rect::new(
                    x as i32 * 32 + offset_x,
                    (y as i32 * 16) + offset_y,
                    32,
                    16,
                )))
                .expect("draw rect");
            } else if color == 0 {
                can.set_draw_color(Color::RGB(0, 0, 0));
                can.fill_rect(Some(Rect::new(
                    (x as i32 * 32) + 1 + offset_x,
                    (y as i32 * 16) + offset_y + 1,
                    31,
                    15,
                )))
                .expect("draw rect");
            }
        }
    }
    let block = grid.get_block();
    let b = blocks.get(block[0].color as usize).unwrap();
    can.copy(
        b,
        None,
        Some(Rect::new(
            block[0].x as i32 * 32 + offset_x,
            (block[0].y as i32 * 16) + offset_y,
            32,
            16,
        )),
    )
    .expect("draw rect");
    let b2 = blocks.get(block[1].color as usize).unwrap();
    can.copy(
        b2,
        None,
        Some(Rect::new(
            block[1].x as i32 * 32 + offset_x,
            (block[1].y as i32 * 16) + offset_y,
            32,
            16,
        )),
    )
    .expect("draw rect");
    let b3 = blocks.get(block[2].color as usize).unwrap();
    can.copy(
        b3,
        None,
        Some(Rect::new(
            block[2].x as i32 * 32 + offset_x,
            (block[2].y as i32 * 16) + offset_y,
            32,
            16,
        )),
    )
    .expect("draw rect");
}

fn main() -> std::io::Result<()> {
    let mut width = 1440;
    let mut height = 1080;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        width = args[1].parse().unwrap();
        height = args[2].parse().unwrap();
    }
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("MasterPiece - [ Rust Edition ]", width, height)
        .resizable()
        .opengl()
        .build()
        .unwrap();
    let window_id = window.raw();
    let mut can = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .expect("Error on canvas");
    let texture_creator = can.texture_creator();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let font = ttf_context
        .load_font("./img/arial.ttf", 18)
        .expect("on load font");
    let small_font = ttf_context
        .load_font("./img/arial.ttf", 12)
        .expect("on load font");
    let _text_surf = font
        .render("Hello, World!")
        .blended(Color::RGB(255, 255, 255))
        .unwrap();

    let mut texture = texture_creator
        .create_texture_target(texture_creator.default_pixel_format(), 640, 480)
        .unwrap();

    let images = load_gfx(&texture_creator);
    let blocks = load_blocks(&texture_creator);
    let mut e = sdl.event_pump().unwrap();
    let mut screen = Screen::Intro;
    let mut prev_tick = 0;
    let mut tick_count = 0;
    let mut start_time = 0;
    let mut cursor_pos = 0;
    let mut opt_cursor_pos: usize = 0;
    let mut opt_index: [i32; 3] = [0, 0, 5];
    let mut grid = Grid::new(8, 17);
    grid.diff = 750;
    let mut final_score = 0;
    let mut score_menu = ScoreMenu::new();
    score_menu.load();
    'main: loop {
        for _event in e.poll_iter() {
            match _event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => match screen {
                    Screen::Start => {
                        break 'main;
                    }
                    Screen::Game => {
                        screen = Screen::Start;
                    }
                    Screen::GameOver => {
                        screen = Screen::Start;
                    }
                    Screen::Intro => {
                        screen = Screen::Start;
                    }
                    Screen::Options => {
                        screen = Screen::Start;
                    }
                    Screen::Credits => {
                        screen = Screen::Start;
                    }
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => match screen {
                    Screen::Start => {
                        if cursor_pos > 0 {
                            cursor_pos -= 1;
                        }
                    }
                    Screen::Options => {
                        if opt_cursor_pos > 0 {
                            opt_cursor_pos -= 1;
                        }
                    }
                    Screen::Game => {
                        grid.swap_piece_colors(0);
                    }
                    _ => {}
                },
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    if screen == Screen::Game {
                        grid.shift_left();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    if screen == Screen::Game {
                        grid.shift_right();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => match screen {
                    Screen::Start => {
                        if cursor_pos < 3 {
                            cursor_pos += 1;
                        }
                    }
                    Screen::Options => {
                        if opt_cursor_pos < 2 {
                            opt_cursor_pos += 1;
                        }
                    }
                    Screen::Game => {
                        grid.move_down();
                    }
                    _ => {}
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => match screen {
                    Screen::Options => match opt_cursor_pos {
                        0 => {
                            if opt_index[opt_cursor_pos] > 0 {
                                opt_index[opt_cursor_pos] -= 1;
                            }
                        }
                        1 => {
                            if opt_index[opt_cursor_pos] > 0 {
                                opt_index[opt_cursor_pos] -= 1;
                                unsafe {
                                    let _result = sdl2::sys::SDL_SetWindowFullscreen(window_id, 0);
                                }
                            }
                        }
                        2 => {
                            if opt_index[opt_cursor_pos] > 1 {
                                opt_index[opt_cursor_pos] -= 1;
                            }
                        }
                        _ => {}
                    },
                    Screen::Game => {
                        grid.move_left();
                    }
                    _ => {}
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => match screen {
                    Screen::Options => match opt_cursor_pos {
                        0 => {
                            if opt_index[opt_cursor_pos] < 2 {
                                opt_index[opt_cursor_pos] += 1;
                            }
                        }
                        1 => {
                            if opt_index[opt_cursor_pos] < 1 {
                                opt_index[opt_cursor_pos] += 1;
                                unsafe {
                                    let _result = sdl2::sys::SDL_SetWindowFullscreen(window_id, 1);
                                }                        

                            }
                        }
                        2 => {
                            if opt_index[opt_cursor_pos] < 10 {
                                opt_index[opt_cursor_pos] += 1;
                            }
                        }
                        _ => {}
                    },
                    Screen::Game => {
                        grid.move_right();
                    }
                    _ => {}
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => match screen {
                    Screen::Start => match cursor_pos {
                        0 => {
                            screen = Screen::Game;
                            grid = Grid::new(8, 17);
                            grid.reset_game();
                            match opt_index[0] {
                                0 => {
                                    grid.diff = 1250;
                                }
                                1 => {
                                    grid.diff = 1000;
                                }
                                2 => {
                                    grid.diff = 750;
                                }
                                _ => {}
                            }
                        }
                        1 => {
                            screen = Screen::Options;
                        }
                        2 => {
                            screen = Screen::Credits;
                        }
                        3 => {
                            println!("Exiting...\n");
                            return Ok(());
                        }
                        _ => {}
                    },
                    Screen::Intro => {
                        screen = Screen::Start;
                    }
                    Screen::GameOver => {
                        if !score_menu.input.is_empty() {
                            let s = (String::from(&score_menu.input), final_score);
                            score_menu.scores.push(s);
                            score_menu.sort_scores();
                            score_menu.input = String::new();
                            final_score = 0;
                            score_menu.save();
                        }
                    }
                    _ => {}
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    if screen == Screen::GameOver {
                        score_menu.input.pop();
                    }
                }
                Event::TextInput {
                    timestamp: _,
                    window_id: _,
                    text: s,
                } => {
                    if screen == Screen::GameOver && final_score > 0 {
                        score_menu.type_key(&s);
                    }
                }
                _ => {}
            }
        }
        let start = SystemTime::now();
        let se = start.duration_since(UNIX_EPOCH).expect("error on time");
        let tick = se.as_secs() * 1000 + se.subsec_nanos() as u64 / 1_000_000;
        let ptick = tick - prev_tick;
        prev_tick = tick;
        tick_count += ptick;
        if start_time == 0 {
            start_time = tick_count / 1000;
        }
        grid.increase = opt_index[2];

        match screen {
            Screen::Intro => {
                let _result = can.with_texture_canvas(&mut texture, |texture_canvas| {
                    texture_canvas.clear();
                    texture_canvas
                        .copy(&images[0], None, None)
                        .expect("on copy");
                });
                if tick_count / 1000 > start_time + 3 {
                    screen = Screen::Start;
                }
            }
            Screen::Game => {
                let _result = can.with_texture_canvas(&mut texture, |texture_canvas| {
                    texture_canvas.clear();
                    texture_canvas
                        .copy(&images[2], None, None)
                        .expect("on copy");

                    printtext(
                        texture_canvas,
                        &texture_creator,
                        &small_font,
                        200,
                        60,
                        Color::RGB(255, 255, 255),
                        &format!("Score: {}", grid.score),
                    );
                    printtext(
                        texture_canvas,
                        &texture_creator,
                        &small_font,
                        310,
                        60,
                        Color::RGB(255, 0, 0),
                        &format!("Tabs: {}", grid.lines),
                    );
                    draw_grid(&grid, texture_canvas, &blocks);
                    texture_canvas
                        .copy(
                            &blocks[grid.next_piece[0].color as usize],
                            None,
                            sdl2::rect::Rect::new(510, 200, 32, 16),
                        )
                        .expect("on copy");
                    texture_canvas
                        .copy(
                            &blocks[grid.next_piece[1].color as usize],
                            None,
                            sdl2::rect::Rect::new(510, 200 + 16, 32, 16),
                        )
                        .expect("on copy");
                    texture_canvas
                        .copy(
                            &blocks[grid.next_piece[2].color as usize],
                            None,
                            sdl2::rect::Rect::new(510, 200 + 32, 32, 16),
                        )
                        .expect("on copy");
                });

                if tick_count > grid.diff as u64 {
                    grid.move_down();
                    tick_count = 0;
                }
                grid.proc_blocks();
                if grid.game_over {
                    final_score = grid.score;
                    screen = Screen::GameOver;
                }
            }
            Screen::Options => {
                let _result = can.with_texture_canvas(&mut texture, |texture_canvas| {
                    texture_canvas.clear();
                    texture_canvas
                        .copy(&images[1], None, None)
                        .expect("on copy");
                    texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                    texture_canvas
                        .fill_rect(sdl2::rect::Rect::new(35, 82, 621 - 35, 440 - 70))
                        .expect("on draw rect");
                    printtext(
                        texture_canvas,
                        &texture_creator,
                        &font,
                        50,
                        95,
                        Color::RGB(255, 255, 255),
                        "[ Press Escape to Return ]",
                    );
                    let diff_str =
                        vec!["Difficulty: Easy", "Difficulty: Normal", "Difficulty: Hard"];
                    printtext(
                        texture_canvas,
                        &texture_creator,
                        &small_font,
                        75,
                        150,
                        Color::RGB(0, 0, 255),
                        diff_str[opt_index[0] as usize],
                    );
                    let mut start_cursor: i32 = 150;
                    start_cursor += opt_cursor_pos as i32 * 25;
                    printtext(
                        texture_canvas,
                        &texture_creator,
                        &small_font,
                        55,
                        start_cursor as i32,
                        Color::RGB(255, 0, 0),
                        "=)>",
                    );
                    let full_str = vec!["Fullscreen: False", "Fullscreen: True"];
                    printtext(
                        texture_canvas,
                        &texture_creator,
                        &small_font,
                        75,
                        150 + 25,
                        Color::RGB(0, 0, 255),
                        full_str[opt_index[1] as usize],
                    );
                    let clr_str = format!("Clears until increase: {}", opt_index[2]);
                    printtext(
                        texture_canvas,
                        &texture_creator,
                        &small_font,
                        75,
                        150 + (25 * 2),
                        Color::RGB(0, 0, 255),
                        &clr_str,
                    );
                });
            }
            Screen::Credits => {
                let _result = can.with_texture_canvas(&mut texture, |texture_canvas| {
                    texture_canvas.clear();
                    texture_canvas
                        .copy(&images[1], None, None)
                        .expect("on copy");
                    texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                    texture_canvas
                        .fill_rect(sdl2::rect::Rect::new(35, 82, 621 - 35, 440 - 70))
                        .expect("on draw rect");
                    printtext(
                        texture_canvas,
                        &texture_creator,
                        &font,
                        175,
                        100,
                        Color::RGB(255, 255, 255),
                        "MasterPiece - [ Rust Edition ]",
                    );
                    printtext(
                        texture_canvas,
                        &texture_creator,
                        &font,
                        75,
                        150,
                        Color::RGB(255, 255, 0),
                        "Programmed by Jared Bruni",
                    );
                    printtext(
                        texture_canvas,
                        &texture_creator,
                        &font,
                        75,
                        175,
                        Color::RGB(0, 255, 0),
                        "LostSideDead Software",
                    );
                    printtext(
                        texture_canvas,
                        &texture_creator,
                        &font,
                        75,
                        200,
                        Color::RGB(255, 0, 0),
                        "Open Source, Open Mind",
                    );
                    texture_canvas
                        .copy(
                            &images[4],
                            None,
                            sdl2::rect::Rect::new(640 - 300, 480 - 300, 240, 240),
                        )
                        .expect("on copy");
                });
            }
            Screen::Start => {
                let _result = can.with_texture_canvas(&mut texture, |texture_canvas| {
                    texture_canvas.clear();
                    texture_canvas
                        .copy(&images[1], None, None)
                        .expect("on copy");
                    let cursor_x = 250;
                    let mut cursor_y = 0;
                    match cursor_pos {
                        0 => {
                            cursor_y = 170;
                        }
                        1 => {
                            cursor_y = 170 + 70;
                        }
                        2 => {
                            cursor_y = 170 + 70 * 2;
                        }
                        3 => {
                            cursor_y = 170 + 70 * 3;
                        }
                        _ => {}
                    }
                    texture_canvas
                        .copy(
                            &images[3],
                            None,
                            sdl2::rect::Rect::new(cursor_x, cursor_y, 64, 64),
                        )
                        .expect("on copy");
                });
            }
            Screen::GameOver => {
                let _result = can.with_texture_canvas(&mut texture, |texture_canvas| {
                    texture_canvas.clear();
                    texture_canvas
                        .copy(&images[1], None, None)
                        .expect("on copy");
                    texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                    texture_canvas
                        .fill_rect(sdl2::rect::Rect::new(35, 82, 621 - 35, 440 - 70))
                        .expect("on draw rect");
                    printtext(
                        texture_canvas,
                        &texture_creator,
                        &font,
                        100,
                        100,
                        Color::RGB(255, 255, 255),
                        "MasterPiece - [ Game Over Escape to Return ]",
                    );
                    if final_score > 0 {
                        printtext(
                            texture_canvas,
                            &texture_creator,
                            &small_font,
                            75,
                            150,
                            Color::RGB(0, 255, 0),
                            &format!("Final Score: {}", final_score),
                        );
                    }

                    if final_score > 0 {
                        printtext(
                            texture_canvas,
                            &texture_creator,
                            &small_font,
                            75,
                            200,
                            Color::RGB(255, 255, 255),
                            "Enter your name: ",
                        );
                    }

                    if !score_menu.input.is_empty() {
                        printtext(
                            texture_canvas,
                            &texture_creator,
                            &small_font,
                            75,
                            225,
                            Color::RGB(255, 255, 255),
                            &score_menu.input,
                        );
                    }

                    if final_score == 0 {
                        let mut start_y = 150;
                        for item in &score_menu.scores {
                            printtext(
                                texture_canvas,
                                &texture_creator,
                                &small_font,
                                175,
                                start_y,
                                Color::RGB(255, 0, 0),
                                &format!("{}: {}", item.0, item.1),
                            );
                            start_y += 20;
                        }
                    }
                });
            }
        }
        can.clear();
        can.copy(&texture, None, Some(Rect::new(0, 0, width, height)))
            .expect("on copy");
        can.present();
    }
    Ok(())
}
