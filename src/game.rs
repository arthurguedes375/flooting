use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;

use rand::Rng;

use crate::time;
use crate::settings;
use crate::helper::{Position, G2UMessage, U2GMessage};
use crate::rectangle::{Rectangle, Size, RectangleSize};
use crate::ui::Ui;

use std::sync::mpsc::{Sender, Receiver};

pub type AsteroidRow = Vec<Asteroid>;
pub type AsteroidRows = Vec<AsteroidRow>;

#[derive(Clone)]
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

#[derive(Clone, Debug)]
pub struct Missile {
    pub position: Position,
    pub active: bool,
}

#[derive(Clone)]
pub struct Spaceship {
    pub position: Position,
    pub life: u8,
    pub shooting: bool,
}

#[derive(Clone)]
pub enum State {
    Running,
    Paused,
    Closed,
    Died,
    NextGen(u128),
}

#[derive(Clone)]
pub struct DebugOptions {
    pub generation_line: bool,
    pub rows: bool,
    pub object_count: bool,
}

#[derive(Clone)]
pub enum Debug {
    Debugging,
    Not,
}

#[derive(Clone)]
pub struct Game {
    pub spaceship: Spaceship,
    pub asteroids: AsteroidRows,
    pub missiles: Vec<Missile>,
    pub shooting_info: ShootingInfo,
    pub state: State,
    pub debug_options: DebugOptions,
    pub debug: Debug,
}

impl Game {
    pub fn new() -> Game {
        return Game {
            spaceship: Spaceship {
                position: Position {
                    x: settings::INITIAL_SPACESHIP_POSITION.x,
                    y: settings::INITIAL_SPACESHIP_POSITION.y,
                },
                life: settings::SPACESHIP_LIFE,
                shooting: false,
            },
            shooting_info: ShootingInfo {
                last_shot_time: time::now() - Game::nanoseconds_shot_delay(settings::SHOTS_PER_SECOND),
                delay_to_next_shot: Game::nanoseconds_shot_delay(settings::SHOTS_PER_SECOND),
            },
            asteroids: Game::initialize_asteroids(),
            missiles: vec![],
            state: State::Running,
            debug_options: settings::DEFAULT_DEBUG_OPTIONS,
            debug: if settings::DEBUG { Debug::Debugging } else { Debug::Not },
        };
    }

    fn map_asteroids(mut asteroids: AsteroidRows, f: fn (row: AsteroidRow) -> Vec<Asteroid>) -> AsteroidRows {
        for (i, row) in asteroids.clone().iter().cloned().enumerate() {
            asteroids[i] = f(row);
        }

        return asteroids;
    }

    pub fn get_centered_row_y_position(row: usize) -> u32 {
        let y_position = (settings::ASTEROIDS_ROWS_HEIGHT as u32 * row as u32) + settings::ASTEROIDS_ROWS_HEIGHT as u32 / 2;
        return y_position;
    }

    pub fn get_row_by_y_position(y_position: i32) -> usize {
        return (y_position as i64 / settings::ASTEROIDS_ROWS_HEIGHT as i64) as usize; 
    }

    fn nanoseconds_shot_delay(shots_per_second: u16) -> u128 {
        return ((1.0 / shots_per_second as f32) * 1_000_000_000.0) as u128;
    }

