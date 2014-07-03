use rsfml::graphics::{RenderWindow, RectangleShape};
use rsfml::window::{keyboard, mouse};
use rsfml::system::Vector2f;

use animation::*;
use event_handler::EventHandler;
use texture_loader::TextureLoader;

pub struct Weapon<'s> {
    weapons : RectangleShape<'s>,
    animations : Vec<Animation>,
    texture_loader : &'s TextureLoader,
    shadows : RectangleShape<'s>,
    shadows_id : Vec<i32>,
    current_weapon : i32,
    mouse_fire : bool
}

impl<'s> Weapon<'s> {
    pub fn new(window_size : &Vector2f, texture_loader : &'s TextureLoader) -> Weapon<'s> {
        Weapon {
            weapons : Weapon::initialize_weapons(window_size),
            animations : Weapon::initialize_animation(),
            texture_loader : texture_loader,
            shadows : Weapon::initialize_shadows(window_size),
            shadows_id : vec![18, 25, 32, 39],
            current_weapon : 0,
            mouse_fire : false
        }
    }

    fn initialize_weapons(window_size : &Vector2f) -> RectangleShape<'s> {
        let mut tmp_weapon = RectangleShape::new_init(&Vector2f {x : 400., y : 400.}).unwrap();
        tmp_weapon.set_position2f(window_size.x / 2. - 200., window_size.y - 400. - 81.);
        tmp_weapon
    }

    fn initialize_shadows(window_size : &Vector2f) -> RectangleShape<'s> {
        let mut tmp_shadow = RectangleShape::new_init(&Vector2f {x : 99., y : 48.}).unwrap();
        tmp_shadow.set_position2f(window_size.x - 115., window_size.y - 66.);
        tmp_shadow
    }

    fn initialize_animation() -> Vec<Animation> {
        let mut animations = Vec::new();
        animations.push(Animation::new(vec![12, 13, 14, 15, 16, 17], Stop, PlayOnce, 0.07, 3));
        animations.push(Animation::new(vec![19, 20, 21, 22, 23, 24], Stop, PlayOnce, 0.07, 3));
        animations.push(Animation::new(vec![26, 27, 28, 29, 30, 31], Stop, PlayOnce, 0.07, 3));
        animations.push(Animation::new(vec![33, 34, 35, 36, 37, 38], Stop, PlayOnce, 0.07, 3));

        animations
    }

    pub fn update<'r>(&'r mut self, event_handler : &'r EventHandler) -> () {
        match event_handler.has_key_pressed_event(keyboard::Num1) {
            Some(_) => self.current_weapon = 0,
            None    => {}
        };
        match event_handler.has_key_pressed_event(keyboard::Num2) {
            Some(_) => self.current_weapon = 1,
            None    => {}
        };
        match event_handler.has_key_pressed_event(keyboard::Num3) {
            Some(_) => self.current_weapon = 2,
            None    => {}
        };
        match event_handler.has_key_pressed_event(keyboard::Num4) {
            Some(_) => self.current_weapon = 3,
            None    => {}
        };

        if self.mouse_fire == false {
            match event_handler.has_mouse_button_pressed_event(mouse::MouseLeft) {
                Some(_) => { self.animations.get_mut(self.current_weapon as uint).set_state(Play) ; self.mouse_fire = true },
                None    => {}
            };
        } else {
            match event_handler.has_mouse_button_released_event(mouse::MouseLeft) {
                Some(_) => { self.mouse_fire = false },
                None    => self.animations.get_mut(self.current_weapon as uint).set_state(Play)
            };
        }

        if event_handler.is_key_pressed(keyboard::E) {
            self.animations.get_mut(self.current_weapon as uint).set_state(Play);
        }
        self.animations.get_mut(self.current_weapon as uint).update();
    }

    pub fn draw<'r>(&'r mut self, render_window : &'r mut RenderWindow) -> () {
        self.weapons.set_texture(self.texture_loader.get_texture(self.animations.get_mut(self.current_weapon as uint).get_current_texture_id()), false);
        self.shadows.set_texture(self.texture_loader.get_texture(*self.shadows_id.get(self.current_weapon as uint)), false);
        render_window.draw(&self.weapons);
        render_window.draw(&self.shadows);
    }
}