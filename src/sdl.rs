use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use super::snake::{Direct, SnakeEvent};

static mut CANVAS: Option<Canvas<Window>> = None;
static mut EVENT_PUMP: Option<EventPump> = None;

pub fn platform_fill_rect(x: i32, y: i32, width: u32, hight: u32, color: Color) {
    let canvas = unsafe { CANVAS.as_mut().expect("canvas need init first") };
    canvas.set_draw_color(color);
    let rect = sdl2::rect::Rect::new(x as i32, y as i32, width, hight);
    canvas.fill_rect(rect).unwrap();
    // canvas.clear();
}

pub fn platform_present() {
    let canvas = unsafe { CANVAS.as_mut().expect("canvas need init first") };
    canvas.present();
}

pub fn init_sdl(window_width: u32, window_hight: u32) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("snake game - SDL Native", window_width, window_hight)
        .position_centered()
        .build()
        .unwrap();
    unsafe {
        CANVAS = Some(window.into_canvas().build().unwrap());
        EVENT_PUMP = Some(sdl_context.event_pump().unwrap());
    }
}

pub fn sdl_event() -> Option<SnakeEvent> {
    let event_pump = unsafe { EVENT_PUMP.as_mut().unwrap() };
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                return Some(SnakeEvent::Quit);
            }
            Event::KeyDown {
                keycode: Some(code),
                ..
            } => match code {
                Keycode::Up => return Some(SnakeEvent::Move(Direct::UP)),
                Keycode::Down => return Some(SnakeEvent::Move(Direct::DOWN)),
                Keycode::Left => return Some(SnakeEvent::Move(Direct::LEFT)),
                Keycode::Right => return Some(SnakeEvent::Move(Direct::RIGHT)),
                _ => {
                    println!("key {code:?} down");
                }
            },
            _ => {}
        }
    }
    None
}
