/*!
* Basic class to display the current game frames per seconds
*
* This class display the current fps in the left bottom of the window.
*
*/

use rsfml::graphics::{RenderWindow, Font, Text, Color};
use rsfml::system::{Clock, Vector2f};

/// Definition of class FPSHandler
pub struct FPSHandler<'s> {
    fps_clock : Clock,
    text : Text<'s>,
    images : uint
}

/// Implementation of class FPSHandler
impl<'s> FPSHandler<'s> {
    /**
    * Constructor of class FPSHandler
    *
    * # Arguments
    * * render_window - The RenderWindow to calculate the FPS
    * * font - The font to render the text on the window.
    *
    * Return a new instance of FPSHandler
    */
    pub fn new(font : &'s Font) -> FPSHandler<'s> {
        let mut t = Text::new().unwrap();
        t.set_font(font);
        t.set_character_size(20);
        t.set_position(&(Vector2f::new(10., 10.)));
        t.set_color(&Color::white());
        t.set_string("0");
        FPSHandler {
            fps_clock : Clock::new(),
            text : t,
            images : 0
        }
    }

    /**
    * Update internal data of the FPSHandler
    *
    * Call this function at each end of the loop to update
    * FPSHandler internal data.
    */
    pub fn update(&mut self) -> () {
        if self.fps_clock.get_elapsed_time().as_seconds() >= 0.33 {
            self.text.set_string((self.images * 3).to_string().as_slice());
            self.images = 0;
            self.fps_clock.restart();
        }
        self.images += 1;
    }

    /**
    * Draw the current FPS on the left bottom of the window
    */
    pub fn draw(&self, render_window: &mut RenderWindow) -> () {
        render_window.draw(&self.text)
    }

}