use std::fs::read as fs_read;

use macroquad::texture::Texture2D;
use macroquad::texture::draw_texture;
use macroquad::prelude::ImageFormat;
use macroquad::color::Color;

pub struct Sprite {
    pub x:f32,
    pub y:f32,
    img: Texture2D,
}
impl Sprite {
    //pub fn new(x:f32,y:f32,path:&str) -> Sprite {
    //    let file = match fs_read(path) {
    //        Ok(o) => o,
    //        Err(e) => panic!("Error reading file {}! {}",path,e),
    //    };
    //    Sprite {
    //        x,y,
    //        img: Texture2D::from_file_with_format(&file, Some(ImageFormat::Png)),
    //    }
    //}
    pub fn new_centered(x:f32,y:f32,path:&str) -> Sprite {
        let file = match fs_read(path) {
            Ok(o) => o,
            Err(e) => panic!("Error reading file {}! {}",path,e),
        };
        let img = Texture2D::from_file_with_format(&file, Some(ImageFormat::Png));
        Sprite {
            x:x-img.width()/2.0,
            y:y-img.height()/2.0,
            img,
        }
    }
    pub fn draw(&self, x:f32, y:f32, color:Color) {
        draw_texture(self.img, self.x+x, self.y+y, color);
    }
}