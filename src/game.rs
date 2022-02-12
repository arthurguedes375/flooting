use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;

use rand::Rng;

use crate::time;
use crate::settings;
use crate::helper::{Position};
use crate::rectangle::{Rectangle, Size};
use crate::ui;
use ui::Ui;


pub type AsteroidRow = Vec<Asteroid>;
pub type AsteroidRows = Vec<AsteroidRow>;

pub struct ShootingInfo {
    pub last_shot_time: u128,
    pub delay_to_next_shot: u128,
}

#[derive(Clone, Copy, Debug)]
pub struct Asteroid {
    pub position: Position,
    pub row: usize,
    pub size: u8,
    pub speed: u32,
}

#[derive(Clone)]
pub struct Missile {
    pub position: Position,
    pub active: bool,
}

pub struct Spaceship {
    pub position: Position,
    pub life: u8,
    pub shooting: bool,
}

pub enum State {
    Running,
    Paused,
    Closed,
}

pub struct DebugOptions {
    pub generation_line: bool,
    pub rows_starting_line: bool,
}

pub enum Debug {
    Debugging,
    Not,
}

pub struct Game {
    pub ui: Ui,
    pub spaceship: Spaceship,
    pub asteroids: AsteroidRows,
    pub missiles: Vec<Missile>,
    pub shooting_info: ShootingInfo,
    pub state: State,
    pub debug_options: DebugOptions,
    pub debug: Debug,
}

impl Game {
    pub fn new(ui_settings: ui::UiSettings) -> Game {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(&ui_settings.title, ui_settings.width, ui_settings.height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        return Game {
            ui: Ui {
                sdl_context,
                video_subsystem,
                canvas,
                event_pump,
            },
            spaceship: Spaceship {
                position: Position {
                    x: settings::INITIAL_SPACESHIP_POSITION.x,
                    y: settings::INITIAL_SPACESHIP_POSITION.y,
                },
                life: 10,
                shooting: false,
            },
            shooting_info: ShootingInfo {
                last_shot_time: time::now() - Game::nanoseconds_shot_delay(settings::SHOTS_PER_SECOND),
                delay_to_next_shot: Game::nanoseconds_shot_delay(settings::SHOTS_PER_SECOND),
            },
            asteroids: Game::initialize_asteroids(),
            missiles: vec![],
            state: State::Running,
            debug_options: DebugOptions {
                generation_line: true,
                rows_starting_line: true,
            },
            debug: if settings::DEBUG { Debug::Debugging } else { Debug::Not },
        };
    }

    fn map_asteroids(mut asteroids: AsteroidRows, f: fn (row: AsteroidRow) -> Vec<Asteroid>) -> AsteroidRows {
        for (i, row) in asteroids.clone().iter().cloned().enumerate() {
            asteroids[i] = f(row);
        }

        return asteroids;
    }

    pub fn get_row_y_position(row: usize) -> u32 {
        let margin = row * settings::ASTEROIDS_ROWS_MARGIN as usize;
        let y_position = settings::ASTEROIDS_MARGIN.y as usize + margin + (settings::ASTEROIDS_ROWS_HEIGHT as usize * row);
        return y_position.try_into().expect("Failed to get the row's y position.");
    }

    fn nanoseconds_shot_delay(shots_per_second: u16) -> u128 {
        return shots_per_second as u128 / 1_000_000_000;
    }

    fn generate_asteroid(
        rng: &mut rand::prelude::ThreadRng,
        existing_asteroids: AsteroidRows
    ) -> Asteroid {
        let row = rng.gen_range(0..settings::ASTEROIDS_ROWS);
        let mut asteroid_position;
        let mut size;
        'generation_loop: loop {
            size = rng.gen_range(
                settings::MIN_ASTEROIDS_SIZE..settings::MAX_ASTEROIDS_SIZE
            );
    
            let minimum_x_position = (settings::WINDOW_WIDTH + Ui::to_pixels(size as u32)) as i32;
    
            asteroid_position = Position {
                x: rng.gen_range(
                    minimum_x_position
                    ..
                    minimum_x_position + settings::ASTEROIDS_MARGIN.x
                ),
                y: Game::get_row_y_position(row) as i32,
            };

            if settings::ALLOW_INSIDE_GENERATION {
                break 'generation_loop;
            }
    
            let mut inside = false;
            for row in existing_asteroids.iter() {
                for existing_asteroid in row.iter() {
                    let outside_rectangle = Rectangle {
                        position: existing_asteroid.position,
                        size: Size::Square(
                            Ui::to_pixels(existing_asteroid.size as u32)
                        ),
                    };
                    let inside_rectangle = Rectangle {
                        position: asteroid_position,
                        size: Size::Square(Ui::to_pixels(size as u32)),
                    };
        
                    if inside_rectangle.over(outside_rectangle) {
                        inside = true;
                        break; 
                    }
                }
            }
            if !inside {
                break 'generation_loop;
            }
        }

        let generated_asteroid = Asteroid {
            position: asteroid_position,
            size,
            row,
            speed: rng.gen_range(
                settings::MIN_ASTEROIDS_SPEED..settings::MAX_ASTEROIDS_SPEED
            )
        };

        return generated_asteroid;
    }

