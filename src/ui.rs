use sdl2::{Sdl, VideoSubsystem, EventPump};
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::event::{Event};
use sdl2::render;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::render::Texture;

use sdl2::rect::Rect;
use sdl2::ttf::Font;

use std::sync::mpsc::{Sender, Receiver};
use std::path::Path;

use crate::helper::{Position, G2UMessage, U2GMessage};
use crate::settings;
use crate::rectangle::{Rectangle, Size, RectangleSize};
use crate::game;

use game::{Game, Debug};

type TextureCreator = sdl2::render::TextureCreator<sdl2::video::WindowContext>;

pub struct UiSettings {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub struct Ui {
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
}

impl Ui {
    pub fn new(ui_settings: UiSettings) -> Ui {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let _image_context = sdl2::image::init(InitFlag::PNG);

        let window = video_subsystem
            .window(&ui_settings.title, ui_settings.width, ui_settings.height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        Ui {
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
        }
    }

    pub fn to_pixels(value: u32) -> u32 {
        return value * settings::PIXELS_MULTIPLIER_FACTOR;
    }

    fn draw_background(&mut self, sprites_texture: &Texture) {
        self.draw_sprite(sprites_texture,
            settings::BACKGROUND_SPRITE_RECTANGLE,
            Rectangle {
                position: Position {
                    x: settings::WINDOW_WIDTH as i32 / 2,
                    y: settings::WINDOW_HEIGHT as i32 / 2,
                },
                size: Size::Rectangle(RectangleSize {
                    width: settings::WINDOW_WIDTH,
                    height: settings::WINDOW_HEIGHT,
                })
            }
        );
    }

    fn draw_asteroids(
        &mut self,
        game: &mut Game,
        sprites_texture: &Texture,
    ) {
        for row in game.asteroids.iter() {
            for asteroid in row.iter() {
                let size = Ui::to_pixels(asteroid.size as u32);
                
                let target_rectangle = Rectangle {
                    position: asteroid.position,
                    size: Size::Square(size),
                };

                let target_rectangle_corners = target_rectangle.get_corners();

                if target_rectangle_corners.top_left.x > settings::WINDOW_WIDTH as i32 {
                    continue;
                }

                self.draw_sprite(
                    sprites_texture,
                    settings::ASTEROID_SPRITE_RECTANGLE,
                    target_rectangle,
                );
            }
        };
    }

    fn draw_spaceship(
        &mut self,
        game: &mut Game,
        sprites_texture: &Texture,
    ) {        
        self.draw_sprite(
            sprites_texture,
            settings::SPACESHIP_SPRITE_RECTANGLE,
            Rectangle {
                position: Position {
                    x: game.spaceship.position.x, 
                    y: game.spaceship.position.y + settings::SPACESHIP_HEIGHT as i32 / 2,
                },
                size: Size::Rectangle(RectangleSize {
                    width: settings::SPACESHIP_WIDTH,
                    height: settings::SPACESHIP_HEIGHT,
                })
            },
        );            
    }

    fn draw_missiles(&mut self, game: &mut Game) {
        let canvas = &mut self.canvas;

        let mut rects: Vec<Rect> = vec![];
        for missile in game.missiles.iter() {
            if missile.position.x > settings::WINDOW_WIDTH as i32 || !missile.active { continue; }
            let rect = Rectangle {
                position: missile.position,
                size: Size::Rectangle(RectangleSize {
                    width: settings::MISSILE_WIDTH,
                    height: settings::MISSILE_HEIGHT,
                }),
            };
            let corners = rect.get_corners();

            rects.push(Rect::new(
                corners.top_left.x,
                corners.top_left.y,
                settings::MISSILE_WIDTH,
                settings::MISSILE_HEIGHT,
            ));
        }

        canvas.set_draw_color(settings::MISSILE_COLOR);
        canvas.fill_rects(&rects).unwrap();
    }

    fn draw_spaceship_life(&mut self, game: &mut Game) {
        let canvas = &mut self.canvas;

        let size = RectangleSize{ 
            width: 13,
            height: (settings::SPACESHIP_HEIGHT as f32 / settings::SPACESHIP_LIFE as f32 * game.spaceship.life as f32) as u32,
        };

        let life_rectangle = Rectangle {
            position: Position {
                x: game.spaceship.position.x
                    - settings::SPACESHIP_WIDTH as i32 / 2
                    - size.width as i32 - 3,
                y: game.spaceship.position.y + (settings::SPACESHIP_HEIGHT - size.height) as i32,
            },
            size: Size::Rectangle(size)
        };

        canvas.set_draw_color(settings::LIFE_COLOR);
        canvas.fill_rect(Rect::new(
            life_rectangle.position.x,
            life_rectangle.position.y,
            size.width,
            size.height,
        )).unwrap();
    }

    fn inputs(&mut self, tx: &Sender<U2GMessage>) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::MouseMotion { x, y, ..} => {
                    let mouse_position = Position {
                        x: x,
                        y: y,
                    };
                    tx.send(U2GMessage::MouseMotion(mouse_position)).unwrap();
                }
                Event::Quit {..} => {
                    tx.send(U2GMessage::Close).unwrap();
                }
                _ => {
                    tx.send(U2GMessage::Event(event)).unwrap();
                }
            }
        }
    }

    fn write_text(
        &mut self,
        text: &str,
        color: Color,
        position: Position,
        font: &Font,
        texture_creator: &TextureCreator,
    ) -> Rectangle {
        let surface = font
            .render(text)
            .blended(color)
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let render::TextureQuery { width, height, .. } = texture.query();

        let target = Rect::new(position.x, position.y, width, height);
        self.canvas.copy(&texture, None, Some(target)).unwrap();

        return Rectangle {
            position,
            size: Size::Rectangle(RectangleSize {
                width,
                height,
            })
        }
    }

    fn draw_sprite(
        &mut self,
        texture: &Texture,
        sprite_rectangle: Rectangle,
        target_rectangle: Rectangle,
    ) {
        let sprite_rectangle_size = Rectangle::to_rectangle_size(sprite_rectangle.size);
        
        let target_corners = target_rectangle.get_corners();
        let target_rectangle_size = Rectangle::to_rectangle_size(target_rectangle.size);

        self.canvas.copy(
            texture,
            Some(
                Rect::new(
                    sprite_rectangle.position.x,
                    sprite_rectangle.position.y,
                    sprite_rectangle_size.width,
                    sprite_rectangle_size.height,
                )
            ),
            Some(
                Rect::new(
                    target_corners.top_left.x,
                    target_corners.top_left.y,
                    target_rectangle_size.width,
                    target_rectangle_size.height,
                )
            )
        ).unwrap();
    }

    pub fn run(&mut self, tx: &Sender<U2GMessage>, rx: &Receiver<G2UMessage>) {
        let ttf_context = sdl2::ttf::init().unwrap();
        let texture_creator = self.canvas.texture_creator();

        // Load Sprites
        let sprites_texture = texture_creator.load_texture(
            Path::new(settings::SPRITES_FILE_PATH)
        ).unwrap();

        // Load debug font
        let mut debug_font = ttf_context.load_font(
            Path::new(settings::DEBUG_FONT_PATH),
            settings::DEBUG_FONT_POINT_SIZE
        ).unwrap();
        debug_font.set_style(sdl2::ttf::FontStyle::NORMAL);


        for message in rx.iter() {
            self.inputs(tx);

            let G2UMessage::StateUpdate(mut game) = message;
            let game = &mut game;

            self.draw_background(&sprites_texture);

            match game.debug {
                Debug::Debugging => {
                    self.debug(
                        game,
                        &debug_font,
                        &texture_creator,
                    );
                }
                _ => {}
            }

            self.draw_spaceship(game, &sprites_texture);
            self.draw_missiles(game);
            self.draw_asteroids(game, &sprites_texture);
            self.draw_spaceship_life(game);

            self.canvas.present();
        }
    }

    pub fn debug(
        &mut self,
        game: &mut Game,
        debug_font: &Font,
        texture_creator: &TextureCreator,
    ) {
        let canvas = &mut self.canvas;

        if game.debug_options.generation_line {
            // Draw Generation Line
            canvas.set_draw_color(settings::DEBUG_COLOR);
            canvas.fill_rect(
                Rect::new(
                    settings::WINDOW_WIDTH as i32
                    - settings::GENERATE_NEW_ASTEROID_AFTER as i32,
                    0,
                    1,
                    settings::WINDOW_HEIGHT
                )
            ).unwrap();
        }

        if game.debug_options.rows_starting_line {
            let mut rects: Vec<Rect> = vec![];
            for row in 0..game.asteroids.len() {
                let y_position = Game::get_row_y_position(row);
                rects.push(Rect::new(
                    0,
                    y_position as i32,
                    settings::WINDOW_WIDTH,
                    1
                ));
            }

            canvas.set_draw_color(settings::DEBUG_COLOR);
            canvas.fill_rects(&rects).unwrap();
        }

        if game.debug_options.object_count {
            let missiles = game.missiles.len();
            let mut asteroids = 0;
            for row in game.asteroids.iter() {
                asteroids += row.len();
            }

            let missiles_count_text = self.write_text(
                &format!("Missiles count: {}", missiles),
                settings::DEBUG_COLOR,
                Position {
                    x: 10,
                    y: 10,
                }, 
                debug_font,
                texture_creator
            );

            let missiles_count_text_corners = missiles_count_text.get_corners();
            let missiles_count_text_size = Rectangle::to_rectangle_size(missiles_count_text.size);

            let asteroids_count_text = self.write_text(
                &format!("Asteroids count: {}", asteroids),
                settings::DEBUG_COLOR,
                Position {
                    x: 10,
                    y: missiles_count_text_corners.top_left.y + missiles_count_text_size.height as i32 + 20,
                }, 
                debug_font,
                texture_creator
            );

            let asteroids_count_text_corners = asteroids_count_text.get_corners();
            let asteroids_count_text_size = Rectangle::to_rectangle_size(asteroids_count_text.size);

            let _life_display_text = self.write_text(
                &format!("Life: {}", game.spaceship.life),
                settings::DEBUG_COLOR,
                Position {
                    x: 10,
                    y: asteroids_count_text_corners.top_left.y + asteroids_count_text_size.height as i32 + 20,
                }, 
                debug_font,
                texture_creator
            );
        }
    }
}