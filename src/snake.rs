// extern "C" {
//     fn platform_fill_rect(x: i32, y: i32, hight: u32, width: u32);
// }
use crate::sdl;
use sdl2::pixels::Color;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HIGHT: u32 = 600;
const CELL_WIDTH: u32 = 50;
const CELL_HIGHT: u32 = 50;

const SNAKE_MOVE_STEP_FAR: i32 = 10;
// time unit ms
const SNAKE_MOVE_STEP_TIME: u64 = 100;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direct {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone)]
struct Cell {
    // x by pixel
    x: i32,
    // y by pixel
    y: i32,
    // dir
    dir: Option<Direct>,
}

impl Cell {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y, dir: None }
    }
    fn new_rect(x: u32, y: u32, dir: Option<Direct>) -> Self {
        Self {
            x: (x * CELL_WIDTH) as i32,
            y: (y * CELL_HIGHT) as i32,
            dir,
        }
    }

    fn die(&self) -> bool {
        self.x < 0
            || self.y < 0
            || self.x > (WINDOW_WIDTH - CELL_WIDTH) as i32
            || self.y > (WINDOW_HIGHT - CELL_HIGHT) as i32
    }

    fn move_to(&mut self, step: i32) {
        match self.dir.unwrap() {
            Direct::DOWN => {
                self.y += step;
            }
            Direct::UP => {
                self.y -= step;
            }
            Direct::LEFT => {
                self.x -= step;
            }
            Direct::RIGHT => {
                self.x += step;
            }
        }
    }
}

#[derive(Debug)]
struct DirEvent {
    cell: Cell,
    dir: Direct,
    cnt: usize,
}

#[derive(Debug, PartialEq)]
enum SnakeStatus {
    Running,
    Pause,
    Dead,
}

struct Snake {
    // last cell is snake head
    body: Vec<Cell>,
    dir_events: Vec<DirEvent>,
    status: SnakeStatus,
}

#[derive(Debug)]
enum SnakeError {
    OutBoard,
    NotRunning,
}

pub enum SnakeEvent {
    Move(Direct),
    Quit,
}

impl Snake {
    fn new(body: Vec<Cell>, directs: Vec<DirEvent>) -> Self {
        Self {
            body,
            dir_events: directs,
            status: SnakeStatus::Running,
        }
    }

    fn head(&self) -> &Cell {
        self.body.last().unwrap()
    }

    // snake can't go opposite direction directly
    fn dir_event(&mut self, dir: Direct) {
        let mut x = self.head().x;
        let mut y = self.head().y;
        match (self.head().dir.unwrap(), dir) {
            (Direct::UP, Direct::LEFT | Direct::RIGHT) => {
                y = y - (y % CELL_HIGHT as i32);
            }
            (Direct::DOWN, Direct::LEFT | Direct::RIGHT) => {
                y = y + (CELL_HIGHT as i32 - y % CELL_HIGHT as i32);
            }
            (Direct::LEFT, Direct::UP | Direct::DOWN) => {
                x = x - (x % CELL_WIDTH as i32);
            }
            (Direct::RIGHT, Direct::UP | Direct::DOWN) => {
                x = x + (CELL_WIDTH as i32 - x % CELL_WIDTH as i32);
            }
            _ => {
                return;
            }
        }
        let e = DirEvent {
            dir,
            cnt: self.body.len(),
            cell: Cell::new(x, y),
        };
        println!("push event: {e:?}");
        self.dir_events.push(e);
    }

    fn move_ahead(&mut self) -> Result<(), SnakeError> {
        if self.status != SnakeStatus::Running {
            return Err(SnakeError::NotRunning);
        }
        for (_, cell) in self.body.iter_mut().rev().enumerate() {
            for dir in &mut self.dir_events {
                if cell.x == dir.cell.x && cell.y == dir.cell.y {
                    cell.dir = Some(dir.dir);
                    dir.cnt -= 1;
                    break;
                }
            }
            let step = SNAKE_MOVE_STEP_FAR;
            cell.move_to(step);
        }
        if self.die() {
            return Err(SnakeError::OutBoard);
        }
        self.dir_events.retain(|v| v.cnt > 0);
        if self.dir_events.is_empty() {
            println!("directs retain to 0");
        }

        println!("snake move to {:?}", self.head());
        draw_snake(self);
        Ok(())
    }

    // TODO: deal with snake circle
    fn die(&self) -> bool {
        self.head().die()
    }
}

pub fn start() {
    sdl::sdl_init(WINDOW_WIDTH, WINDOW_HIGHT).unwrap();
    let dir = Some(Direct::DOWN);
    let snake_zero = Snake::new(
        vec![
            Cell::new_rect(4, 2, dir),
            Cell::new_rect(4, 3, dir),
            Cell::new_rect(4, 4, dir),
            Cell::new_rect(4, 5, dir),
        ],
        vec![],
    );
    move_snake(snake_zero);
}

fn draw_board() {
    let light_gray: Color = Color::RGB(200, 200, 169);
    let deep_gray: Color = Color::RGB(131, 175, 155);
    let mut color1;
    let mut color2;

    for x in 0..=WINDOW_WIDTH / CELL_WIDTH {
        if x % 2 == 0 {
            color1 = light_gray;
            color2 = deep_gray;
        } else {
            color1 = deep_gray;
            color2 = light_gray;
        }
        for y in 0..=WINDOW_HIGHT / CELL_HIGHT {
            let color = if y % 2 == 0 { color1 } else { color2 };
            let cell = Cell::new_rect(x, y, None);
            sdl::platform_fill_rect(cell.x, cell.y, CELL_WIDTH, CELL_HIGHT, color);
        }
    }
}

fn move_snake(mut snake: Snake) {
    loop {
        if let Some(event) = sdl::sdl_event() {
            match event {
                SnakeEvent::Quit => {
                    println!("quit snake game, Bye!");
                    break;
                }
                SnakeEvent::Move(dir) => {
                    snake.dir_event(dir);
                }
            }
        }
        if let Err(e) = snake.move_ahead() {
            println!("snake error:{e:?}");
            match e {
                SnakeError::OutBoard => {
                    sdl::sdl_text_render(WINDOW_WIDTH, WINDOW_HIGHT, "Pacifico.ttf", "Game Over!")
                        .unwrap();
                    snake.status = SnakeStatus::Dead;
                }
                SnakeError::NotRunning => {
                    println!("snake not running...");
                }
            }
        } else {
            std::thread::sleep(std::time::Duration::from_millis(SNAKE_MOVE_STEP_TIME));
        }
    }
}

const SNAKE_COLOR_BODY: Color = Color::RGB(252, 157, 154);
const SNAKE_COLOR_HEAD: Color = Color::RGB(254, 67, 101);
fn draw_snake(snake: &Snake) {
    draw_board();
    for (i, cell) in snake.body.iter().enumerate() {
        let mut color = SNAKE_COLOR_BODY;
        if i == snake.body.len() - 1 {
            color = SNAKE_COLOR_HEAD;
        }
        sdl::platform_fill_rect(cell.x, cell.y, CELL_WIDTH, CELL_HIGHT, color);
    }
    sdl::platform_present();
}
