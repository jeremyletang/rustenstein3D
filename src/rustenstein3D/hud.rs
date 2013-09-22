use rsfml::graphics::{RectangleShape, RenderWindow, Color, Vertex, VertexArray, LinesStrip};
use rsfml::system::Vector2f;

pub struct HUD<'self> {
    priv window_size : Vector2f,
    priv background : RectangleShape<'self>,
    priv face : i32,
    priv hud_vertex_array : VertexArray
}

impl<'self> HUD<'self> {
    pub fn new(window_size : &Vector2f) -> HUD<'self> {
        HUD {
            window_size : window_size.clone(),
            background : RectangleShape::new().unwrap(),
            face : 0,
            hud_vertex_array : VertexArray::new().unwrap()
        }
    }

    pub fn update(&mut self) -> () {
        self.background.set_size2f(self.window_size.x - 20., 60.);
        self.background.set_fill_color(~Color::new_RGB(6, 1, 162));
        self.background.set_position2f(10., self.window_size.y - 70.);
    }

    fn build_line_vertex_array(&mut self, x1 : f32, x2 : f32, y1 : f32, y2 : f32, color : &Color, render_window : &mut RenderWindow) -> () {
        self.hud_vertex_array.set_primitive_type(LinesStrip);
        self.hud_vertex_array.clear();
        self.hud_vertex_array.append(&Vertex::new_with_pos_color(~Vector2f {x : x1, y : y1}, color));
        self.hud_vertex_array.append(&Vertex::new_with_pos_color(~Vector2f {x : x2, y : y2}, color));
        render_window.draw(&self.hud_vertex_array);
    }

    pub fn draw(&mut self, render_window : &mut RenderWindow) -> () {
        render_window.draw(&self.background);
        let mut border = VertexArray::new().unwrap();
        border.set_primitive_type(LinesStrip);
        border.append(&Vertex::new_with_pos_color(~Vector2f {x : self.window_size.x - 9., y : self.window_size.y - 70.}, &Color::new_RGBA(255, 255, 255, 75)));
        border.append(&Vertex::new_with_pos_color(~Vector2f {x : self.window_size.x - 9., y : self.window_size.y - 10.}, &Color::new_RGBA(255, 255, 255, 75)));
        border.append(&Vertex::new_with_pos_color(~Vector2f {x : 9., y : self.window_size.y - 10.}, &Color::new_RGBA(255, 255, 255, 75)));
        render_window.draw(&border);
        border.clear();
        border.append(&Vertex::new_with_pos_color(~Vector2f {x : self.window_size.x - 11., y : self.window_size.y - 70.}, &Color::black()));
        border.append(&Vertex::new_with_pos_color(~Vector2f {x : self.window_size.x - 11., y : self.window_size.y - 12.}, &Color::black()));
        border.append(&Vertex::new_with_pos_color(~Vector2f {x : 11., y : self.window_size.y - 12.}, &Color::black()));
        render_window.draw(&border);
        border.clear();
        border.append(&Vertex::new_with_pos_color(~Vector2f {x : 9., y : self.window_size.y - 12.}, &Color::black()));
        border.append(&Vertex::new_with_pos_color(~Vector2f {x : 9., y : self.window_size.y - 71.}, &Color::black()));
        border.append(&Vertex::new_with_pos_color(~Vector2f {x : self.window_size.x - 9., y : self.window_size.y - 71.}, &Color::black()));
        render_window.draw(&border);
        border.clear();
        border.append(&Vertex::new_with_pos_color(~Vector2f {x : 11., y : self.window_size.y - 11.}, &Color::new_RGBA(255, 255, 255, 75)));
        border.append(&Vertex::new_with_pos_color(~Vector2f {x : 11., y : self.window_size.y - 69.}, &Color::new_RGBA(255, 255, 255, 75)));
        border.append(&Vertex::new_with_pos_color(~Vector2f {x : self.window_size.x - 11., y : self.window_size.y - 69.}, &Color::new_RGBA(255, 255, 255, 75)));
        render_window.draw(&border);
        
        self.build_line_vertex_array(self.window_size.x, 0., self.window_size.y - 80., self.window_size.y - 80., &Color::new_RGBA(255, 255, 255, 50), render_window);
        self.build_line_vertex_array(self.window_size.x, 0., self.window_size.y - 79., self.window_size.y - 79., &Color::new_RGBA(255, 255, 255, 75), render_window);
    }
}