use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::{event::Event, render::TextureQuery};

use super::snake::{Direct, SnakeEvent};

static mut CANVAS: Option<Canvas<Window>> = None;
static mut EVENT_PUMP: Option<EventPump> = None;
static mut TTF_CONTEXT: Option<Sdl2TtfContext> = None;

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

pub fn sdl_init(window_width: u32, window_hight: u32) -> Result<(), String> {
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
        TTF_CONTEXT = Some(sdl2::ttf::init().map_err(|e| e.to_string())?);
    }
    Ok(())
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

pub fn sdl_text_render(
    window_width: u32,
    window_hight: u32,
    font_path: &str,
    text: &str,
) -> Result<(), String> {
    let canvas = unsafe { CANVAS.as_mut().unwrap() };
    let mut font = unsafe { TTF_CONTEXT.as_ref().unwrap().load_font(font_path, 64)? };
    font.set_style(sdl2::ttf::FontStyle::BOLD);
    let surface = font
        .render(text)
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let TextureQuery { width, height, .. } = texture.query();
    let target = sdl2::rect::Rect::new(
        window_width as i32 / 4,
        window_hight as i32 / 4,
        width,
        height,
    );
    canvas.copy(&texture, None, Some(target))?;
    platform_present();
    Ok(())
}
