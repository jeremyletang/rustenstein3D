use rsfml::graphics::{RenderWindow, RectangleShape};
use rsfml::window::{keyboard, mouse};
use rsfml::system::Vector2f;

use animation::*;
use event_handler::EventHandler;
use texture_loader::TextureLoader;

pub struct Weapon<'self> {
	priv weapon : RectangleShape<'self>,
	priv animation : Animation,
	priv texture_loader : &'self TextureLoader,
	priv gun_shadow : RectangleShape<'self>
}

impl<'self> Weapon<'self> {
	pub fn new(window_size : &Vector2f, texture_loader : &'self TextureLoader) -> Weapon<'self> {
		let mut tmp_weapon = RectangleShape::new_init(&Vector2f {x : 400., y : 400.}).unwrap();
		tmp_weapon.set_position2f(window_size.x / 2. - 200., window_size.y - 400. - 81.);
		let mut tmp_shadow = RectangleShape::new_init(&Vector2f {x : 99., y : 48.}).unwrap();
		tmp_shadow.set_position2f(window_size.x - 115., window_size.y - 66.);
		tmp_shadow.set_texture(texture_loader.get_texture(18), false);
		Weapon {
			weapon : tmp_weapon,
			animation : Animation::new(~[12, 13, 14, 15, 16, 17], Stop, PlayOnce, 0.09),
			texture_loader : texture_loader,
			gun_shadow : tmp_shadow
		}
	}

	pub fn update<'r>(&'r mut self, event_handler : &'r EventHandler) -> () {
		match event_handler.has_mouse_button_pressed_event(mouse::MouseLeft) {
			Some(_) => self.animation.set_state(Play),
			None	=> {}
		};
		match event_handler.has_key_pressed_event(keyboard::E) {
		Some(_) => self.animation.set_state(Play),
		None	=> {}
		};
		self.animation.update();
	}

	pub fn draw<'r>(&'r mut self, render_window : &'r mut RenderWindow) -> () {
		self.weapon.set_texture(self.texture_loader.get_texture(self.animation.get_current_texture_id()), false);
		render_window.draw(&self.weapon);
		render_window.draw(&self.gun_shadow);
	}
}