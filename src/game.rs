use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use std::sync::mpsc::{Sender, Receiver};

use crate::time;
use crate::settings;
use crate::physics;
use crate::asteroid::{Asteroid};
use crate::missile;
use crate::helper::{G2UMessage, U2GMessage};
use crate::rectangle::{Rectangle, Size, RectangleSize};
use crate::ui::Ui;

use missile::{Missile, MissileType};
use physics::{Position};


pub type AsteroidRow = Vec<Asteroid>;
pub type AsteroidRows = Vec<AsteroidRow>;

#[derive(Clone)]
pub struct ShootingInfo {
    pub last_shot_time: u128,
    pub delay_to_next_shot: u128,
}

#[derive(Clone)]
pub struct Spaceship {
    pub position: Position,
    pub life: u8,
    pub shooting: bool,
    pub missile_type: MissileType,
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
    pub game_state: bool,
    pub invincible: bool,
    pub asteroid_generation: bool,
}

#[derive(Clone)]
pub struct Game {
    pub spaceship: Spaceship,
    pub asteroids: AsteroidRows,
    pub missiles: Vec<Missile>,
    pub shooting_info: ShootingInfo,
    pub state: State,
    pub debug_options: DebugOptions,
    pub debugging: bool,
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
                missile_type: MissileType::Normal,
            },
            shooting_info: ShootingInfo {
                last_shot_time: time::now(),
                delay_to_next_shot: 0,
            },
            asteroids: Asteroid::initialize_asteroids(
                settings::ASTEROIDS_ROWS,
                settings::MIN_GENERATED_ASTEROIDS,
                settings::MAX_GENERATED_ASTEROIDS,
            ),
            missiles: vec![],
            state: State::Running,
            debug_options: settings::DEFAULT_DEBUG_OPTIONS,
            debugging: settings::DEBUG,
        };
    }

    pub fn get_centered_row_y_position(row: usize) -> u32 {
        let y_position = (settings::ASTEROIDS_ROWS_HEIGHT as u32 * row as u32) + settings::ASTEROIDS_ROWS_HEIGHT as u32 / 2;
        return y_position;
    }

    pub fn get_row_by_y_position(y_position: i32) -> usize {
        return (y_position as i64 / settings::ASTEROIDS_ROWS_HEIGHT as i64) as usize; 
    }

    fn asteroids_generation(&mut self) {
        if self.debugging {
            if !self.debug_options.asteroid_generation {
                return;
            }
        }
        let appearing_asteroids = Asteroid::appearing_asteroids(&self.asteroids);

        let mut rng = rand::thread_rng();
        for _ in 0..appearing_asteroids {
            let generated_asteroid = Asteroid::new(&mut rng, Some(&self.asteroids), None);
            self.asteroids[generated_asteroid.row].push(generated_asteroid);
        }
    }

    fn check_spaceship_crash(&mut self) {
        if self.debugging && self.debug_options.invincible {
            return;
        }
        for row in self.asteroids.iter() {
            for asteroid in row.iter() {
                let new_spaceship_life = self.spaceship.life as i16 - asteroid.size as i16;
                let next_pos = physics::next_position(
                    Rectangle {
                        position: asteroid.position,
                        size: Size::Square(Ui::to_pixels(asteroid.size as u32)),
                    },
                    asteroid.velocity,
                    vec![
                        physics::Force {
                            direction: asteroid.direction,
                            acceleration: asteroid.acceleration,
                        }
                    ]
                );

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
                velocity: Missile::get_types_data(self.spaceship.missile_type).initial_velocity,
                position: Position {
                    x: self.spaceship.position.x + settings::SPACESHIP_WIDTH as i32 / 2,
                    y: self.spaceship.position.y + settings::SPACESHIP_HEIGHT as i32 / 2,
                },
                missile_type: self.spaceship.missile_type,
                direction: Missile::get_types_data(self.spaceship.missile_type).direction,
                acceleration: Missile::get_types_data(self.spaceship.missile_type).acceleration,
                collision_handler: Missile::get_types_handler(self.spaceship.missile_type),
            };
            
            if self.missiles.len() > 0 && self.missiles[0].active == false {
                self.missiles[0] = missile;
            } else {
                self.missiles.push(missile);
            }

            self.shooting_info.last_shot_time = time::now();
            self.shooting_info.delay_to_next_shot = Missile::get_types_data(self.spaceship.missile_type).delay;
        }
    }

    fn check_missile_collision(&mut self) {
        let mut cloned_missiles = self.missiles.clone();
        for (missile_i, missile) in cloned_missiles.iter_mut().enumerate() {
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

            let mut cloned_asteroids = self.asteroids[missiles_row].clone();
            
            for (asteroid_i, asteroid) in cloned_asteroids.iter_mut().enumerate() {
                let outside_rectangle = Rectangle {
                    position: asteroid.position,
                    size: Size::Square(Ui::to_pixels(asteroid.size as u32)),
                };
                
                if inside_rectangle.over(outside_rectangle) {
                    (missile.collision_handler)(self, missile, asteroid);
                    self.asteroids[missiles_row][asteroid_i] = *asteroid;
                    self.missiles[missile_i] = missile.clone();

                    break;
                }
            }
        }
    }

    fn next_generation(&mut self) {
        if !self.debugging || self.debug_options.asteroid_generation {
            if let State::NextGen(next_gen_timestamp) = self.state {
                if time::now() >= next_gen_timestamp {
                        self.asteroids = Asteroid::initialize_asteroids(
                            settings::ASTEROIDS_ROWS,
                            settings::MIN_GENERATED_ASTEROIDS,
                            settings::MAX_GENERATED_ASTEROIDS,
                        );
                        self.state = State::Running;
                }
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
        self.shot();
        self.check_missile_collision();

        Missile::update_missiles_position(&mut self.missiles);

        self.asteroids_generation();
        Asteroid::update_asteroids_positions(&mut self.asteroids);

        Asteroid::unload_unused_asteroids(&mut self.asteroids);
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
                            self.spaceship.missile_type = MissileType::Normal;
                            self.spaceship.shooting = true;
                        }
                        Event::MouseButtonUp {
                            mouse_btn: MouseButton::Left,
                            ..
                        } => {
                            self.spaceship.missile_type = MissileType::Normal;
                            self.spaceship.shooting = false;
                        }

                        Event::MouseButtonDown {
                            mouse_btn: MouseButton::Right,
                            ..
                        } => {
                            self.spaceship.missile_type = MissileType::Bomb;
                            self.spaceship.shooting = true;
                        }
                        Event::MouseButtonUp {
                            mouse_btn: MouseButton::Right,
                            ..
                        } => {
                            self.spaceship.missile_type = MissileType::Bomb;
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
                            self.debugging = !self.debugging; 
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
                                game_state: !self.debug_options.game_state,
                                ..self.debug_options
                            }
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::F9),
                            ..
                        } => {
                            self.debug_options = DebugOptions {
                                invincible: !self.debug_options.invincible,
                                ..self.debug_options
                            }
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::F10),
                            ..
                        } => {
                            self.debug_options = DebugOptions {
                                asteroid_generation: !self.debug_options.asteroid_generation,
                                ..self.debug_options
                            }
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