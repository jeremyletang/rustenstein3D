
use FPS::*;
use event_handler::*;
use game_mode::*;
use texture_loader::TextureLoader;

use rsfml::graphics::{RenderWindow, Font, Color};
use rsfml::window::keyboard;

pub struct GameLoop<'self> {
    priv render_window : @mut RenderWindow,
    priv fps_handler : Option<FPSHandler<'self>>,
    priv event_handler : @mut EventHandler,
    priv clear_color : Color,
    priv game_mode : GameMode<'self>,
    priv texture_loader : &'self TextureLoader
}

impl<'self> GameLoop<'self> {
    pub fn new(render_window : @mut RenderWindow,
               texture_loader : &'self TextureLoader) -> GameLoop<'self> {
        GameLoop {
            render_window : render_window,
            fps_handler : None,
            event_handler : @mut EventHandler::new(render_window),
            clear_color : Color::new_RGB(3, 64, 59),
            game_mode : GameMode::new(render_window.get_size(), texture_loader),
            texture_loader : texture_loader
        }
    }

    pub fn activate_FPS(&mut self, 
                        font : &'self Font) -> (){
        match self.fps_handler {
            Some(_)     => (),
            None        => self.fps_handler = Some(FPSHandler::new(self.render_window, font))
        }
    }

    pub fn deactivate_FPS(&mut self) -> () {
        match self.fps_handler {
            Some(_)     => self.fps_handler = None,
            None        => ()
        }
    }

    pub fn run(&mut self) -> () {
        while self.render_window.is_open() {
            self.update();
            self.draw();
        }
    }

    pub fn update(&mut self) -> () {
        self.event_handler.update_events();
        if self.event_handler.has_closed_event() ||
           self.event_handler.is_key_pressed(keyboard::Escape) {
            self.render_window.close();
        }
        self.game_mode.update(self.event_handler);
        self.fps_handler.get_mut_ref().update();
    }

    pub fn draw(&mut self) -> () {
        self.render_window.clear(&self.clear_color);
        self.game_mode.draw(self.render_window);
        match self.fps_handler {
            Some(_)     => self.fps_handler.get_mut_ref().draw(),
            None        => {}
        };
        self.render_window.display();
    }
}