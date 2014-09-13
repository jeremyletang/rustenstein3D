#![allow(non_snake_case)]

use FPS::*;
use event_handler::*;
use game_mode::*;
use texture_loader::TextureLoader;

use rsfml::graphics::{RenderWindow, RenderTarget, Font, Color};
use rsfml::window::keyboard;

pub struct GameLoop<'s> {
    render_window : RenderWindow,
    fps_handler : Option<FPSHandler<'s>>,
    event_handler : EventHandler,
    clear_color : Color,
    game_mode : GameMode<'s>,
    texture_loader : &'s TextureLoader
}

impl<'s> GameLoop<'s> {
    pub fn new(render_window : RenderWindow,
               texture_loader : &'s TextureLoader,
               noground : bool) -> GameLoop<'s> {
        let tmp_size = render_window.get_size();
        GameLoop {
            render_window : render_window,
            fps_handler : None,
            event_handler : EventHandler::new(),
            clear_color : Color::new_RGB(3, 64, 59),
            game_mode : GameMode::new(tmp_size, texture_loader, noground),
            texture_loader : texture_loader
        }
    }

    pub fn activate_FPS(&mut self, 
                        font : &'s Font) -> (){
        match self.fps_handler {
            Some(_)     => (),
            None        => self.fps_handler = Some(FPSHandler::new(font))
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
        self.event_handler.update_events(&mut self.render_window);
        if self.event_handler.has_closed_event() ||
           self.event_handler.is_key_pressed(keyboard::Escape) {
            self.render_window.close();
        }
        self.game_mode.update(&self.event_handler);
        self.fps_handler.as_mut().unwrap().update();
    }

    pub fn draw(&mut self) -> () {
        self.render_window.clear(&self.clear_color);
        self.game_mode.draw(&mut self.render_window);
        match self.fps_handler {
            Some(_)     => self.fps_handler.as_mut().unwrap().draw(&mut self.render_window),
            None        => {}
        };
        self.render_window.display();
    }
}