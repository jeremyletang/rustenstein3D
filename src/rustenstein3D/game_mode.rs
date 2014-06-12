
use rsfml::graphics::{RenderWindow, RectangleShape, Color};
use rsfml::system::{Vector2u, Vector2f, Vector2i, ToVec};
use rsfml::window::keyboard;

use map;
use mini_map::*;
use event_handler::*;
use raycasting_engine::REngine;
use texture_loader::TextureLoader;
use hud::HUD;
use weapon::Weapon;

pub struct GameMode<'s> {
    window_size : Vector2u,
    map : map::Map,
    mini_map : MiniMap,
    player_position : Vector2f,
    r_engine : REngine,
    texture_loader : &'s TextureLoader,
    hud : HUD<'s>,
    weapon : Weapon<'s>,
    sky : RectangleShape<'s>,
    ground : RectangleShape<'s>
}

impl<'s> GameMode<'s> {
    pub fn new(window_size : Vector2u,
               texture_loader : &'s TextureLoader,
               noground : bool) -> GameMode<'s> {
        let map = GameMode::get_map();
        let mut sky = RectangleShape::new_init(&Vector2f { x : window_size.x as f32, y : window_size.y as f32 / 2. - 40.}).unwrap();
        sky.set_fill_color(&Color::new_RGB(63, 48, 21));
        let mut ground = RectangleShape::new_init(&Vector2f { x : window_size.x as f32, y : window_size.y as f32 / 2. - 40. }).unwrap();
        ground.set_fill_color(&Color::new_RGB(109, 108, 112));
        ground.set_position2f(0., window_size.y as f32 / 2. - 40.);
        GameMode {
            window_size : window_size,
            map : map.clone(),
            mini_map : MiniMap::new(map.clone(), &window_size),
            player_position : Vector2f { x : 4., y : 1. },
            r_engine : REngine::new(map, &window_size.to_vector2f(), noground),
            texture_loader : texture_loader,
            hud : HUD::new(&window_size.to_vector2f(), texture_loader),
            weapon : Weapon::new(&window_size.to_vector2f(), texture_loader),
            sky : sky,
            ground : ground
        }
    }

    pub fn get_map() -> map::Map {
        let map_i32 : Vec<i32> = vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                                 1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1,
                                 1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1,
                                 1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1,
                                 1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];
        map::Map::new(map_i32, &Vector2f::new(24., 24.))
    }

    pub fn update<'r>(&mut self, event_handler : &'r EventHandler) -> () {
        let mut rotation : f32 = 0.;
        if event_handler.is_key_pressed(keyboard::Left) {
            rotation = -5.25;
        }
        if event_handler.is_key_pressed(keyboard::Right) {
            rotation = 5.25;
        }
        match event_handler.has_key_pressed_event(keyboard::M) {
            Some((_, _, _, _, _))   => self.mini_map.set_active(),
            None                    => true
        };
        self.r_engine.update(event_handler);
        if self.mini_map.is_active() {
            self.mini_map.update(self.r_engine.get_player_pos(), rotation);
        }
        self.hud.update();
        self.weapon.update(event_handler);
    }

    pub fn draw<'r>(&mut self, render_window : &'r mut RenderWindow) -> () {
        render_window.draw(&self.sky);
        render_window.draw(&self.ground);
        self.r_engine.draw(render_window, self.texture_loader);
        if self.mini_map.is_active() {
            self.mini_map.draw(render_window, self.texture_loader);
        }
        self.hud.draw(render_window);
        self.weapon.draw(render_window);
        render_window.set_mouse_cursor_visible(false);
        render_window.set_mouse_position(&Vector2i::new((self.window_size.x / 2) as i32, (self.window_size.y / 2) as i32));
    }
}