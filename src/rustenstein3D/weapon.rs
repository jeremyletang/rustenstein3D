use rsfml::graphics::{RenderWindow, RectangleShape};
use rsfml::window::{keyboard, mouse};
use rsfml::system::Vector2f;

use animation::*;
use event_handler::EventHandler;
use texture_loader::TextureLoader;

pub struct Weapon<'self> {
	priv weapons : RectangleShape<'self>,
	priv animations : ~[Animation],
	priv texture_loader : &'self TextureLoader,
	priv shadows : RectangleShape<'self>,
	priv shadows_id : ~[i32],
	priv current_weapon : i32
}

impl<'self> Weapon<'self> {
	pub fn new(window_size : &Vector2f, texture_loader : &'self TextureLoader) -> Weapon<'self> {
		Weapon {
			weapons : Weapon::initialize_weapons(window_size),
			animations : Weapon::initialize_animation(),
			texture_loader : texture_loader,
			shadows : Weapon::initialize_shadows(window_size),
			shadows_id : ~[18, 25, 32, 39],
			current_weapon : 0
		}
	}

	fn initialize_weapons(window_size : &Vector2f) -> RectangleShape<'self> {
		let mut tmp_weapon = RectangleShape::new_init(&Vector2f {x : 400., y : 400.}).unwrap();
		tmp_weapon.set_position2f(window_size.x / 2. - 200., window_size.y - 400. - 81.);
		tmp_weapon
	}

	fn initialize_shadows(window_size : &Vector2f) -> RectangleShape<'self> {
		let mut tmp_shadow = RectangleShape::new_init(&Vector2f {x : 99., y : 48.}).unwrap();
		tmp_shadow.set_position2f(window_size.x - 115., window_size.y - 66.);
		tmp_shadow
	}

	fn initialize_animation() -> ~[Animation] {
		let mut animations = ~[];
		animations.push(Animation::new(~[12, 13, 14, 15, 16, 17], Stop, PlayOnce, 0.05, 3));
		animations.push(Animation::new(~[19, 20, 21, 22, 23, 24], Stop, PlayOnce, 0.05, 3));
		animations.push(Animation::new(~[26, 27, 28, 29, 30, 31], Stop, PlayOnce, 0.05, 3));
		animations.push(Animation::new(~[33, 34, 35, 36, 37, 38], Stop, PlayOnce, 0.05, 3));
		
		animations
	}

	pub fn update<'r>(&'r mut self, event_handler : &'r EventHandler) -> () {
		match event_handler.has_key_pressed_event(keyboard::Num1) {
			Some(_) => self.current_weapon = 0,
			None	=> {}
		};
		match event_handler.has_key_pressed_event(keyboard::Num2) {
			Some(_) => self.current_weapon = 1,
			None	=> {}
		};
		match event_handler.has_key_pressed_event(keyboard::Num3) {
			Some(_) => self.current_weapon = 2,
			None	=> {}
		};
		match event_handler.has_key_pressed_event(keyboard::Num4) {
			Some(_) => self.current_weapon = 3,
			None	=> {}
		};

		match event_handler.has_mouse_button_pressed_event(mouse::MouseLeft) {
			Some(_) => self.animations[self.current_weapon].set_state(Play),
			None	=> {}
		};
		match event_handler.has_key_pressed_event(keyboard::E) {
		Some(_) => self.animations[self.current_weapon].set_state(Play),
		None	=> {}
		};
		self.animations[self.current_weapon].update();
	}

	pub fn draw<'r>(&'r mut self, render_window : &'r mut RenderWindow) -> () {
		self.weapons.set_texture(self.texture_loader.get_texture(self.animations[self.current_weapon].get_current_texture_id()), false);
		self.shadows.set_texture(self.texture_loader.get_texture(self.shadows_id[self.current_weapon]), false);
		render_window.draw(&self.weapons);
		render_window.draw(&self.shadows);
	}
}