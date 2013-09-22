#[link(name = "rustenstein3D",
       vers = "0.0.1",
       author = "letang.jeremy@gmail.com",
       uuid = "63B20707-95B9-4327-9C7B-F09C53F17F73",
       url = "http://https://github.com/JeremyLetang/Rustenstein")];

#[license = "MIT"];
#[crate_type = "bin"];

extern mod rsfml;

use rsfml::graphics::{RenderWindow, Font, sfClose};
use rsfml::window::{VideoMode, ContextSettings};

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
pub mod game;


#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}

fn load_texture() -> texture_loader::TextureLoader {
    let mut texture_loader = texture_loader::TextureLoader::new();
    if texture_loader.load_texture(~"./resources/ground.tga") == false ||
       texture_loader.load_texture(~"./resources/1.tga") == false ||
       texture_loader.load_texture(~"./resources/2.tga") == false ||
       texture_loader.load_texture(~"./resources/3.tga") == false ||
       texture_loader.load_texture(~"./resources/4.tga") == false ||
       texture_loader.load_texture(~"./resources/5.tga") == false ||
       texture_loader.load_texture(~"./resources/6.tga") == false ||
       texture_loader.load_texture(~"./resources/7.tga") == false ||
       texture_loader.load_texture(~"./resources/8.tga") == false ||
       texture_loader.load_texture(~"./resources/9.tga") == false ||
       texture_loader.load_texture(~"./resources/10.tga") == false ||
       texture_loader.load_texture(~"./resources/sky.tga") == false {
        fail!("Error : Cannot load texture.");
    }
    texture_loader
}

fn main() -> () {
    // Check if a custom width is set.
    let args = os::args();
    let mut width : uint;
    let mut height : uint;
    match args.len() {
        3 => {
            width = from_str::<uint>(args[1]).expect("Error the first argument is not a width!");
            height = from_str::<uint>(args[2]).expect("Error the second argument is not a width!");
        },
        1 => {
            width = 768;
            height = 480;
        },
        _ => fail!("Error incompatible number of arguments!")
    };

    // Create the render_window.
    let settings = ContextSettings::default();
    let video_mode = VideoMode::new_init(width, height, 32);
    // let video_mode = VideoMode::new_init(512, 384, 32);
    let render_window = @mut RenderWindow::new(video_mode, ~"Rustenstein3D", sfClose, &settings).expect("Error : Cannot create a render_window!");
    
    // set the framerate limit to 30 fps.
    render_window.set_framerate_limit(45);

    // Create the font for the FPS_handler.
    let font = @Font::new_from_file(~"./resources/sansation.ttf").expect("Error : Cannot load font, font resources/sansation.ttf doesn.t exist!");

    // Create the texture loader and load textures
    let texture_loader = load_texture();

    // Create the game_loop and activate the fps handler.
    let mut game_loop = game::GameLoop::new(render_window, &texture_loader);
    game_loop.activate_FPS(font);

    game_loop.run();
}