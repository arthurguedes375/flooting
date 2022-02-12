use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;

use rand::Rng;

use crate::time;
use crate::settings;
use crate::helper::{Position};
use crate::rectangle::{Rectangle, Size, RectangleSize};
use crate::ui;
use ui::Ui;

pub struct ShootingInfo {
    pub last_shot_time: u128,
    pub next_shots_count: u8,
    pub shots_per_second: u16,
}

#[derive(Clone, Debug)]
pub struct Asteroid {
    pub position: Position,
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

pub struct Game {
    pub ui: Ui,
    pub spaceship: Spaceship,
    pub asteroids: Vec<Asteroid>,
    pub missiles: Vec<Missile>,
    pub shooting: ShootingInfo,
    pub state: State,
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
            shooting: ShootingInfo {
                last_shot_time: time::now() - Game::nanoseconds_shot_delay(settings::SHOTS_PER_SECOND),
                next_shots_count: 0,
                shots_per_second: settings::SHOTS_PER_SECOND,
            },
            asteroids: Game::initialize_asteroids(),
            missiles: vec![],
            state: State::Running,
        };
    }

    fn nanoseconds_shot_delay(shots_per_second: u16) -> u128 {
        return shots_per_second as u128 / 1_000_000_000;
    }

    fn find_furthest_asteroid_position(asteroids: Vec<Asteroid>) -> Position {
        let mut furthest_position: Position = Position {
            x: -1,
            y: -1
        };

        for asteroid in asteroids.iter() {
            if asteroid.position.x > furthest_position.x {
                furthest_position = asteroid.position;
            }
        }

        return furthest_position;
    }

    fn find_asteroids_within_x_range(asteroids: Vec<Asteroid>, min_x: u32, max_x: u32) -> Vec<Position> {
        let mut asteroids_positions: Vec<Position> = vec![];
        let size = RectangleSize {
            width: max_x - min_x,
            height: settings::WINDOW_HEIGHT,
        };

        let outside_rectangle = Rectangle {
            position: Position {
                y: settings::WINDOW_HEIGHT as i32 / 2,
                x: min_x as i32 + size.width as i32 / 2,
            },
            size: Size::Rectangle(size),
        };

        for asteroid in asteroids.iter() {
            let inside_rectangle = Rectangle {
                size: Size::Square(Ui::to_pixels(asteroid.size as u32)),
                position: asteroid.position,
            };
            
            if inside_rectangle.over(outside_rectangle.clone()) {
                asteroids_positions.push(asteroid.position);
            }
        }

        return asteroids_positions;
    }

    fn generate_asteroid(
        rng: &mut rand::prelude::ThreadRng,
        existing_asteroids: Vec<Asteroid>
    ) -> Asteroid {
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
                y: rng.gen_range(
                    settings::ASTEROIDS_MARGIN.y
                    ..
                    (settings::WINDOW_HEIGHT - settings::SPACESHIP_HEIGHT / 2) as i32
                ),
            };

            if settings::ALLOW_INSIDE_GENERATION {
                break 'generation_loop;
            }
    
            let mut inside = false;
            for existing_asteroid in existing_asteroids.iter() {
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
            if !inside {
                break 'generation_loop;
            }
        }

        let generated_asteroid = Asteroid {
            position: asteroid_position,
            size,
            speed: rng.gen_range(
                settings::MIN_ASTEROIDS_SPEED..settings::MAX_ASTEROIDS_SPEED
            )
        };

        return generated_asteroid;
    }

    fn initialize_asteroids() -> Vec<Asteroid> {
        let mut asteroids = vec![];
        let mut rng = rand::thread_rng();

        for _ in 0..rng.gen_range(settings::MIN_GENERATED_ASTEROIDS..settings::MAX_GENERATED_ASTEROIDS) {
            let generated_asteroid = Game::generate_asteroid(&mut rng, vec![]);
            asteroids.push(generated_asteroid);
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

        for asteroid in self.asteroids.iter() {
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
                    println!("Generating asteroid");
                    appearing_asteroids += 1;
                }
            }
        }

        let mut rng = rand::thread_rng();
        for _ in 0..appearing_asteroids {
            let generated_asteroid = Game::generate_asteroid(&mut rng, self.asteroids.clone());
            self.asteroids.push(generated_asteroid);
        }
    }

    fn update_asteroids_positions(&mut self) {
        self.asteroids = self.asteroids.iter().map(|asteroid| {
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
        }).collect();
    }

    fn clear_asteroids(&mut self) {
        self.asteroids = self.asteroids.iter().cloned().filter(|asteroid| asteroid.position.x > 0).collect();
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
                    println!("Shooting");
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    self.spaceship.shooting = false;
                    println!("Stopped Shooting");
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