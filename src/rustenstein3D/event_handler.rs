
use rsfml::graphics::RenderWindow;
use rsfml::window::{event, keyboard};
use rsfml::window::keyboard::Key;
use rsfml::window::mouse::*;

pub struct EventHandler {
    pub events : ~[event::Event]
}

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler {
            events : ~[]
        }
    }

    pub fn is_key_pressed(&self, key : Key) -> bool {
        keyboard::is_key_pressed(key)
    }

    pub fn has_closed_event(&self) -> bool {
        for ev in self.events.iter() {
            match *ev {
                event::Closed   => return true,
                _               => {}
            }
        }
        false
    }

    pub fn has_gained_focus_event(&self) -> bool {
        for ev in self.events.iter() {
            match *ev {
                event::GainedFocus  => return true,
                _                   => {}
            }
        }
        false
    }

    pub fn has_lost_focus_event(&self) -> bool {
        for ev in self.events.iter() {
            match *ev {
                event::LostFocus    => return true,
                _                   => {}
            }
        }
        false
    }

    pub fn has_text_entered(&self) -> Option<char> {
        for ev in self.events.iter() {
            match *ev {
                event::TextEntered { code }     => return Some(code),
                _                               => {}
            }
        }
        None
    }

    pub fn has_key_pressed_event(&self, key : Key) -> Option<(Key, bool, bool, bool, bool)> {
     for ev in self.events.iter()  {
         match *ev {
             event::KeyPressed { code, alt, ctrl, shift, system }   =>  {
                 if code as int == key as int {
                     return Some((code, alt, ctrl, shift, system))
                 }
             },
             _                                                      => {}
         }
     }
     None
    }

    pub fn has_key_released_event(&self, key : Key) -> Option<(Key, bool, bool, bool, bool)> {
     for ev in self.events.iter() {
         match *ev {
             event::KeyReleased { code, alt, ctrl, shift, system }  =>  {
                 if code as int == key as int {
                     return Some((code, alt, ctrl, shift, system))
                 }
             },
             _                                                      => {}
         }
     }
     None
    }

    pub fn has_mouse_wheel_moved_event(&self) -> Option<(int, int, int)> {
     for ev in self.events.iter() {
         match *ev {
             event::MouseWheelMoved { delta, x, y }     => return Some((delta, x, y)),
             _                                          => {} 
         }
     }
     None
    }

    pub fn has_mouse_button_pressed_event(&self, mouse_button : MouseButton) -> Option<(MouseButton, int, int)> {
     for ev in self.events.iter() {
         match *ev {
             event::MouseButtonPressed { button, x, y }     =>  {
                 if mouse_button as int == button as int {
                     return Some((button, x, y))
                 }
             },
             _                                              => {}
         }
     }
     None
    }

    pub fn has_mouse_button_released_event(&self, mouse_button : MouseButton) -> Option<(MouseButton, int, int)> {
     for ev in self.events.iter() {
         match *ev {
             event::MouseButtonReleased { button, x, y }    =>  {
                 if mouse_button as int == button as int {
                     return Some((button, x, y))
                 }
             },
             _                                              => {}
         }
     }
     None
    }

    pub fn has_mouse_moved_event(&self) -> Option<(int, int)> {
     for ev in self.events.iter() {
         match *ev {
             event::MouseMoved { x, y }     => return Some((x, y)),
             _                           => {} 
         }
     }
     None
    }

    pub fn has_mouse_entered_event(&self) -> bool {
        for ev in self.events.iter() {
            match *ev {
                event::MouseEntered     => return true,
                _                       => {} 
            }
        }
        false
    }

    pub fn has_mouse_left_event(&self) -> bool {
        for ev in self.events.iter() {
            match *ev {
                event::MouseLeft    => return true,
                _                   => {} 
            }
        }
        false
    }

    // pub fn get_mouse_position(&self) -> Vector2i {
    //     self.render_window.get_mouse_position()
    // }

    pub fn get_events(&self) -> ~[event::Event] {
        let mut r_events : ~[event::Event] = ~[];
        for ev in self.events.iter() {
            r_events.push(*ev)
        }
        r_events
    }

    pub fn update_events(&mut self, render_window: &mut RenderWindow) -> () {
        self.events.clear();
        let mut ev;
        loop {
            ev = render_window.poll_event();
            match ev {
                event::NoEvent => break,
                _ => self.events.push(ev)
            }   
        }
    }
}

// TODO IMPLEMENT FUNCTION FOR JOYSTICK HANDLE
// JoystickButtonPressed { joystickid : int, button : int },
// JoystickButtonReleased { joystickid : int, button : int },
// JoystickMoved { joystickid : uint, axis : Axis, position : float },
// JoystickConnected { joystickid : uint },
// JoystickDisconnected { joystickid : uint },
