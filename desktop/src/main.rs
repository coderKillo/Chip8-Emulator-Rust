use chip8_core::*;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::env;
use std::fs::File;
use std::io::Read;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const TICKS_PER_FRAME: u32 = 10;

fn draw_screen(emulator: &Emulator, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    canvas.set_draw_color(Color::WHITE);

    for (idx, pixel) in emulator.get_display().iter().enumerate() {
        if !pixel {
            continue;
        }

        let x: i32 = (idx % SCREEN_WIDTH) as i32;
        let y: i32 = (idx / SCREEN_WIDTH) as i32;
        let rect = Rect::new(x * SCALE as i32, y * SCALE as i32, SCALE, SCALE);

        canvas.fill_rect(rect).unwrap();
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        print!("Usage: cargo run path/to/game");
        return;
    }

    let sdl_context = sdl2::init().unwrap();
    let video_system = sdl_context.video().unwrap();
    let window = video_system
        .window("Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.clear();
    canvas.present();

    let mut chip8 = Emulator::new();

    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();
    _ = rom.read_to_end(&mut buffer).unwrap();

    chip8.load(&buffer);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'gameloop;
                }
                _ => {}
            }
        }

        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }

        draw_screen(&chip8, &mut canvas);
    }
}
