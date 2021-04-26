use macroquad::color::*;

use crate::geometry::sprite::Sprite;
use crate::geometry::rectangle::Rectangle;
use crate::MISSILE_SPEED;

pub struct Missile {
    pub bounds: Rectangle,
    img: Sprite,
    vel_y: f32,
    grav: f32,
}
impl Missile {
    pub fn new(x:f32, y:f32, vel_y:f32, grav:f32) -> Missile {
        Missile {
            bounds: Rectangle::new_centered(x, y, 16.0, 16.0),
            img: Sprite::new_centered(x, y, "res/missile_projectile.png"),
            vel_y, grav,
        }
    }
    
    pub fn draw(&self, x: f32, y:f32, color:Color) {
        self.img.draw(x, y, color);
    }
    pub fn update(&mut self) {
        self.vel_y += self.grav;
        self.img.y += self.vel_y + MISSILE_SPEED;

        self.bounds.y = self.img.y;
    }
    pub fn check_collision(&mut self, rect: &Rectangle) -> bool {
        return self.bounds.check_collision(rect);
    }
}