    fn generate_asteroid(
        rng: &mut rand::prelude::ThreadRng,
        existing_asteroids: AsteroidRows,
        row: Option<usize>,
    ) -> Asteroid {
        let row = if let Some(set_row) = row { set_row } else { rng.gen_range(0..settings::ASTEROIDS_ROWS) };
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
                y: Game::get_centered_row_y_position(row) as i32,
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
            let range;
            if settings::MIN_GENERATED_ASTEROIDS >= settings::MAX_GENERATED_ASTEROIDS {
                range = 1;
            } else {
                range = rng.gen_range(settings::MIN_GENERATED_ASTEROIDS..settings::MAX_GENERATED_ASTEROIDS);
            }

            for _ in 0..range {
                let generated_asteroid = Game::generate_asteroid(&mut rng, vec![vec![]; settings::ASTEROIDS_ROWS], Some(row_i));
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

    pub fn appearing_asteroids(asteroids: AsteroidRows) -> i32 {
        let mut appearing_asteroids = 0;

        for row in asteroids.iter() {
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

        return appearing_asteroids;
    }

    fn asteroids_generation(&mut self) {
        let appearing_asteroids = Game::appearing_asteroids(self.asteroids.clone());

        let mut rng = rand::thread_rng();
        for _ in 0..appearing_asteroids {
            let generated_asteroid = Game::generate_asteroid(&mut rng, self.asteroids.clone(), None);
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
            row.iter().cloned().filter(|asteroid| asteroid.position.x > 0 && asteroid.size > 0).collect()
        });
    }

    fn check_spaceship_crash(&mut self) {
        for row in self.asteroids.iter() {
            for asteroid in row.iter() {
                let new_spaceship_life = self.spaceship.life as i16 - asteroid.size as i16;
                let next_pos = Game::next_position(asteroid.position, asteroid.speed, Position {
                    x: -1,
                    y: 0,
                });

                if next_pos.x < self.spaceship.position.x && asteroid.position.x > self.spaceship.position.x && new_spaceship_life > 0 {
                    if self.spaceship.life as i32 - (asteroid.size as i32) < 0 {
                        self.spaceship.life = 0;
                    } else {
                        self.spaceship.life -= asteroid.size;
                    }
                }else if new_spaceship_life == 0 {
                    self.state = State::Died;
                }
            }
        }
    }

    fn shot(&mut self) {
        if self.spaceship.shooting && time::now() >= self.shooting_info.last_shot_time + self.shooting_info.delay_to_next_shot{
            let missile = Missile {
                active: true,
                position: Position {
                    x: self.spaceship.position.x + settings::SPACESHIP_WIDTH as i32 / 2,
                    y: self.spaceship.position.y + settings::SPACESHIP_HEIGHT as i32 / 2,
                }
            };
            
            if self.missiles.len() > 0 && self.missiles[0].active == false {
                self.missiles[0] = missile;
            } else {
                self.missiles.push(missile);
            }

            self.shooting_info.last_shot_time = time::now();
        }
    }

    fn update_missiles_position(&mut self) {
        for (missile_i, missile) in self.missiles.clone().iter().enumerate() {
            if !missile.active { continue; } 
            let corners = Rectangle {
                position: Position {
                    x: missile.position.x,
                    y: missile.position.y,
                },
                size: Size::Rectangle(RectangleSize {
                    width: settings::MISSILE_WIDTH,
                    height: settings::MISSILE_HEIGHT,
                }),
            }.get_corners();

            if corners.top_left.x > settings::WINDOW_WIDTH as i32 {
                self.missiles[missile_i].active = false;
                continue;
            }
            
            self.missiles[missile_i].position.x += settings::MISSILE_SPEED as i32;
        }
    }

    fn check_missile_collision(&mut self) {
        for (missile_i, _) in self.missiles.clone().iter().enumerate() {
            let missile = &mut self.missiles[missile_i];

            if !missile.active { continue; }

            let missiles_row = Game::get_row_by_y_position(missile.position.y);
            let inside_rectangle = Rectangle {
                position: missile.position,
                size: Size::Rectangle(RectangleSize {
                    width: settings::MISSILE_WIDTH,
                    height: settings::MISSILE_HEIGHT,
                }),
            };
            if missiles_row >= self.asteroids.len() {
                continue;
            }
            
            for (asteroid_i, _) in self.asteroids[missiles_row].clone().iter().enumerate() {
                let asteroid = &mut self.asteroids[missiles_row][asteroid_i];

                let outside_rectangle = Rectangle {
                    position: asteroid.position,
                    size: Size::Square(Ui::to_pixels(asteroid.size as u32)),
                };
                
                if inside_rectangle.over(outside_rectangle) {
                    asteroid.size -= 1;
                    missile.active = false;
                    break;
                }
            }
        }
    }

    fn next_generation(&mut self) {
        if let State::NextGen(next_gen_timestamp) = self.state {
            if time::now() >= next_gen_timestamp {
                self.asteroids = Game::initialize_asteroids();
                self.state = State::Running;
            }
        }
    }

    fn check_next_generation(&mut self) {
        if let State::Running = self.state {
            let mut asteroid_count = 0;
            
            for row in self.asteroids.iter() {
                asteroid_count += row.len();
            }
            if asteroid_count == 0 {
                self.state = State::NextGen(time::now() + settings::NEXT_GENERATION_DELAY);
            }
        }
    }

    fn update(&mut self) {
        self.check_next_generation();
        self.next_generation();
        self.check_spaceship_crash();
        self.missiles = Game::sort_missiles(self.missiles.clone());
        self.shot();
        self.check_missile_collision();
        self.update_missiles_position();
        self.asteroids_generation();
        self.update_asteroids_positions();
        self.clear_asteroids();
    }

    fn get_inputs(&mut self, rx: &Receiver<U2GMessage>) {
        let rx_message = rx.try_iter();

        for message in rx_message {
            match message {
                U2GMessage::MouseMotion(mouse_position) => {
                    self.spaceship.position.y = mouse_position.y - settings::SPACESHIP_HEIGHT as i32 / 2;
                }
                U2GMessage::Event(event) => {
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
                                rows: !self.debug_options.rows,
                                ..self.debug_options
                            }
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::F8),
                            ..
                        } => {
                            self.debug_options = DebugOptions {
                                object_count: !self.debug_options.object_count,
                                ..self.debug_options
                            }
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::F12),
                            ..
                        } => {
                            println!("Asteroids: {:#?}", self.asteroids);
                            println!("Missiles: {:#?}", self.missiles);
                        }
        
                        Event::Quit {..} => {
                            self.state = State::Closed;
                        }
                        _ => {}
                    }
                }
                U2GMessage::Close => {
                    self.state = State::Closed;
                }
            }
        }
    }

    pub fn delay_fps(fps: u32) {
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / fps));
    }

    pub fn init(&mut self, tx: &Sender<G2UMessage>, rx: &Receiver<U2GMessage>) {
        'main_loop: loop {
            match self.state {
                State::Running | State::Paused | State::NextGen(..) => {
                    self.get_inputs(rx);

                    if let State::Running | State::NextGen(..) = &self.state {
                        self.update()
                    }
                                    
                    tx.send(G2UMessage::StateUpdate(self.clone())).unwrap();
                    Game::delay_fps(settings::FPS);
                }
                _ => {
                    break 'main_loop;
                }
            }
        }
    }
}