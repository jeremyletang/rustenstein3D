#![crate_name = "rustenstein3D"]
#![license = "MIT"]
#![crate_type = "bin"]

#![feature(globs)]
#![feature(managed_boxes)]
#![allow(non_camel_case_types)]
#![allow(visible_private_types)]
#![allow(dead_code)]

extern crate native;
extern crate rsfml;

use rsfml::graphics::{RenderWindow, Font};
use rsfml::window::{VideoMode, ContextSettings};
use rsfml::system::Vector2i;
use rsfml::window::Close;

use std::os;
use std::from_str::*;

pub mod FPS;
pub mod event_handler;
pub mod game_mode;
pub mod map;
pub mod mini_map;
pub mod raycasting_engine;
pub mod texture_loader;
pub mod hud;
pub mod animation;
pub mod weapon;
pub mod game;


#[cfg(target_os="macos")]
#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn display_help() -> () {
    println!("Arguments availables for rustenstein3D :");
    println!("\t-w [window_width] [window_height] : specify a new size for the window.");
    println!("\t--noground : diseable the ground texturing (improve performance).");
    println!("\t--help : display this help.");
}

fn load_texture() -> texture_loader::TextureLoader {
    let mut texture_loader = texture_loader::TextureLoader::new();
    if texture_loader.load_texture("../resources/ground.tga".to_string()) == false || // 0
       texture_loader.load_texture("../resources/1.tga".to_string()) == false || // 1
       texture_loader.load_texture("../resources/2.tga".to_string()) == false || // 2
       texture_loader.load_texture("../resources/3.tga".to_string()) == false || // 3
       texture_loader.load_texture("../resources/4.tga".to_string()) == false || // 4
       texture_loader.load_texture("../resources/5.tga".to_string()) == false || // 5
       texture_loader.load_texture("../resources/6.tga".to_string()) == false || // 6
       texture_loader.load_texture("../resources/7.tga".to_string()) == false || // 7
       texture_loader.load_texture("../resources/8.tga".to_string()) == false || // 8
       texture_loader.load_texture("../resources/9.tga".to_string()) == false || // 9
       texture_loader.load_texture("../resources/10.tga".to_string()) == false || // 10
       texture_loader.load_texture("../resources/sky.tga".to_string()) == false || // 11
       texture_loader.load_texture("../resources/weapons/gun_1.png".to_string()) == false || // 12
       texture_loader.load_texture("../resources/weapons/gun_2.png".to_string()) == false || // 13
       texture_loader.load_texture("../resources/weapons/gun_3.png".to_string()) == false || // 14
       texture_loader.load_texture("../resources/weapons/gun_4.png".to_string()) == false || // 15
       texture_loader.load_texture("../resources/weapons/gun_5.png".to_string()) == false || // 16
       texture_loader.load_texture("../resources/weapons/gun_6.png".to_string()) == false || // 17
       texture_loader.load_texture("../resources/weapons/gun_shadow.png".to_string()) == false || // 18
       texture_loader.load_texture("../resources/weapons/gun2_1.png".to_string()) == false || // 19
       texture_loader.load_texture("../resources/weapons/gun2_2.png".to_string()) == false || // 20
       texture_loader.load_texture("../resources/weapons/gun2_3.png".to_string()) == false || // 21
       texture_loader.load_texture("../resources/weapons/gun2_4.png".to_string()) == false || // 22
       texture_loader.load_texture("../resources/weapons/gun2_5.png".to_string()) == false || // 23
       texture_loader.load_texture("../resources/weapons/gun2_6.png".to_string()) == false || // 24
       texture_loader.load_texture("../resources/weapons/gun2_shadow.png".to_string()) == false || // 25
       texture_loader.load_texture("../resources/weapons/gun3_1.png".to_string()) == false || // 26
       texture_loader.load_texture("../resources/weapons/gun3_2.png".to_string()) == false || // 27
       texture_loader.load_texture("../resources/weapons/gun3_3.png".to_string()) == false || // 28
       texture_loader.load_texture("../resources/weapons/gun3_4.png".to_string()) == false || // 29
       texture_loader.load_texture("../resources/weapons/gun3_5.png".to_string()) == false || // 30
       texture_loader.load_texture("../resources/weapons/gun3_6.png".to_string()) == false || // 31
       texture_loader.load_texture("../resources/weapons/gun3_shadow.png".to_string()) == false || // 32
       texture_loader.load_texture("../resources/weapons/cut_1.png".to_string()) == false || // 33
       texture_loader.load_texture("../resources/weapons/cut_2.png".to_string()) == false || // 34
       texture_loader.load_texture("../resources/weapons/cut_3.png".to_string()) == false || // 35
       texture_loader.load_texture("../resources/weapons/cut_4.png".to_string()) == false || // 36
       texture_loader.load_texture("../resources/weapons/cut_5.png".to_string()) == false || // 37
       texture_loader.load_texture("../resources/weapons/cut_6.png".to_string()) == false || //38
       texture_loader.load_texture("../resources/weapons/cut_shadow.png".to_string()) == false || // 39
       texture_loader.load_texture("../resources/face1.png".to_string()) == false || //40
       texture_loader.load_texture("../resources/face2.png".to_string()) == false || //41
       texture_loader.load_texture("../resources/face3.png".to_string()) == false { //42
        fail!("Error : Cannot load texture.");
    }
    texture_loader
}

fn main() -> () {
    // Check if a custom width is set.
    let args = os::args();
    let mut width : uint = 768;
    let mut height : uint = 480;
    let mut noground : bool = false;
    let mut i_args = 1;

    while i_args < args.len() {
        match args.get(i_args).as_slice() {
            "--help"       => { display_help(); return; },
            "--noground"   => noground = true,
            "-w"           => {
                if i_args + 2 >= args.len() { fail!("Error missing arguments for -w option."); }
                width = from_str::<uint>(args.get(i_args + 1).as_slice()).expect("Error the first parameter after -w argument is not a width!");
                height = from_str::<uint>(args.get(i_args + 2).as_slice()).expect("Error the second parameter after -w argument is not a width!");
                i_args += 2;
            },
            _              => fail!("Error unknown argument."),
        }
        i_args += 1;
    }

    // Create the render_window.
    let settings = ContextSettings::default();
    let video_mode = VideoMode::new_init(width, height, 32);
    // let video_mode = VideoMode::new_init(512, 384, 32);
    let mut render_window = RenderWindow::new(video_mode,
                                              "Rustenstein3D",
                                              Close,
                                              &settings).expect("Error : Cannot create a render_window!");

    // set the framerate limit to 30 fps.
    render_window.set_framerate_limit(40);

    // hide the cursor.
    render_window.set_mouse_cursor_visible(false);

    // set the mouse positon on the center of the window
    render_window.set_mouse_position(&Vector2i {x : width as i32 / 2,
                                                y : height as i32 / 2});

    // Create the font for the FPS_handler.
    let font = Font::new_from_file("../resources/sansation.ttf")
        .expect("Error : Cannot load font, font resources/sansation.ttf doesn.t exist!");

    // Create the texture loader and load textures
    let texture_loader = load_texture();

    // Create the game_loop and activate the fps handler.
    let mut game_loop = game::GameLoop::new(render_window,
                                            &texture_loader,
                                            noground);
    game_loop.activate_FPS(&font);

    game_loop.run();
}