    fn initialize_asteroids() -> AsteroidRows {
        let mut asteroids = vec![vec![]; settings::ASTEROIDS_ROWS];
        let mut rng = rand::thread_rng();
        for row_i in 0..settings::ASTEROIDS_ROWS {
            for _ in 0..rng.gen_range(settings::MIN_GENERATED_ASTEROIDS..settings::MAX_GENERATED_ASTEROIDS) {
                let generated_asteroid = Game::generate_asteroid(&mut rng, vec![vec![]; settings::ASTEROIDS_ROWS]);
                asteroids[row_i].push(generated_asteroid);
            }
        }

        return asteroids;
    }

    fn sort_missiles(missiles: Vec<Missile>) -> Vec<Missile> {
        let mut inactive_missiles_first: Vec<Missile> = vec![];
        let mut active_missiles: Vec<Missile> = vec![];

        for missile in missiles.clone().iter() {
            if missile.active == true {
                active_missiles.push(missile.clone());
                continue;
            }
            
            inactive_missiles_first.push(missile.clone());
        }

        inactive_missiles_first.extend(active_missiles);
        
        return inactive_missiles_first;
    }

    fn next_position(rectangle_position: Position, speed: u32, direction: Position) -> Position {
        let next_pos = Position {
            x: rectangle_position.x + direction.x * speed as i32,
            y: rectangle_position.y + direction.y * speed as i32,
        };

        return next_pos;
    }

    fn asteroids_generation(&mut self) {
        let mut appearing_asteroids = 0;

        for row in self.asteroids.iter() {
            for asteroid in row.iter() {
                let corners = Rectangle {
                    position: asteroid.position,
                    size: Size::Square(Ui::to_pixels(asteroid.size as u32)),
                }.get_corners();

                if (corners.top_left.x as i64) < settings::WINDOW_WIDTH as i64
                && corners.top_left.x as i64 > settings::WINDOW_WIDTH as i64 - settings::GENERATE_NEW_ASTEROID_AFTER as i64 {
                    let next_position = Game::next_position(
                        asteroid.position,
                        asteroid.speed,
                        Position {
                            x: -1,
                            y: 0,
                        },
                    );
                    let corners = Rectangle {
                        position: next_position,
                        size: Size::Square(Ui::to_pixels(asteroid.size as u32)),
                    }.get_corners();
                    
                    if corners.top_left.x as i64 <= settings::WINDOW_WIDTH as i64 - settings::GENERATE_NEW_ASTEROID_AFTER as i64 {
                        appearing_asteroids += 1;
                    }
                }
            }
        }

        let mut rng = rand::thread_rng();
        for _ in 0..appearing_asteroids {
            let generated_asteroid = Game::generate_asteroid(&mut rng, self.asteroids.clone());
            self.asteroids[generated_asteroid.row].push(generated_asteroid);
        }
    }

    fn update_asteroids_positions(&mut self) {
        self.asteroids = Game::map_asteroids(self.asteroids.clone(), |row| {
            row.iter().map(|asteroid| {
                Asteroid {
                    position: Game::next_position(
                        asteroid.position,
                        asteroid.speed,
                        Position {
                            x: -1,
                            y: 0,
                        }
                    ),
                    ..*asteroid
                }
            }).collect()
        });
    }

    fn clear_asteroids(&mut self) {
        self.asteroids = Game::map_asteroids(self.asteroids.clone(), |row| {
            row.iter().cloned().filter(|asteroid| asteroid.position.x > 0).collect()
        });
    }

    fn update(&mut self) {
        self.missiles = Game::sort_missiles(self.missiles.clone());
        self.asteroids_generation();
        self.update_asteroids_positions();
        self.clear_asteroids();
    }

    fn get_inputs(&mut self) {
        self.spaceship.position.y = self.ui.event_pump.mouse_state().y();

        for event in self.ui.event_pump.poll_iter() {
            match event {
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    self.spaceship.shooting = true;
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    self.spaceship.shooting = false;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    match self.state {
                        State::Running => self.state = State::Paused,
                        State::Paused => self.state = State::Running,

                        _ => {}
                    }
                }

                Event::KeyDown {
                    keycode: Some(Keycode::F5),
                    ..
                } => {
                    match self.debug {
                        Debug::Not => self.debug = Debug::Debugging,
                        Debug::Debugging => self.debug = Debug::Not,
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F6),
                    ..
                } => {
                    self.debug_options = DebugOptions {
                        generation_line: !self.debug_options.generation_line,
                        ..self.debug_options
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F7),
                    ..
                } => {
                    self.debug_options = DebugOptions {
                        rows_starting_line: !self.debug_options.rows_starting_line,
                        ..self.debug_options
                    }
                }

                Event::Quit {..} => {
                    self.state = State::Closed;
                }
                _ => {}
            }
        }
    }

    pub fn delay_fps(fps: u32) {
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / fps));
    }

    fn run(&mut self) {
        self.get_inputs();

        match &self.state {
            State::Running => self.update(),
            _ => {}
        }
    
        Ui::draw(self);
        Game::delay_fps(settings::FPS);
    } 

    pub fn init(&mut self) {
        'main_loop: loop {
            match self.state {
                State::Running | State::Paused => {
                    self.run();
                }
                State::Closed => {
                    break 'main_loop;
                }
            }
        }
    }
}