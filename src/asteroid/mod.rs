use crate::settings;
use crate::physics;
use crate::rectangle::{Rectangle, Size};
use crate::game::{Game, AsteroidRows, AsteroidRow};
use crate::ui::{Ui};

use rand::prelude::*;

use physics::{Position, ChangingFactor};

#[derive(Clone, Copy, Debug)]
pub struct Asteroid {
    pub position: Position,
    pub row: usize,
    pub size: u8,
    pub velocity: ChangingFactor,
    pub direction: ChangingFactor,
    pub acceleration: ChangingFactor,
}

impl Asteroid {
    

    pub fn new(
        rng: &mut rand::prelude::ThreadRng,
        existing_asteroids: Option<&AsteroidRows>,
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
                if let Some(existing_asteroids) = existing_asteroids {
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
                }

                if !inside {
                    break 'generation_loop;
                }
            }
    
            let generated_asteroid = Asteroid {
                position: asteroid_position,
                size,
                row,
                velocity: ChangingFactor {
                    x: rng.gen_range(
                        settings::MIN_ASTEROIDS_SPEED..settings::MAX_ASTEROIDS_SPEED
                    ) as f32,
                    y: 0.0,
                },
                direction: ChangingFactor {
                    x: -1_f32,
                    y: 0_f32,
                },
                acceleration: ChangingFactor {
                    x: 0.04,
                    y: 0_f32,
                }
            };
    
            return generated_asteroid;
    }

    pub fn update_asteroid_position(&mut self) {
        self.position = physics::next_position(
            Rectangle {
                position: self.position,
                size: Size::Square(self.size as u32),
            },
            self.velocity,
            vec![
                physics::Force {
                    direction: self.direction,
                    acceleration: self.acceleration,
                }
            ]
        );
    }

    pub fn is_used(&self) -> bool {
        self.position.x > 0 && self.size > 0
    }

    pub fn update_asteroids_positions(asteroids: &mut AsteroidRows) {
        *asteroids = Asteroid::map_asteroids(asteroids.clone(), |row| {
            row.iter().cloned().map(|mut asteroid| {
                asteroid.update_asteroid_position();
                return asteroid;
            }).collect()
        });
    }

    pub fn unload_unused_asteroids(asteroids: &mut AsteroidRows) {
        *asteroids = Asteroid::map_asteroids(asteroids.clone(), |row| {
            row.iter().cloned().filter(|asteroid| asteroid.is_used()).collect()
        });
    }

    pub fn map_asteroids(mut asteroids: AsteroidRows, f: fn (row: AsteroidRow) -> Vec<Asteroid>) -> AsteroidRows {
        for (i, row) in asteroids.clone().iter().cloned().enumerate() {
            asteroids[i] = f(row);
        }

        return asteroids;
    }

    pub fn appearing_asteroids(asteroids: &AsteroidRows) -> i32 {
        let mut appearing_asteroids = 0;

        for row in asteroids.iter() {
            for asteroid in row.iter() {
                let corners = Rectangle {
                    position: asteroid.position,
                    size: Size::Square(Ui::to_pixels(asteroid.size as u32)),
                }.get_corners();

                if (corners.top_left.x as i64) < settings::WINDOW_WIDTH as i64
                && corners.top_left.x as i64 > settings::WINDOW_WIDTH as i64 - settings::GENERATE_NEW_ASTEROID_AFTER as i64 {
                    let next_position = physics::next_position(
                        Rectangle {
                            position: asteroid.position,
                            size: Size::Square(asteroid.size as u32),
                        },
                        asteroid.velocity,
                        vec![
                            physics::Force {
                                direction: asteroid.direction,
                                acceleration: asteroid.acceleration,
                            }
                        ]
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

    pub fn initialize_asteroids(rows: usize, min_asteroids_per_row: u8, max_asteroids_per_row: u8) -> AsteroidRows {
        let mut asteroids = vec![vec![]; rows];
        let mut rng = rand::thread_rng();
        for row_i in 0..rows {
            let range;
            if min_asteroids_per_row >= max_asteroids_per_row {
                range = 1;
            } else {
                range = rng.gen_range(min_asteroids_per_row..max_asteroids_per_row);
            }

            for _ in 0..range {
                let generated_asteroid = Asteroid::new(&mut rng, Some(&vec![vec![]; rows]), Some(row_i));
                asteroids[row_i].push(generated_asteroid);
            }
        }

        return asteroids;
    }
}