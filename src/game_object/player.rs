use macroquad::color::*;
use macroquad::input::KeyCode;
use macroquad::input::is_key_down;

use crate::geometry::triangle::Triangle;
use crate::geometry::rectangle::Rectangle;
use crate::PLAYERINITX;
use crate::PLAYERINITY;
use crate::PLAYERSCALEX;
use crate::PLAYERSCALEY;
use crate::PLAYER_MIN_X;
use crate::PLAYER_MAX_X;

pub struct Player {
    body: Triangle,
    bounds: Rectangle,
    pub x: f32,
    pub y: f32,
    pub vel_y: f32,
    pub move_speed: f32,
    pub grav: f32,
    pub shield: u32,
    pub shieldleft: u32,
    pub missile: u32,
    pub missileleft: u32,
    pub money: f32,
    pub invincible: bool,
}
impl Player {
    pub fn new(grav: f32, shield: u32, missile: u32, move_speed: f32, invincible:bool) -> Player {
        Player {
            body: Triangle,
            bounds: Rectangle::new(PLAYERINITX, PLAYERINITY, 14.0, 14.0),
            x: 0.0,
            y: 0.0,
            vel_y: 0.0,
            grav,shield,missile,move_speed,invincible,
            missileleft:missile,
            shieldleft:shield,
            money: 0.00,
        }
    }
    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.vel_y = 0.0;
        self.shieldleft = self.shield;
        self.missileleft = self.missile;
        self.bounds.x = PLAYERINITX;
        self.bounds.y = PLAYERINITY;
    }
    pub fn draw(&self, x: f32, y:f32, color:Color, angle_deg:f32) {
        self.body.draw(self.x+PLAYERINITX+x, y+PLAYERINITY, 1.0, color, angle_deg + 90.0, 8.0);
    }
    pub fn update(&mut self) {
        self.vel_y += self.grav;
        self.y -= self.vel_y;
        if (is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) || is_key_down(KeyCode::J) || is_key_down(KeyCode::Kp4)) && self.x - self.move_speed > PLAYER_MIN_X {
            self.x -= self.move_speed;
        } else if (is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) || is_key_down(KeyCode::L) || is_key_down(KeyCode::Kp6)) && self.x + self.move_speed <= PLAYER_MAX_X {
            self.x += self.move_speed;
        }

        self.bounds.x = PLAYERINITX - PLAYERSCALEX * 0.866 + self.x;
        self.bounds.y = PLAYERINITY - PLAYERSCALEY * 0.5 - self.y;
    }
    pub fn check_collision(&mut self, rect: &Rectangle) -> u8 {
        let res = self.bounds.check_collision(rect);
        if !res {
            if -self.y > rect.y + rect.h {
                return 2;
            }
            return 0;
        }
        if self.shieldleft <= 0 {
            if !self.invincible {
                self.grav = 0.0;
                self.vel_y = 0.0;
                return 1;
            } else {
                return 2;
            }
        }
        self.shieldleft -= 1;
        return 3;
    }
}