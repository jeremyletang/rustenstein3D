use rsfml::graphics::Texture;

pub struct TextureLoader {
    textures : Vec<Box<Texture>>
}

impl TextureLoader {
    pub fn new() -> TextureLoader {
        TextureLoader {
            textures : Vec::new()
        }
    }

    pub fn load_texture(&mut self, texture_path : StrBuf) -> bool {
        let texture = Texture::new_from_file(texture_path.as_slice());
        match texture {
            Some(tex)   => {
                self.textures.push(box tex);
                true
            },
            None        => false
        }
    }

    pub fn get_texture<'r>(&'r self, index : i32) -> &'r Texture {
        &**(self.textures.get(index as uint))
    }
}