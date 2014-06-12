use rsfml::graphics::{RectangleShape, RenderWindow, Color, Vertex, VertexArray, LinesStrip};
use rsfml::system::{Vector2f, Clock};

use texture_loader::TextureLoader;
use animation::*;

pub struct HUD<'s> {
    window_size : Vector2f,
    background : RectangleShape<'s>,
    hud_vertex_array : VertexArray,
    face : RectangleShape<'s>,
    face_animation : Animation,
    texture_loader : &'s TextureLoader,
    face_clock : Clock
}

impl<'s> HUD<'s> {
    pub fn new(window_size : &Vector2f, texture_loader : &'s TextureLoader) -> HUD<'s> {
        let mut array = VertexArray::new().unwrap();
        array.set_primitive_type(LinesStrip);
        let mut tmp_face = RectangleShape::new_init(&Vector2f {x : 43., y : 58.}).unwrap();
        tmp_face.set_position2f(window_size.x / 2. - 21., window_size.y - 71.);
        HUD {
            window_size : window_size.clone(),
            background : RectangleShape::new().unwrap(),
            hud_vertex_array : array,
            face : tmp_face,
            face_animation : Animation::new(vec![40, 41, 42], Play, PlayOnce, 1., 0),
            texture_loader : texture_loader,
            face_clock : Clock::new()
        }
    }

    pub fn update(&mut self) -> () {
        self.background.set_size2f(self.window_size.x - 21., 59.);
        self.background.set_fill_color(&Color::new_RGB(6, 1, 162));
        self.background.set_position2f(10., self.window_size.y - 70.);
        self.face_animation.update();
        self.face.set_texture(self.texture_loader.get_texture(self.face_animation.get_current_texture_id()), false);
        if self.face_clock.get_elapsed_time().as_seconds() >= 7. {
            self.face_animation.set_state(Play);
            self.face_clock.restart();
        } 
    }

    fn draw_line(&mut self, x1 : f32, x2 : f32, y1 : f32, y2 : f32, color : &Color, render_window : &mut RenderWindow) -> () {
        self.hud_vertex_array.clear();
        self.hud_vertex_array.append(&Vertex::new_with_pos_color(&Vector2f {x : x1, y : y1}, color));
        self.hud_vertex_array.append(&Vertex::new_with_pos_color(&Vector2f {x : x2, y : y2}, color));
        render_window.draw(&self.hud_vertex_array);
    }

    fn draw_2line(&mut self, x1 : f32, x2 : f32, x3 : f32, y1 : f32, y2 : f32, y3 : f32, color : &Color, render_window : &mut RenderWindow) -> () {
        self.hud_vertex_array.clear();
        self.hud_vertex_array.append(&Vertex::new_with_pos_color(&Vector2f {x : x1, y : y1}, color));
        self.hud_vertex_array.append(&Vertex::new_with_pos_color(&Vector2f {x : x2, y : y2}, color));
        self.hud_vertex_array.append(&Vertex::new_with_pos_color(&Vector2f {x : x3, y : y3}, color));
        render_window.draw(&self.hud_vertex_array);
    }

    pub fn draw(&mut self, render_window : &mut RenderWindow) -> () {
        render_window.draw(&self.background);
        self.draw_2line(self.window_size.x - 9., self.window_size.x - 9., 9., self.window_size.y - 70., self.window_size.y - 10., self.window_size.y - 10., &Color::new_RGBA(255, 255, 255, 75), render_window);
        self.draw_2line(self.window_size.x - 11., self.window_size.x - 11., 11., self.window_size.y - 70., self.window_size.y - 12., self.window_size.y - 12., &Color::black(), render_window);
        self.draw_2line(9., 9., self.window_size.x - 9., self.window_size.y - 12., self.window_size.y - 71., self.window_size.y - 71., &Color::black(), render_window);
        self.draw_2line(11., 11., self.window_size.x - 11., self.window_size.y - 11., self.window_size.y - 69., self.window_size.y - 69., &Color::new_RGBA(255, 255, 255, 75), render_window);
        self.draw_line(self.window_size.x, 0., self.window_size.y - 80., self.window_size.y - 80., &Color::new_RGBA(255, 255, 255, 50), render_window);
        self.draw_line(self.window_size.x, 0., self.window_size.y - 79., self.window_size.y - 79., &Color::new_RGBA(255, 255, 255, 75), render_window);
        render_window.draw(&self.face);
    }
}