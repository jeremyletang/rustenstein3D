
use rsfml::graphics::{RenderWindow, View, Color, FloatRect, RectangleShape};
use rsfml::system::{Vector2u, Vector2i, Vector2f};

use texture_loader::TextureLoader;
use map::*;

pub struct MiniMap {
    priv map : Map,
    priv active : bool,
    priv mini_map_view : @mut View,
    priv player_pos : Vector2f,
    priv rotation : f32
}

impl MiniMap {
    pub fn new(map : Map, 
               window_size : &Vector2u) -> MiniMap {
        let tmp_view = @mut View::new().unwrap();
        tmp_view.set_size2f(window_size.x as f32, window_size.y as f32);
        tmp_view.set_viewport(&FloatRect::new(0.70, 0.70, 0.25, 0.25));
        tmp_view.set_rotation(-90.);
        MiniMap {
            map : map,
            active : true,
            mini_map_view : tmp_view,
            player_pos : Vector2f {x : 0., y : 0. },
            rotation : 0.
        }
    }

    pub fn set_active(&mut self) -> bool {
        self.active = match self.active {
            true    => false,
            false   => true,
        };
        self.active
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn update(&mut self, 
                  player_position : Vector2f, 
                  new_rotation : f32) -> () {    
        self.player_pos = player_position;
        self.mini_map_view.rotate(new_rotation as float);
        self.mini_map_view.set_center2f(self.player_pos.x * 80., self.player_pos.y * 80.);         
        self.rotation += new_rotation;
    }

    pub fn draw(&self, 
                render_window : &mut RenderWindow, 
                texture_loader : &TextureLoader) -> () {
        let mut block : i32;
        let def_view = render_window.get_default_view();
        let map_size = self.map.get_map_size();
        let mut pos : Vector2i = Vector2i::new(0, 0);
        let mut rect = RectangleShape::new_init(~Vector2f::new(80., 80.)).unwrap();
        rect.set_fill_color(&Color::new_RGBA(255, 255, 255, 125));
        render_window.set_view(self.mini_map_view);
        while pos.x < map_size.x {
            while pos.y < map_size.y {
                block = self.map.get_block(&pos).expect("Cannot get block in minimap.");
                match block {
                    0 => { rect.disable_texture(); 
                           rect.set_position2f(pos.x as f32 * 80., pos.y as f32 * 80.); 
                    },
                    _ => { rect.set_texture(texture_loader.get_texture(block), false); 
                           rect.set_position2f(pos.x as f32 * 80., pos.y as f32 * 80.); 
                    }
                }
                render_window.draw(&rect);
                pos.y += 1;
            }
            pos.x += 1;
            pos.y = 0;
        }
        rect.set_fill_color(~Color::new_RGBA(255, 0, 0, 125));
        rect.set_origin2f(40., 40.);
        rect.set_position2f(self.player_pos.x as f32 * 80., self.player_pos.y as f32 * 80.);
        render_window.draw(&rect);
        render_window.set_view(def_view);
    }
}