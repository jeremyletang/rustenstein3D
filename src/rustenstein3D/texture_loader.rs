use rsfml::graphics::Texture;

pub struct TextureLoader {
    priv textures : ~[~Texture]
}

impl TextureLoader {
    pub fn new() -> TextureLoader {
        TextureLoader {
            textures : ~[]
        }
    }

    pub fn load_texture(&mut self, texture_path : ~str) -> bool {
        let texture = Texture::new_from_file(texture_path);
        match texture {
            Some(tex)   => { 
                self.textures.push(~tex);
                true
            },
            None        => false
        }
    }

    pub fn get_texture<'r>(&'r self, index : i32) -> &'r Texture {
        &*self.textures[index]
    }  
}